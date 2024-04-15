use codec::{Codec, Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::crypto::AccountId32;
use sp_core::{ConstU32, RuntimeDebug, H256, U256};
use sp_runtime::BoundedVec;
use sp_runtime::DispatchError;
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

use crate::{
	block_seal::MiningAuthority,
	notary::{NotaryId, NotaryNotebookVoteDetails, NotaryNotebookVoteDigestDetails},
	tick::{Tick, Ticker},
	AccountOrigin, BestBlockVoteSeal, BlockVoteDigest, BlockVotingPower, NotebookNumber,
	VoteMinimum,
};

sp_api::decl_runtime_apis! {
	pub trait BlockSealApis<AccountId:Codec, BlockSealAuthorityId:Codec> {
		fn vote_minimum() -> VoteMinimum;
		fn compute_difficulty() -> u128;
		fn create_vote_digest(tick: Tick, included_notebooks: Vec<NotaryNotebookVoteDigestDetails>) -> BlockVoteDigest;
		fn find_vote_block_seals(
			votes: Vec<NotaryNotebookVotes>,
			with_better_strength: U256,
		) -> Result<BoundedVec<BestBlockVoteSeal<AccountId, BlockSealAuthorityId>, ConstU32<2>>, DispatchError>;
	}
}

sp_api::decl_runtime_apis! {
	pub trait TickApis {
		fn current_tick() -> Tick;
		fn ticker() -> Ticker;
		fn blocks_at_tick(tick: Tick) -> Vec<Block::Hash>;
	}
}

sp_api::decl_runtime_apis! {
	pub trait NotaryApis<NotaryRecord> where
		NotaryRecord: Codec + MaxEncodedLen{
		fn notary_by_id(notary_id: NotaryId) -> Option<NotaryRecord>;
		fn notaries() -> Vec<NotaryRecord>;
	}
}

sp_api::decl_runtime_apis! {
	pub trait MiningApis<AccountId:Codec, BlockSealAuthorityId:Codec>{
		fn get_authority_id(account_id: &AccountId) -> Option<MiningAuthority<BlockSealAuthorityId,AccountId>>;
	}
}

sp_api::decl_runtime_apis! {
	pub trait NotebookApis<VerifyError: Codec> {
		fn audit_notebook_and_get_votes(
			version: u32,
			notary_id: NotaryId,
			notebook_number: NotebookNumber,
			header_hash: H256,
			vote_minimums: &BTreeMap<Block::Hash, VoteMinimum>,
			bytes: &Vec<u8>,
			audit_dependency_summaries: Vec<NotebookAuditSummary>,
		) -> Result<NotebookAuditResult, VerifyError>;


		fn decode_signed_raw_notebook_header(raw_header: Vec<u8>) -> Result<NotaryNotebookVoteDetails<Block::Hash>, DispatchError>;

		fn latest_notebook_by_notary() -> BTreeMap<NotaryId, (NotebookNumber, Tick)>;
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, RuntimeDebug)]
pub struct NotaryNotebookVotes {
	#[codec(compact)]
	pub notary_id: NotaryId,
	#[codec(compact)]
	pub notebook_number: NotebookNumber,
	pub raw_votes: Vec<(Vec<u8>, BlockVotingPower)>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, RuntimeDebug)]
pub struct NotebookAuditSummary {
	#[codec(compact)]
	pub notary_id: NotaryId,
	#[codec(compact)]
	pub notebook_number: NotebookNumber,
	#[codec(compact)]
	pub tick: Tick,
	pub changed_accounts_root: H256,
	pub account_changelist: Vec<AccountOrigin>,
	pub used_transfers_to_localchain: Vec<(AccountId32, u32)>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, RuntimeDebug)]
pub struct NotebookAuditResult {
	#[codec(compact)]
	pub notary_id: NotaryId,
	#[codec(compact)]
	pub notebook_number: NotebookNumber,
	#[codec(compact)]
	pub tick: Tick,
	pub raw_votes: Vec<(Vec<u8>, BlockVotingPower)>,
	pub changed_accounts_root: H256,
	pub account_changelist: Vec<AccountOrigin>,
	pub used_transfers_to_localchain: Vec<(AccountId32, u32)>,
}

impl Into<(NotebookAuditSummary, NotaryNotebookVotes)> for NotebookAuditResult {
	fn into(self) -> (NotebookAuditSummary, NotaryNotebookVotes) {
		(
			NotebookAuditSummary {
				notary_id: self.notary_id,
				notebook_number: self.notebook_number,
				tick: self.tick,
				changed_accounts_root: self.changed_accounts_root,
				account_changelist: self.account_changelist,
				used_transfers_to_localchain: self.used_transfers_to_localchain,
			},
			NotaryNotebookVotes {
				notary_id: self.notary_id,
				notebook_number: self.notebook_number,
				raw_votes: self.raw_votes,
			},
		)
	}
}
