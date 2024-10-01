use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;
use sp_core::sr25519::Signature;
use sp_core::Decode;
use sp_core::H256;
use sp_runtime::MultiSignature;
use sqlx::{Sqlite, SqlitePool, Transaction};
use tokio::sync::Mutex;

use crate::{bail, Error, Result};
use argon_primitives::tick::Tick;
use argon_primitives::{AccountType, BlockVote, NotaryId, NotebookNumber};

use crate::accounts::AccountStore;
use crate::balance_changes::{BalanceChangeRow, BalanceChangeStatus, BalanceChangeStore};
use crate::keystore::Keystore;
use crate::mainchain_transfer::MainchainTransferStore;
use crate::notarization_builder::NotarizationBuilder;
use crate::notarization_tracker::NotarizationTracker;
use crate::open_channel_holds::OpenChannelHoldsStore;
use crate::transactions::{TransactionType, Transactions};
use crate::{ChannelHold, MainchainClient};
use crate::{LocalAccount, NOTARIZATION_MAX_BALANCE_CHANGES};
use crate::{Localchain, OpenChannelHold};
use crate::{LocalchainTransfer, NotaryAccountOrigin, TickerRef};
use crate::{NotaryClient, NotaryClients};

#[cfg_attr(feature = "napi", napi)]
pub struct BalanceSync {
  db: SqlitePool,
  ticker: TickerRef,
  mainchain_client: Arc<Mutex<Option<MainchainClient>>>,
  notary_clients: NotaryClients,
  lock: Arc<Mutex<()>>,
  open_channel_holds: OpenChannelHoldsStore,
  keystore: Keystore,
  tick_counter: Arc<Mutex<(u32, u32)>>,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Clone)]
pub struct ChannelHoldCloseOptions {
  pub votes_address: Option<String>,
  /// What's the minimum amount of tax we should wait for before voting on blocks
  pub minimum_vote_amount: Option<i64>,
}

#[cfg_attr(feature = "napi", napi)]
pub struct BalanceSyncResult {
  pub(crate) balance_changes: Vec<BalanceChangeRow>,
  pub(crate) mainchain_transfers: Vec<NotarizationTracker>,
  pub(crate) channel_hold_notarizations: Vec<NotarizationBuilder>,
  pub(crate) jump_account_consolidations: Vec<NotarizationTracker>,
  pub(crate) block_votes: Vec<NotarizationTracker>,
}

impl BalanceSyncResult {
  pub fn balance_changes(&self) -> Vec<BalanceChangeRow> {
    self.balance_changes.clone()
  }
  pub fn channel_hold_notarizations(&self) -> Vec<NotarizationBuilder> {
    self.channel_hold_notarizations.clone()
  }

  pub fn mainchain_transfers(&self) -> Vec<NotarizationTracker> {
    self.mainchain_transfers.clone()
  }

  pub fn jump_account_consolidations(&self) -> Vec<NotarizationTracker> {
    self.jump_account_consolidations.clone()
  }

  pub fn block_votes(&self) -> Vec<NotarizationTracker> {
    self.block_votes.clone()
  }
}

#[cfg(feature = "uniffi")]
pub mod uniffi_ext {
  use crate::notarization_tracker::uniffi_ext::BalanceChange;
  use crate::notarization_tracker::uniffi_ext::NotarizationTracker;
  use std::sync::Arc;

  #[derive(uniffi::Record)]
  pub struct BalanceSyncResult {
    pub balance_changes: Vec<BalanceChange>,
    pub mainchain_transfers: Vec<Arc<NotarizationTracker>>,
    pub jump_account_consolidations: Vec<Arc<NotarizationTracker>>,
    pub block_votes: Vec<Arc<NotarizationTracker>>,
  }

  impl From<super::BalanceSyncResult> for BalanceSyncResult {
    fn from(result: super::BalanceSyncResult) -> Self {
      BalanceSyncResult {
        balance_changes: result
          .balance_changes
          .into_iter()
          .map(|x| x.into())
          .collect(),
        mainchain_transfers: result
          .mainchain_transfers
          .into_iter()
          .map(|x| Arc::new(x.into()))
          .collect(),
        jump_account_consolidations: result
          .jump_account_consolidations
          .into_iter()
          .map(|x| Arc::new(x.into()))
          .collect(),
        block_votes: result
          .block_votes
          .into_iter()
          .map(|x| Arc::new(x.into()))
          .collect(),
      }
    }
  }
}

