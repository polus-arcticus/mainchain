use crate::keystore::Keystore;
use crate::overview::LocalchainOverview;
use crate::{
  overview, AccountStore, ChannelHoldCloseOptions, CryptoScheme, DomainStore, Localchain,
  LocalchainConfig, MainchainClient,
};
use anyhow::anyhow;
use argon_primitives::argon_utils::format_argons;
use argon_primitives::Domain;
use clap::{Args, Parser, Subcommand, ValueHint};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use std::ffi::OsString;
use std::fmt::Debug;
use std::path::Path;
use std::{env, fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version = env!("IMPL_VERSION"), about, arg_required_else_help = true, long_about = None)]
struct Cli {
  /// Where is your localchain? Defaults to a project-specific directory based on OS.
  ///    Linux:   /home/alice/.config/argon/localchain
  ///    Windows: C:\Users\Alice\AppData\Roaming\argon\localchain
  ///    macOS:   /Users/Alice/Library/Application Support/argon/localchain
  #[clap(short, long, env = "ARGON_LOCALCHAIN_BASE_PATH", global=true, value_hint = ValueHint::DirPath, verbatim_doc_comment)]
  base_dir: Option<PathBuf>,
  /// The localchain name you'd like to use
  #[clap(
    short,
    default_value = "primary",
    long,
    global = true,
    env = "ARGON_LOCALCHAIN_NAME"
  )]
  name: String,

  /// The mainchain to connect to (this is how a notary url is looked up)
  #[clap(
    short,
    long,
    env = "ARGON_MAINCHAIN_URL",
    default_value = "ws://127.0.0.1:9944",
    global = true
  )]
  mainchain_url: String,

  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Sync the localchain proofs with the latest notebooks. This will also submit votes and close/claim channel_holds as needed.
  Sync {
    /// What address should be used for votes (only relevant if claiming channel_holds)
    #[clap(long, value_name = "SS58_ADDRESS")]
    vote_address: Option<String>,

    /// Set a minimum amount of tax to wait for before submitting votes (does not ignore blockchain minimum
    #[clap(long)]
    minimum_vote_amount: Option<u128>,

    /// Password to unlock the embedded keystore
    #[clap(flatten)]
    keystore_password: EmbeddedKeyPassword,
  },

  /// Explore and manage domains
  Domains {
    #[clap(subcommand)]
    subcommand: DomainsSubcommand,
  },

  /// Manage local accounts
  Accounts {
    #[clap(subcommand)]
    subcommand: AccountsSubcommand,
  },

  /// Create and receive transactions
  Transactions {
    #[clap(subcommand)]
    subcommand: TransactionsSubcommand,
  },
}

#[derive(Subcommand, Debug)]
enum DomainsSubcommand {
  /// List all locally registered domains
  List,
  /// Generate the hash for a domain
  Hash {
    /// The domain name
    #[clap()]
    domain: String,
  },
  /// Check if a domain is registered
  Check {
    /// The domain name
    #[clap()]
    domain: String,
  },
  /// Lease a domain
  Lease {
    /// The domain name
    #[clap()]
    domain: String,

    /// Password to unlock the embedded keystore
    #[clap(flatten)]
    keystore_password: EmbeddedKeyPassword,

    /// Which account should the registration be assigned to. This is the account you'll manage the domain with on the mainchain.
    #[arg(short, long, value_name = "SS58_ADDRESS", required = true)]
    owner_address: String,
  },
}

#[derive(Subcommand, Debug)]
enum AccountsSubcommand {
  /// List all localchains you have access to
  List,
  /// Create a new localchain
  Create {
    /// The secret key URI.
    /// If the value is a file, the file content is used as URI.
    /// If not given, a key will be autogenerated
    #[clap(long)]
    suri: Option<String>,

    /// Add a password for this key (can also be embedded in the suri by trailing the suri with ///password)
    /// You will use this to unlock your keystore signing
    #[clap(flatten)]
    keystore_password: EmbeddedKeyPassword,

    /// The crypto scheme to use for the key
    #[clap(long, default_value = "sr25519")]
    scheme: CryptoScheme,
  },

