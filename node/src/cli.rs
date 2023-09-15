use sc_cli::RunCmd;
use ulx_node_runtime::AccountId;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: RunCmd,

	/// Enable mining and credit rewards to the given account.
	///
	/// The account address must be given in SS58 format.
	#[arg(long, value_name = "SS58_ADDRESS", value_parser = parse_ss58_account_id)]
	pub mine: Option<AccountId>,
}

impl Cli {
	pub fn block_author(&self) -> Option<AccountId> {
		if let Some(block_author) = &self.mine {
			Some(block_author.clone())
		} else if self.run.shared_params.dev {
			use sp_core::crypto::Pair;
			let block_author = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
			Some(AccountId::from(block_author.public()))
		} else {
			None
		}
	}
}

#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand {
	/// Key management cli utilities
	#[command(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Sub-commands concerned with benchmarking.
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,

	/// Db meta columns information.
	ChainInfo(sc_cli::ChainInfoCmd),
}

fn parse_ss58_account_id(data: &str) -> Result<AccountId, String> {
	sp_core::crypto::Ss58Codec::from_ss58check(data).map_err(|err| format!("{:?}", err))
}