#[cfg(feature = "napi")]
pub mod napi_ext {
  use crate::error::NapiOk;
  use crate::notarization_builder::NotarizationBuilder;
  use crate::notarization_tracker::NotarizationTracker;
  use crate::{
    BalanceChangeRow, BalanceSync, BalanceSyncResult, ChannelHoldCloseOptions, Localchain,
  };
  use napi_derive::napi;

  #[napi]
  impl BalanceSyncResult {
    #[napi(getter, js_name = "balanceChanges")]
    pub fn balance_changes_napi(&self) -> Vec<BalanceChangeRow> {
      self.balance_changes.clone()
    }
    #[napi(getter, js_name = "channelHoldNotarizations")]
    pub fn channel_hold_notarizations_napi(&self) -> Vec<NotarizationBuilder> {
      self.channel_hold_notarizations.clone()
    }
    #[napi(getter, js_name = "mainchainTransfers")]
    pub fn mainchain_transfers_napi(&self) -> Vec<NotarizationTracker> {
      self.mainchain_transfers.clone()
    }
    #[napi(getter, js_name = "jumpAccountConsolidations")]
    pub fn jump_account_consolidations_napi(&self) -> Vec<NotarizationTracker> {
      self.jump_account_consolidations.clone()
    }
    #[napi(getter, js_name = "blockVotes")]
    pub fn block_votes_napi(&self) -> Vec<NotarizationTracker> {
      self.block_votes.clone()
    }
  }

  #[napi]
  impl BalanceSync {
    #[napi(constructor)]
    pub fn new_napi(localchain: &Localchain) -> Self {
      Self::new(localchain)
    }
    #[napi(js_name = "sync")]
    pub async fn sync_napi(
      &self,
      options: Option<ChannelHoldCloseOptions>,
    ) -> napi::Result<BalanceSyncResult> {
      self.sync(options).await.napi_ok()
    }
    #[napi(js_name = "consolidateJumpAccounts")]
    pub async fn consolidate_jump_accounts_napi(&self) -> napi::Result<Vec<NotarizationTracker>> {
      self.consolidate_jump_accounts().await.napi_ok()
    }
    #[napi(js_name = "syncUnsettledBalances")]
    pub async fn sync_unsettled_balances_napi(&self) -> napi::Result<Vec<BalanceChangeRow>> {
      self.sync_unsettled_balances().await.napi_ok()
    }
    #[napi(js_name = "syncMainchainTransfers")]
    pub async fn sync_mainchain_transfers_napi(&self) -> napi::Result<Vec<NotarizationTracker>> {
      self.sync_mainchain_transfers().await.napi_ok()
    }
    #[napi(js_name = "convertTaxToVotes")]
    pub async fn convert_tax_to_votes_napi(
      &self,
      options: ChannelHoldCloseOptions,
    ) -> napi::Result<Vec<NotarizationTracker>> {
      self.convert_tax_to_votes(options).await.napi_ok()
    }

    #[napi(js_name = "syncBalanceChange")]
    pub async fn sync_balance_change_napi(
      &self,
      balance_change: &BalanceChangeRow,
    ) -> napi::Result<BalanceChangeRow> {
      self.sync_balance_change(balance_change).await.napi_ok()
    }
    #[napi(js_name = "processPendingChannelHolds")]
    pub async fn process_pending_channel_holds_napi(
      &self,
    ) -> napi::Result<Vec<NotarizationBuilder>> {
      self.process_pending_channel_holds().await.napi_ok()
    }
  }
}

impl BalanceSync {
  pub fn new(localchain: &Localchain) -> Self {
    BalanceSync {
      db: localchain.db.clone(),
      ticker: localchain.ticker.clone(),
      mainchain_client: localchain.mainchain_client.clone(),
      notary_clients: localchain.notary_clients.clone(),
      lock: Arc::new(Mutex::new(())),
      open_channel_holds: localchain.open_channel_holds(),
      tick_counter: Arc::new(Mutex::new((0, 0))),
      keystore: localchain.keystore.clone(),
    }
  }