  /// Get the current account information
  Info {
    /// Should we sync the latest changes before showing the account info
    #[clap(long)]
    sync_latest: bool,
    #[clap(flatten)]
    keystore_password: EmbeddedKeyPassword,
  },
}

#[derive(Subcommand, Debug)]
enum TransactionsSubcommand {
  /// Create an argon file to send funds to another account
  Send {
    #[clap(flatten)]
    send_argon_file: SendArgonFileArgs,

    /// The path to save the argon file to. Defaults to a file in the OS tmp directory.
    #[clap(long, value_hint = ValueHint::DirPath)]
    save_to_path: Option<String>,

    /// The password to unlock the keystore
    #[clap(flatten)]
    keystore_password: EmbeddedKeyPassword,
  },
  /// Receive an argon file from another account
  Receive {
    #[clap(flatten)]
    receive_argon_file: ReceiveArgonFileArgs,

    /// The password to unlock the keystore
    #[clap(flatten)]
    keystore_password: EmbeddedKeyPassword,
  },
  /// Transfer funds from the corresponding account on the mainchain to this localchain
  FromMainchain {
    #[clap(flatten)]
    transfer_args: TransferArgs,

    /// The password to unlock the keystore
    #[clap(flatten)]
    keystore_password: EmbeddedKeyPassword,
  },
  /// Transfer funds from to the corresponding account on the mainchain
  ToMainchain {
    #[clap(flatten)]
    transfer_args: TransferArgs,

    /// Wait for this transaction to be in a notebook recorded to the mainchain
    #[clap(long, default_value_t = false)]
    wait_for_immortalized: bool,

    /// The password to unlock the keystore
    #[clap(flatten)]
    keystore_password: EmbeddedKeyPassword,
  },
}

#[derive(Debug, Args)]
struct ReceiveArgonFileArgs {
  /// The argon file text or path
  argon_file: String,
}

#[derive(Debug, Args)]
struct SendArgonFileArgs {
  /// The number of argons to send
  argons: f32,

  /// The account to send to. If omitted, this should be treated like cash sent in the mail (can be stolen).
  #[clap(value_name = "SS58_ADDRESS")]
  to: Option<String>,
}

#[derive(Debug, Args)]
struct TransferArgs {
  /// The number of argons to transfer
  argons: f32,
}

#[cfg(feature = "napi")]
#[napi(js_name = "runCli")]
pub async fn run_js() -> napi::Result<()> {
  let _ = tracing_subscriber::FmtSubscriber::builder()
    .with_env_filter(tracing_subscriber::EnvFilter::from_env("DEBUG"))
    .try_init();

  let inner_args = {
    let mut args = env::args_os();
    // lop off the first nodejs arg
    let _ = args.next();
    args.collect::<Vec<OsString>>()
  };
  let result = run(inner_args).await;
  result?;
  Ok(())
}

