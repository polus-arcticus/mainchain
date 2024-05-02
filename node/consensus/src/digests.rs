use codec::{Codec, Decode, Encode};
use sp_runtime::{
	generic::DigestItem,
	traits::{Block as BlockT, Header as HeaderT},
	Digest,
};

use ulx_node_runtime::{AccountId, NotebookVerifyError};
use ulx_primitives::{
	digests::{
		BlockVoteDigest, FinalizedBlockNeededDigest, AUTHOR_DIGEST_ID, BLOCK_SEAL_DIGEST_ID,
		BLOCK_VOTES_DIGEST_ID, FINALIZED_BLOCK_DIGEST_ID, NOTEBOOKS_DIGEST_ID,
		PARENT_VOTING_KEY_DIGEST, TICK_DIGEST_ID,
	},
	tick::Tick,
	BlockSealDigest, NotebookDigest, ParentVotingKeyDigest, TickDigest,
};

use crate::error::Error;

pub struct Digests<B: BlockT> {
	pub finalized_block: FinalizedBlockNeededDigest<B>,
	pub author: AccountId,
	pub block_vote: BlockVoteDigest,
	pub voting_key: ParentVotingKeyDigest,
	pub tick: TickDigest,
	pub notebooks: NotebookDigest<NotebookVerifyError>,
}

pub fn load_digests<B: BlockT>(header: &B::Header) -> Result<Digests<B>, Error<B>> {
	let mut author = None;
	let mut finalized_block = None;
	let mut block_vote = None;
	let mut tick = None;
	let mut notebooks = None;
	let mut parent_voting_key = None;

	for log in header.digest().logs() {
		match log {
			DigestItem::PreRuntime(FINALIZED_BLOCK_DIGEST_ID, v) => {
				if finalized_block.is_some() {
					return Err(Error::DuplicatePreRuntimeDigest(
						"FinalizedBlockNeededDigest".to_string(),
					));
				}
				let digest = FinalizedBlockNeededDigest::<B>::decode(&mut &v[..])
					.map_err(|e| Error::<B>::Codec(e.clone()))?;
				finalized_block = Some(digest);
			},
			DigestItem::PreRuntime(BLOCK_VOTES_DIGEST_ID, v) => {
				if block_vote.is_some() {
					return Err(Error::DuplicatePreRuntimeDigest("BlockVoteDigest".to_string()));
				}
				let digest = BlockVoteDigest::decode(&mut &v[..])
					.map_err(|e| Error::<B>::Codec(e.clone()))?;
				block_vote = Some(digest);
			},
			DigestItem::PreRuntime(AUTHOR_DIGEST_ID, v) => {
				if author.is_some() {
					return Err(Error::DuplicatePreRuntimeDigest("AuthorDigest".to_string()));
				}
				let digest =
					AccountId::decode(&mut &v[..]).map_err(|e| Error::<B>::Codec(e.clone()))?;
				author = Some(digest);
			},
			DigestItem::PreRuntime(TICK_DIGEST_ID, v) => {
				if tick.is_some() {
					return Err(Error::DuplicatePreRuntimeDigest("TickDigest".to_string()));
				}
				let digest =
					TickDigest::decode(&mut &v[..]).map_err(|e| Error::<B>::Codec(e.clone()))?;
				tick = Some(digest);
			},
			DigestItem::Consensus(PARENT_VOTING_KEY_DIGEST, v) => {
				if parent_voting_key.is_some() {
					return Err(Error::DuplicatePreRuntimeDigest(
						"ParentVotingKeyDigest".to_string(),
					));
				}
				let digest = ParentVotingKeyDigest::decode(&mut &v[..])
					.map_err(|e| Error::<B>::Codec(e.clone()))?;
				parent_voting_key = Some(digest);
			},
			DigestItem::PreRuntime(NOTEBOOKS_DIGEST_ID, v) => {
				if notebooks.is_some() {
					return Err(Error::DuplicatePreRuntimeDigest("NotebookDigest".to_string()));
				}
				let digest = NotebookDigest::decode(&mut &v[..])
					.map_err(|e| Error::<B>::Codec(e.clone()))?;
				notebooks = Some(digest);
			},
			_ => {},
		}
	}

	Ok(Digests {
		finalized_block: finalized_block
			.ok_or(Error::<B>::MissingPreRuntimeDigest("FinalizedBlockNeededDigest".to_string()))?,
		block_vote: block_vote
			.ok_or(Error::<B>::MissingPreRuntimeDigest("BlockVoteDigest".to_string()))?,
		author: author.ok_or(Error::<B>::MissingPreRuntimeDigest("AuthorDigest".to_string()))?,
		tick: tick.ok_or(Error::<B>::MissingPreRuntimeDigest("TickDigest".to_string()))?,
		notebooks: notebooks
			.ok_or(Error::<B>::MissingPreRuntimeDigest("NotebookDigest".to_string()))?,
		// since this comes from consensus (ie, runtime), we will allow runtime to verify
		voting_key: parent_voting_key.unwrap_or_default(),
	})
}

pub fn create_seal_digest(block_seal_digest: &BlockSealDigest) -> DigestItem {
	DigestItem::Seal(BLOCK_SEAL_DIGEST_ID, block_seal_digest.encode())
}

pub fn read_seal_digest(digest: &DigestItem) -> Option<BlockSealDigest> {
	digest.seal_try_to(&BLOCK_SEAL_DIGEST_ID)
}

pub fn get_tick_digest(digest: &Digest) -> Option<Tick> {
	for log in digest.logs() {
		if let Some(tick) = log.pre_runtime_try_to::<TickDigest>(&TICK_DIGEST_ID) {
			return Some(tick.tick);
		}
	}
	None
}

pub fn create_pre_runtime_digests<B: BlockT, A: Codec>(
	author: A,
	tick: Tick,
	block_vote_digest: BlockVoteDigest,
	finalized_block_needed_digest: FinalizedBlockNeededDigest<B>,
	notebooks: NotebookDigest<NotebookVerifyError>,
) -> Digest {
	let mut inherent_digest = Digest::default();

	// add author in pow standard field (for client)
	inherent_digest.push(DigestItem::PreRuntime(AUTHOR_DIGEST_ID, author.encode()));
	inherent_digest.push(DigestItem::PreRuntime(TICK_DIGEST_ID, TickDigest { tick }.encode()));
	inherent_digest.push(DigestItem::PreRuntime(
		FINALIZED_BLOCK_DIGEST_ID,
		finalized_block_needed_digest.encode(),
	));
	inherent_digest.push(DigestItem::PreRuntime(BLOCK_VOTES_DIGEST_ID, block_vote_digest.encode()));
	inherent_digest.push(DigestItem::PreRuntime(NOTEBOOKS_DIGEST_ID, notebooks.encode()));
	inherent_digest
}