  pub async fn sync(&self, options: Option<ChannelHoldCloseOptions>) -> Result<BalanceSyncResult> {
    let balance_changes = self.sync_unsettled_balances().await?;

    tracing::debug!(
      "Finished processing unsettled balances {}",
      balance_changes.len(),
    );

    let channel_hold_notarizations = self.process_pending_channel_holds().await?;

    let jump_account_consolidations = self.consolidate_jump_accounts().await?;

    let mainchain_transfers = self.sync_mainchain_transfers().await?;

    let block_votes = if let Some(options) = options {
      self
        .convert_tax_to_votes(options)
        .await
        .unwrap_or_else(|e| {
          tracing::warn!("Error converting tax to votes: {:?}", e);
          vec![]
        })
    } else {
      vec![]
    };

    Ok(BalanceSyncResult {
      balance_changes,
      channel_hold_notarizations,
      jump_account_consolidations,
      mainchain_transfers,
      block_votes,
    })
  }

  pub async fn consolidate_jump_accounts(&self) -> Result<Vec<NotarizationTracker>> {
    let mut db = self.db.acquire().await?;

    let all_accounts = AccountStore::db_list(&mut db, true).await?;
    let mut notarizations: Vec<NotarizationTracker> = vec![];
    let mut notarization = NotarizationBuilder::new(
      self.db.clone(),
      self.notary_clients.clone(),
      self.keystore.clone(),
      self.ticker.clone(),
    );
    for jump_account in all_accounts {
      let latest = BalanceChangeStore::db_get_latest_for_account(&mut db, jump_account.id).await?;
      if let Some(latest) = latest {
        let balance = latest.balance.parse::<u128>()?;
        if balance > 0 {
          let claim_account = match jump_account.account_type {
            AccountType::Deposit => notarization.default_deposit_account().await?,
            AccountType::Tax => notarization.default_tax_account().await?,
          };
          if claim_account.local_account_id == jump_account.id {
            continue;
          }
          notarization
            .load_account(&jump_account)
            .await?
            .send(balance, Some(vec![claim_account.address.clone()]))
            .await?;
          let claim_result = claim_account.claim(balance).await?;
          if claim_result.tax > 0 {
            notarization
              .default_tax_account()
              .await?
              .claim(claim_result.tax)
              .await?;
          }
          if notarization.accounts().await.len() >= NOTARIZATION_MAX_BALANCE_CHANGES as usize {
            let tracker = notarization.notarize().await?;
            notarizations.push(tracker);
            notarization = NotarizationBuilder::new(
              self.db.clone(),
              self.notary_clients.clone(),
              self.keystore.clone(),
              self.ticker.clone(),
            );
            let transaction =
              Transactions::create_static(&mut db, TransactionType::Consolidation).await?;
            notarization.set_transaction(transaction).await;
          }
        }
      }
    }

    if notarization.has_items_to_notarize().await {
      let transaction =
        Transactions::create_static(&mut db, TransactionType::Consolidation).await?;
      notarization.set_transaction(transaction).await;
      let tracker = notarization.notarize().await?;
      notarizations.push(tracker);
    }

    Ok(notarizations)
  }

  pub async fn sync_unsettled_balances(&self) -> Result<Vec<BalanceChangeRow>> {
    let mut db = self.db.acquire().await?;

    let pending_changes = BalanceChangeStore::db_find_unsettled(&mut db).await?;
    tracing::debug!("Found {} unsettled balance changes", pending_changes.len());

    let mut results = vec![];

    for change in pending_changes.into_iter() {
      let updated = self.sync_balance_change(&change).await?;
      results.push(updated);
    }

    Ok(results)
  }