pub async fn run<I, T>(itr: I) -> anyhow::Result<()>
where
  I: IntoIterator<Item = T>,
  T: Into<OsString> + Clone,
{
  Localchain::config_logs();
  let cli = Cli::parse_from(itr);

  let base_dir = cli.base_dir.clone();
  let name = cli.name.clone();
  let mainchain_url = cli.mainchain_url.clone();
  let path = get_path(base_dir.clone(), name.clone());

  match cli.command {
    Commands::Sync {
      vote_address,
      keystore_password,
      minimum_vote_amount,
    } => {
      let localchain = Localchain::load(LocalchainConfig {
        path,
        mainchain_url,
        ntp_pool_url: None,
        keystore_password: Some(keystore_password),
      })
      .await?;

      let balance_sync = localchain.balance_sync();
      let sync_options = vote_address.map(|vote_address| ChannelHoldCloseOptions {
        votes_address: Some(vote_address),
        minimum_vote_amount: minimum_vote_amount.map(|v| v as i64),
      });

      let sync = balance_sync.sync(sync_options.clone()).await?;
      println!(
        "Synced {:?} balance changes. ChannelHolds updated: {:?}",
        sync.balance_changes().len(),
        sync.channel_hold_notarizations().len()
      );
    }
    Commands::Domains { subcommand } => match subcommand {
      DomainsSubcommand::List => {
        let db = Localchain::create_db(path).await?;
        let domains = DomainStore::new(db);
        let domains = domains.list().await?;

        let mut table = Table::new();

        table
          .load_preset(UTF8_FULL)
          .apply_modifier(UTF8_ROUND_CORNERS)
          .set_content_arrangement(ContentArrangement::Dynamic)
          .set_header(vec![
            "Top Level",
            "Second Level",
            "Owner",
            "Registration Tick",
            "Hash",
          ]);
        for domain in domains {
          table.add_row(vec![
            domain.top_level.clone(),
            domain.name.clone(),
            domain.registered_to_address,
            domain.registered_at_tick.to_string(),
            Domain::from_string(
              domain.name,
              DomainStore::tld_from_string(domain.top_level)
                .expect("Should be able to translate a top_level"),
            )
            .hash()
            .to_string(),
          ]);
        }
        println!("{table}");
      }
      DomainsSubcommand::Hash { domain } => {
        let domain = Domain::parse(domain).map_err(|_| anyhow!("Not a valid domain"))?;
        println!("Hash: {:?}", domain.hash());
      }
      DomainsSubcommand::Check { domain } => {
        let argon_domain =
          Domain::parse(domain.clone()).map_err(|_| anyhow!("Not a valid domain"))?;
        let mainchain = MainchainClient::connect(mainchain_url, 5_000).await?;
        let registration = mainchain
          .get_domain_registration(
            argon_domain.name.clone().to_string(),
            argon_domain.top_level,
          )
          .await?;
        let mut table = Table::new();
        table
          .load_preset(UTF8_FULL)
          .apply_modifier(UTF8_ROUND_CORNERS)
          .set_content_arrangement(ContentArrangement::Dynamic)
          .set_header(vec!["Domain", "Registered?", "Hash"]);
        table.add_row(vec![
          Cell::new(&domain),
          Cell::new(match registration.is_some() {
            true => "Yes",
            false => "No",
          })
          .set_alignment(CellAlignment::Center),
          Cell::new(hex::encode(argon_domain.hash().0)).set_alignment(CellAlignment::Center),
        ]);
        println!("{table}");
      }
      DomainsSubcommand::Lease {
        keystore_password,
        domain,
        owner_address,
      } => {
        let argon_domain =
          Domain::parse(domain.clone()).map_err(|_| anyhow!("Not a valid domain"))?;
        let localchain = Localchain::load(LocalchainConfig {
          path,
          mainchain_url,
          ntp_pool_url: None,
          keystore_password: Some(keystore_password),
        })
        .await?;

        let change = localchain.begin_change();
        change.lease_domain(domain.clone(), owner_address).await?;
        change.sign().await?;
        let tracker = change.notarize().await?;
        let mut table = Table::new();

        table
          .load_preset(UTF8_FULL)
          .apply_modifier(UTF8_ROUND_CORNERS)
          .set_content_arrangement(ContentArrangement::Dynamic)
          .set_header(vec!["Change #", "Balance", "Status"]);
        for (_account, balance_change) in tracker.get_changed_accounts().await {
          table.add_row(vec![
            balance_change.change_number.to_string(),
            balance_change.balance,
            format!("{:?}", balance_change.status),
          ]);
        }
        println!("{} registered at tick {} in notebook {}. Domain hash={:#?} (use this hash for zone record registration on mainchain).\
          \n\nChanged Accounts:\n{table}",
                         domain, tracker.tick, tracker.notebook_number, argon_domain.hash());
      }
    },

    Commands::Accounts { subcommand } => match subcommand {
      AccountsSubcommand::Info {
        sync_latest,
        keystore_password,
      } => {
        if !Path::new(&path).exists() {
          return Err(anyhow!("Localchain does not exist at {:?}", path));
        }
        let account_overview: LocalchainOverview = if sync_latest {
          let localchain = Localchain::load(LocalchainConfig {
            path: path.clone(),
            mainchain_url,
            ntp_pool_url: None,
            keystore_password: Some(keystore_password),
          })
          .await?;
          localchain.balance_sync().sync(None).await?;
          localchain.account_overview().await?
        } else {
          let db = Localchain::create_db(path.clone()).await?;

          overview::OverviewStore::new(db, name, Default::default())
            .get()
            .await?
        };

        let mut table = Table::new();
        table
          .load_preset(UTF8_FULL)
          .apply_modifier(UTF8_ROUND_CORNERS)
          .set_content_arrangement(ContentArrangement::Dynamic)
          .set_header(vec!["Address", "Balance", "Tax"]);

        table.add_row(vec![
          account_overview.address.clone(),
          account_overview.balance_with_pending(),
          account_overview.tax_with_pending(),
        ]);

        println!("Account at {path}:\n{table}");
      }
      AccountsSubcommand::Create {
        scheme,
        suri,
        keystore_password,
      } => {
        if fs::metadata(&path).is_ok() {
          return Err(anyhow!("Localchain already exists at {:?}", path));
        }

        let db = Localchain::create_db(path.clone()).await?;
        let keystore = Keystore::new(db.clone());
        if let Some(suri) = suri {
          keystore
            .import_suri(suri, scheme, Some(keystore_password))
            .await?;
        } else {
          keystore
            .bootstrap(Some(scheme), Some(keystore_password))
            .await?;
        }

        let mut table = Table::new();

        let mut conn = db.acquire().await?;
        table
          .load_preset(UTF8_FULL)
          .apply_modifier(UTF8_ROUND_CORNERS)
          .set_content_arrangement(ContentArrangement::Dynamic)
          .set_header(vec!["Address", "Path", "NotaryId"]);
        let account = AccountStore::db_deposit_account(&mut conn, None).await?;
        table.add_row(vec![account.address, path, account.notary_id.to_string()]);

        println!("Account created at:\n{table}");
      }

      AccountsSubcommand::List => {
        let dir = base_dir.unwrap_or(PathBuf::from(Localchain::get_default_dir()));

        let mut table = Table::new();

        table
          .load_preset(UTF8_FULL)
          .apply_modifier(UTF8_ROUND_CORNERS)
          .set_content_arrangement(ContentArrangement::Dynamic)
          .set_header(account_columns());

        for entry in fs::read_dir(dir.clone())? {
          let Some(entry) = entry.ok() else {
            continue;
          };
          let Some(file_type) = entry.file_type().ok() else {
            continue;
          };
          if !file_type.is_file() {
            continue;
          }
          let Some(name) = entry.file_name().into_string().ok() else {
            continue;
          };
          if name.ends_with(".db") {
            let path = get_path(Some(dir.clone()), name.clone());
            let db = Localchain::create_db(path).await?;
            let account_overview: LocalchainOverview =
              overview::OverviewStore::new(db, name, Default::default())
                .get()
                .await?;
            table.add_row(format_account_record(
              &account_overview.name,
              account_overview.address.clone(),
              account_overview.balance_with_pending(),
              account_overview.tax_with_pending(),
            ));
          }
        }

        println!("{table}");
      }
    },
    Commands::Transactions { subcommand } => match subcommand {
      TransactionsSubcommand::Send {
        send_argon_file: SendArgonFileArgs { argons, to },
        save_to_path,
        keystore_password,
      } => {
        let localchain = Localchain::load(LocalchainConfig {
          path,
          mainchain_url,
          ntp_pool_url: None,
          keystore_password: Some(keystore_password),
        })
        .await?;
        let transactions = localchain.transactions();
        let microgons = (argons * 1_000_000.0) as u128;

        let result = transactions.send(microgons, to.map(|a| vec![a])).await?;
        let filename = save_to_path.unwrap_or_else(|| {
          let mut path = env::temp_dir();
          let argons = format_argons(microgons);
          path.push(format!("Send {}.argon", argons));
          path
            .to_str()
            .expect("Path should convert to a string")
            .to_string()
        });
        fs::write(&filename, result)?;

        println!("Argon file saved to: {:?}", filename);
      }
      TransactionsSubcommand::Receive {
        receive_argon_file: ReceiveArgonFileArgs { argon_file },
        keystore_password,
      } => {
        let localchain = Localchain::load(LocalchainConfig {
          path,
          mainchain_url,
          ntp_pool_url: None,
          keystore_password: Some(keystore_password),
        })
        .await?;
        let transactions = localchain.transactions();
        // if argon file is a path, read it
        let argon_json = if argon_file.starts_with('{') {
          argon_file
        } else {
          fs::read_to_string(argon_file)?
        };
        let result = transactions.import_argons(argon_json).await?;
        let added = result.imports[0].net_balance_change();
        println!("Imported {} argons", format_argons(added.unsigned_abs()));
      }
      TransactionsSubcommand::FromMainchain {
        transfer_args: TransferArgs { argons },
        keystore_password,
      } => {
        let microgons = (argons * 1_000_000.0) as u128;
        let localchain = Localchain::load(LocalchainConfig {
          path,
          mainchain_url,
          ntp_pool_url: None,
          keystore_password: Some(keystore_password),
        })
        .await?;
        let mainchain_transfers = localchain.mainchain_transfers();
        let transfer = mainchain_transfers
          .send_to_localchain(microgons, None)
          .await?;
        localchain.balance_sync().sync(None).await?;
        let details = mainchain_transfers.get(transfer.transfer_id).await?;
        println!(
          "Transfer details:\n\tblock_hash: {}\n\text_hash: {}",
          details.first_block_hash, details.extrinsic_hash
        );
      }
      TransactionsSubcommand::ToMainchain {
        transfer_args: TransferArgs { argons },
        wait_for_immortalized,
        keystore_password,
      } => {
        let microgons = (argons * 1_000_000.0) as u128;
        let localchain = Localchain::load(LocalchainConfig {
          path,
          mainchain_url,
          ntp_pool_url: None,
          keystore_password: Some(keystore_password),
        })
        .await?;
        let change = localchain.begin_change();
        let mainchain_client = localchain.mainchain_client().await.ok_or(anyhow!(
          "Mainchain client not available. Ensure a mainchain url was provided."
        ))?;
        let main_account = change.default_deposit_account().await?;
        main_account.send_to_mainchain(microgons).await?;
        let notarization_tracker = change.notarize().await?;
        localchain.balance_sync().sync(None).await?;
        if wait_for_immortalized {
          notarization_tracker
            .wait_for_immortalized(&mainchain_client)
            .await?;
          println!(
            "Sent {} argons to mainchain. Immortalized in notebook {}",
            format_argons(microgons),
            notarization_tracker.notebook_number
          );
        } else {
          println!(
            "Sent {} argons to mainchain. Will be included in notebook {}",
            format_argons(microgons),
            notarization_tracker.notebook_number
          );
        }
      }
    },
  }
  Ok(())
}