  pub async fn sync_mainchain_transfers(&self) -> Result<Vec<NotarizationTracker>> {
    {
      let mainchain_mutex = self.mainchain_client.lock().await;
      let Some(_) = *mainchain_mutex else {
        return Ok(vec![]);
      };
    }
    let _lock = self.lock.lock().await;
    let mainchain_transfers = MainchainTransferStore::new(
      self.db.clone(),
      self.mainchain_client.clone(),
      self.keystore.clone(),
    );
    mainchain_transfers.update_finalization().await?;

    let transfers = mainchain_transfers.find_ready_for_claim().await?;
    if transfers.is_empty() {
      return Ok(vec![]);
    }
    let notarization = NotarizationBuilder::new(
      self.db.clone(),
      self.notary_clients.clone(),
      self.keystore.clone(),
      self.ticker.clone(),
    );
    let mut notarizations = vec![];

    for x in &transfers {
      let transfer = LocalchainTransfer {
        address: x.address.clone(),
        amount: x.amount.parse::<u128>().unwrap_or_default(),
        transfer_id: x.transfer_id as u32,
        notary_id: x.notary_id as u32,
        expiration_tick: x.expiration_tick.unwrap_or_default() as u32,
      };
      notarization.claim_from_mainchain(transfer).await?;
    }

    if notarization.has_items_to_notarize().await {
      let tracker = notarization.notarize().await?;
      for transfer in transfers {
        mainchain_transfers
          .record_balance_change_id(transfer.id, tracker.notarization_id)
          .await?;
      }
      notarizations.push(tracker);
    }

    Ok(notarizations)
  }

  pub async fn sync_balance_change(
    &self,
    balance_change: &BalanceChangeRow,
  ) -> Result<BalanceChangeRow> {
    let _lock = self.lock.lock().await;
    let mut change = balance_change.clone();

    let mut db = self.db.acquire().await?;
    tracing::debug!(
      "Checking status of balance change; id={}, change #{} (account={}), in status {:?}.",
      change.id,
      change.change_number,
      change.account_id,
      change.status
    );
    match BalanceChangeStore::db_check_if_superseded(&mut db, &mut change).await {
      Ok(true) => {
        tracing::debug!(
          "Balance Change superseded by another change; id={}.",
          change.id,
        );
        return Ok(change.clone());
      }
      Ok(false) => (),
      Err(e) => {
        tracing::warn!("Error checking if superseded (#{}): {:?}", change.id, e);
      }
    }

    let mut check_notary_for_tip = change.status == BalanceChangeStatus::WaitingForSendClaim;
    if change.status == BalanceChangeStatus::Notarized {
      check_notary_for_tip =
        match Self::sync_notebook_proof(&self.db, &mut change, &self.notary_clients).await {
          Ok(x) => x,
          Err(e) => {
            if is_notebook_finalization_error(&e) {
              return Ok(change);
            }

            tracing::warn!("Error syncing notebook proof (#{}): {:?}", change.id, e);
            true
          }
        };
    }

    if check_notary_for_tip {
      match Self::check_notary(&self.db, &mut change, &self.notary_clients).await {
        Ok(_) => {}
        Err(e) => {
          if !is_notebook_finalization_error(&e) {
            tracing::warn!("Error checking notary (#{}): {:?}", change.id, e);
          }
        }
      }
    }

    if change.status == BalanceChangeStatus::NotebookPublished {
      match self.check_finalized(&mut change).await {
        Ok(_) => {}
        Err(e) => {
          if !is_notebook_finalization_error(&e) {
            tracing::warn!("Error checking finalized (#{}): {:?}", change.id, e);
          }
        }
      }
    }
    Ok(change)
  }

  pub async fn process_pending_channel_holds(&self) -> Result<Vec<NotarizationBuilder>> {
    let _lock = self.lock.lock().await;
    let open_channel_holds = self.open_channel_holds.get_claimable().await?;
    tracing::debug!(
      "Processing pending channel_holds. Found {} to check for claims.",
      open_channel_holds.len(),
    );

    let mut builder_by_notary = HashMap::new();

    let mut notarizations = vec![];

    for open_channel_hold in open_channel_holds {
      let mut channel_hold = open_channel_hold.inner().await;

      let notary_id = channel_hold.notary_id;

      let notarization = builder_by_notary.entry(notary_id).or_insert_with(|| {
        NotarizationBuilder::new(
          self.db.clone(),
          self.notary_clients.clone(),
          self.keystore.clone(),
          self.ticker.clone(),
        )
      });

      if channel_hold.is_client {
        if let Err(e) = self
          .sync_client_channel_hold(
            notary_id,
            &open_channel_hold,
            &mut channel_hold,
            notarization,
          )
          .await
        {
          tracing::warn!(
            "Error syncing client channel_hold (#{}): {:?}",
            channel_hold.id,
            e
          );
        } else {
          let _ = open_channel_hold.reload().await;
        }
      } else {
        if let Err(e) = self
          .sync_server_channel_hold(&open_channel_hold, &mut channel_hold, notarization)
          .await
        {
          tracing::warn!(
            "Error syncing server channel_hold (#{}): {:?}",
            channel_hold.id,
            e
          );
        }
        if notarization.is_finalized().await {
          if let Some(n) = builder_by_notary.remove(&notary_id) {
            notarizations.push(n);
          }
        }
      }
    }

    for (_, mut notarization) in builder_by_notary {
      if !notarization.has_items_to_notarize().await {
        continue;
      }
      self
        .finalize_channel_hold_notarization(&mut notarization)
        .await;
      if notarization.is_finalized().await {
        notarizations.push(notarization.clone());
      }
    }

    Ok(notarizations)
  }

  pub async fn convert_tax_to_votes(
    &self,
    options: ChannelHoldCloseOptions,
  ) -> Result<Vec<NotarizationTracker>> {
    let Some(votes_address) = options.votes_address.as_ref() else {
      bail!("No votes address provided to create votes with tax");
    };

    let mainchain_mutex = self.mainchain_client.lock().await;
    let Some(ref mainchain_client) = *mainchain_mutex else {
      bail!("Cannot create votes.. No mainchain client available!");
    };

    let mut notarizations = vec![];
    let mut db = self.db.acquire().await?;
    let accounts = AccountStore::db_list(&mut db, false).await?;
    drop(db);

    for account in accounts {
      if account.account_type == AccountType::Deposit {
        continue;
      }
      let notarization = NotarizationBuilder::new(
        self.db.clone(),
        self.notary_clients.clone(),
        self.keystore.clone(),
        self.ticker.clone(),
      );
      let balance_change = notarization.load_account(&account).await?;
      let total_available_tax = balance_change.balance().await;
      if total_available_tax < options.minimum_vote_amount.unwrap_or_default() as u128 {
        continue;
      }

      balance_change.send_to_vote(total_available_tax).await?;

      let current_tick = self.ticker.current();
      let Some(best_block_for_vote) = mainchain_client.get_vote_block_hash(current_tick).await?
      else {
        continue;
      };
      if total_available_tax < best_block_for_vote.vote_minimum {
        continue;
      }

      let mut tick_counter = self.tick_counter.lock().await;
      if tick_counter.0 == current_tick {
        tick_counter.1 += 1;
      } else {
        *tick_counter = (current_tick, 0);
      }
      let vote_address = votes_address.clone();

      let mut vote = BlockVote {
        account_id: account.get_account_id32()?,
        power: total_available_tax,
        index: tick_counter.1,
        block_hash: H256::from_slice(best_block_for_vote.block_hash.as_ref()),
        block_rewards_account_id: AccountStore::parse_address(&vote_address)?,
        signature: Signature::from_raw([0; 64]).into(),
      };
      let signature = self
        .keystore
        .sign(account.address, vote.hash().as_bytes().to_vec())
        .await?;
      vote.signature = MultiSignature::decode(&mut signature.as_ref())?;
      notarization.add_vote(vote).await?;
      let tracker = notarization.notarize().await?;
      notarizations.push(tracker);
    }

    Ok(notarizations)
  }