fn account_columns() -> Vec<&'static str> {
  vec!["Name", "Address", "Balance", "Tax"]
}
fn format_account_record(name: &str, address: String, balance: String, tax: String) -> Vec<String> {
  vec![name.replace(".db", ""), address, balance, tax]
}

fn get_path(base_dir: Option<PathBuf>, name: String) -> String {
  let base_dir = base_dir.unwrap_or(PathBuf::from(Localchain::get_default_dir()));
  base_dir
    .join(format!("{}.db", name.replace(".db", "")))
    .to_str()
    .expect("Path should convert to a string")
    .to_string()
}

/// Parameters of the keystore
#[derive(Debug, Clone, Args)]
pub struct EmbeddedKeyPassword {
  /// Use interactive shell for entering the password used by the embedded keystore.
  #[arg(long, conflicts_with_all = &["key_password", "key_password_filename"])]
  pub key_password_interactive: bool,

  /// Password used by the embedded keystore.
  ///
  /// This allows appending an extra user-defined secret to the seed.
  #[arg(
    long,
    conflicts_with_all = &["key_password_interactive", "key_password_filename"]
  )]
  pub key_password: Option<String>,

  /// File that contains the password used by the embedded keystore.
  #[arg(
    long,
    value_name = "PATH",
    conflicts_with_all = &["key_password_interactive", "key_password"]
  )]
  pub key_password_filename: Option<String>,
}