  pub async fn finalize_channel_hold_notarization(&self, notarization: &mut NotarizationBuilder) {
    if let Err(e) = notarization.sign().await {
      tracing::error!("Could not claim a channel_hold -> {:?}", e);
      return;
    }

    for i in 0..3 {
      if i > 0 {
        tracing::debug!("Retrying notarization finalization. Attempt #{}", i);
      }

      match notarization.notarize().await {
        Ok(tracker) => {
          tracing::info!(
            "Finalized channel_hold notarization. id={}, balance_changes={}, votes={}",
            tracker.notarization_id,
            tracker.notarized_balance_changes,
            tracker.notarized_votes
          );
          break;
        }
        Err(e) => {
          if is_notebook_finalization_error(&e)
            || matches!(
              e,
              Error::NotaryApiError(argon_notary_apis::Error::BalanceChangeVerifyError(
                argon_notary_audit::VerifyError::ChannelHoldNotReadyForClaim { .. }
              ))
            )
          {
            let delay = (2 + i) ^ 5;
            tracing::debug!("Channel hold not ready for claim. Waiting {delay} seconds.");
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
            continue;
          }
          tracing::warn!("Error finalizing channel_hold notarization: {:?}", e);
        }
      }
    }
  }

  pub async fn sync_server_channel_hold(
    &self,
    open_channel_hold: &OpenChannelHold,
    channel_hold: &mut ChannelHold,
    notarization: &mut NotarizationBuilder,
  ) -> Result<()> {
    let current_tick = self.ticker.current();

    if channel_hold.is_past_claim_period(current_tick) {
      tracing::warn!(
        "ChannelHold expired and we missed claim window, marking unable to claim. id={}",
        channel_hold.id
      );
      let mut db = self.db.acquire().await?;
      channel_hold.db_mark_unable_to_claim(&mut db).await?;
      return Ok(());
    }

    tracing::debug!(
      "Server channel_hold {} ready for claim. channel_hold address={}, change number={}",
      channel_hold.id,
      channel_hold.from_address,
      channel_hold.balance_change_number
    );
    if !notarization.can_add_channel_hold(open_channel_hold).await {
      self.finalize_channel_hold_notarization(notarization).await;
      return Ok(());
    }
    notarization.claim_channel_hold(open_channel_hold).await?;
    Ok(())
  }

  pub async fn sync_client_channel_hold(
    &self,
    notary_id: u32,
    open_channel_hold: &OpenChannelHold,
    channel_hold: &mut ChannelHold,
    notarization: &mut NotarizationBuilder,
  ) -> Result<()> {
    let tip = self
      .notary_clients
      .get(notary_id)
      .await?
      .get_balance_tip(channel_hold.from_address.clone(), AccountType::Deposit)
      .await?;

    let hold_notebook = channel_hold.hold_notebook_number();
    // hasn't changed - aka, nothing synced
    if tip.notebook_number == hold_notebook {
      let current_tick = self.ticker.current();
      if channel_hold.is_past_claim_period(current_tick) {
        tracing::info!(
          "A channel_hold was not claimed by the recipient. We're taking it back. id={}",
          channel_hold.id
        );
        notarization.cancel_channel_hold(open_channel_hold).await?;
      }
      return Ok(());
    }

    tracing::debug!(
      "Trying to sync a client channel_hold that appears to have been updated by the recipient. channel_hold address={}, change number={}",
      channel_hold.from_address,
      channel_hold.balance_change_number
    );

    // will handle notarization
    let _ = self
      .sync_notarization(
        channel_hold.from_address.clone(),
        AccountType::Deposit,
        notary_id,
        tip.notebook_number,
        channel_hold.balance_change_number,
        tip.tick,
      )
      .await?;
    notarization.add_channel_hold(open_channel_hold).await;
    Ok(())
  }

  pub async fn sync_notarization(
    &self,
    address: String,
    account_type: AccountType,
    notary_id: NotaryId,
    notebook_number: NotebookNumber,
    change_number: u32,
    tick: Tick,
  ) -> Result<i64> {
    let mut tx = self.db.begin().await?;
    let account = AccountStore::db_get(&mut tx, address, account_type, notary_id).await?;
    let notary_client = self.notary_clients.get(notary_id).await?;

    let notarization = notary_client
      .get_notarization(
        account.get_account_id32()?,
        account_type,
        notebook_number,
        change_number,
      )
      .await?;

    let json = json!(&notarization);

    let notarization_id = sqlx::query_scalar!(
      "INSERT INTO notarizations (json, notary_id, notebook_number, tick) VALUES (?, ?, ?, ?) RETURNING id",
        json,
        notary_id,
        notebook_number,
        tick,
      )
      .fetch_one(&mut *tx)
      .await?;

    let transaction_id = sqlx::query_scalar!("SELECT transaction_id FROM balance_changes WHERE notarization_id = ? AND transaction_id IS NOT NULL", notarization_id)
        .fetch_optional(&mut *tx)
        .await?
        .map(|a| a.unwrap());

    for balance_change in notarization.balance_changes.iter() {
      let _ =
        OpenChannelHoldsStore::db_record_notarized(&mut tx, balance_change, notarization_id).await;

      BalanceChangeStore::tx_upsert_notarized(
        &mut tx,
        account.id,
        balance_change,
        notary_id,
        notarization_id,
        transaction_id,
      )
      .await?;
    }

    tx.commit().await?;

    Ok(notarization_id)
  }

  pub async fn check_notary(
    db: &SqlitePool,
    balance_change: &mut BalanceChangeRow,
    notary_clients: &NotaryClients,
  ) -> Result<()> {
    let mut tx = db.begin().await?;
    let mut account = AccountStore::db_get_by_id(&mut tx, balance_change.account_id).await?;
    let notary_id = balance_change.notary_id as u32;
    let notary_client = notary_clients.get(notary_id).await?;
    if account.origin.is_none() {
      let is_synched = Self::sync_account_origin(&mut tx, &mut account, &notary_client).await?;
      if !is_synched {
        return Ok(());
      }
    }

    let mut needs_notarization_download = true;
    let mut notebook_number = None;
    let expected_tip = balance_change.get_balance_tip(&account)?;
    if let Some(notarization_id) = balance_change.notarization_id {
      let record = sqlx::query!(
        "SELECT notebook_number, json IS NOT NULL as json FROM notarizations WHERE id = ?",
        notarization_id
      )
      .fetch_one(&mut *tx)
      .await?;

      notebook_number = record.notebook_number.map(|a| a as u32);
      needs_notarization_download = record.json == 0;
    }

    if needs_notarization_download {
      let tip = notary_client
        .get_balance_tip(account.address.clone(), account.account_type)
        .await?;

      if tip.balance_tip.as_ref() != expected_tip.tip().as_slice() {
        return Ok(());
      }

      notebook_number = Some(tip.notebook_number);

      let notarization = notary_client
        .get_notarization(
          account.get_account_id32()?,
          account.account_type,
          tip.notebook_number,
          balance_change.change_number as u32,
        )
        .await?;
      tracing::debug!(
        "Downloaded notarization for balance change. id={}, notarization_id={:?}, change={}. In notebook #{}, tick {}.",
          balance_change.id,
          balance_change.notarization_id,
          balance_change.change_number,
          tip.notebook_number,
          tip.tick
      );

      let json = json!(notarization);

      let notarization_id = match balance_change.notarization_id {
        Some(id) => {
          sqlx::query!(
              "UPDATE notarizations SET json = ?, notebook_number = ?, tick = ? WHERE id = ?",
              json,
              tip.notebook_number,
              tip.tick,
              id
            )
              .execute(&mut *tx)
              .await?;
          id
        },
        None =>
          sqlx::query_scalar!(
              "INSERT INTO notarizations (json, notary_id, notebook_number, tick) VALUES (?, ?, ?, ?) RETURNING id",
                json,
                balance_change.notary_id,
                tip.notebook_number,
                tip.tick,
              )
              .fetch_one(&mut *tx)
              .await?
      };
      balance_change.notarization_id = Some(notarization_id);
    }

    balance_change.status = BalanceChangeStatus::NotebookPublished;
    sqlx::query!(
      "UPDATE balance_changes SET notarization_id = ?, status = ? WHERE id = ?",
      balance_change.notarization_id,
      BalanceChangeStatus::NotebookPublished as i64,
      balance_change.id
    )
    .execute(&mut *tx)
    .await?;

    if let Some(notebook_number) = notebook_number {
      let result = notary_client
        .get_balance_proof(notebook_number, expected_tip)
        .await?;

      BalanceChangeStore::tx_save_notebook_proof(&mut tx, balance_change, &result).await?;
    }
    tx.commit().await?;

    Ok(())
  }

  pub async fn sync_account_origin(
    tx: &mut Transaction<'static, Sqlite>,
    account: &mut LocalAccount,
    notary_client: &NotaryClient,
  ) -> Result<bool> {
    let Ok(result) = notary_client
      .get_account_origin(account.address.clone(), account.account_type)
      .await
    else {
      return Ok(false);
    };
    AccountStore::db_update_origin(
      &mut *tx,
      account.id,
      result.notebook_number,
      result.account_uid,
    )
    .await?;
    account.origin = Some(NotaryAccountOrigin {
      account_uid: result.account_uid,
      notary_id: account.notary_id,
      notebook_number: result.notebook_number,
    });
    Ok(true)
  }

  /// Asks the notary to supply their own proof
  pub async fn sync_notebook_proof(
    db: &SqlitePool,
    balance_change: &mut BalanceChangeRow,
    notary_clients: &NotaryClients,
  ) -> Result<bool> {
    let mut tx = db.begin().await?;

    let notebook_number = sqlx::query_scalar!(
      "SELECT notebook_number FROM notarizations WHERE id = ?",
      balance_change.notarization_id
    )
    .fetch_one(&mut *tx)
    .await?;

    let Some(notebook_number) = notebook_number else {
      return Ok(false);
    };
    let mut account = AccountStore::db_get_by_id(&mut tx, balance_change.account_id).await?;
    let notary_client = notary_clients.get(balance_change.notary_id as u32).await?;

    if account.origin.is_none() {
      Self::sync_account_origin(&mut tx, &mut account, &notary_client).await?;
    }

    let tip = balance_change.get_balance_tip(&account)?;

    let result = notary_client
      .get_balance_proof(notebook_number as u32, tip)
      .await?;

    BalanceChangeStore::tx_save_notebook_proof(&mut tx, balance_change, &result).await?;
    tx.commit().await?;
    tracing::debug!(
      "Balance Change synched notebook proof; id={}. Notebook={}, tick={}",
      balance_change.id,
      notebook_number,
      result.tick
    );
    Ok(true)
  }

  pub async fn check_finalized(&self, balance_change: &mut BalanceChangeRow) -> Result<()> {
    let mut tx = self.db.begin().await?;

    let mainchain_mutex = self.mainchain_client.lock().await;
    let Some(ref mainchain_client) = *mainchain_mutex else {
      tracing::warn!(
        "Cannot synchronize finalization of balance change; id={}. No mainchain client available.",
        balance_change.id,
      );
      return Ok(());
    };

    let latest_notebook = mainchain_client
      .get_latest_notebook(balance_change.notary_id as u32)
      .await?;

    let latest_finalized = mainchain_client.latest_finalized_number().await?;

    let notebook_number = sqlx::query_scalar!(
      "SELECT notebook_number FROM notarizations WHERE id = ?",
      balance_change.notarization_id
    )
    .fetch_one(&mut *tx)
    .await?;

    let Some(notebook_number) = notebook_number else {
      return Ok(());
    };
    let notebook_number = notebook_number as u32;
    let notary_id = balance_change.notary_id as u32;

    if latest_notebook.notebook_number < notebook_number {
      return Ok(());
    }

    let Some(account_change_root) = mainchain_client
      .get_account_changes_root(notary_id, notebook_number)
      .await?
    else {
      return Ok(()); // not yet finalized
    };

    let account = AccountStore::db_get_by_id(&mut tx, balance_change.account_id).await?;
    let change_root = H256::from_slice(account_change_root.as_ref());
    BalanceChangeStore::tx_save_immortalized(
      &mut tx,
      balance_change,
      &account,
      change_root,
      latest_finalized,
    )
    .await?;
    tx.commit().await?;
    tracing::debug!(
      "Balance Change finalized and proof verified in mainchain; id={}. Block #{}",
      balance_change.id,
      latest_finalized
    );

    Ok(())
  }
}

fn is_notebook_finalization_error(e: &Error) -> bool {
  matches!(
    e,
    Error::NotaryApiError(argon_notary_apis::Error::NotebookNotFinalized)
  )
}
