#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::{apis::*, notary::NotaryId, note::*, notebook::*, providers::*};
pub use balance_change::*;
pub use block_seal::{BlockSealAuthorityId, BlockSealAuthoritySignature, BLOCK_SEAL_KEY_TYPE};
pub use block_vote::*;
pub use digests::{BlockSealDigest, AUTHOR_DIGEST_ID, BLOCK_SEAL_DIGEST_ID, *};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature,
};

pub type ComputeDifficulty = u128;

mod apis;
pub mod balance_change;
pub mod block_seal;
pub mod block_vote;
pub mod bond;
pub mod digests;
pub mod inherents;
pub mod macros;
pub mod notary;
pub mod note;
pub mod notebook;
mod providers;
pub mod tick;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A timestamp: milliseconds since the unix epoch.
pub type Moment = u64;

pub type BondId = u64;
pub type BondFundId = u32;

/// A hash of some data used by the chain.
pub type HashOutput = H256;
pub type BlockHash = BlakeTwo256;

pub mod localchain {
	pub use crate::{
		AccountType, BalanceChange, BestBlockVoteSeal, BlockVote, BlockVoteT, ChannelPass, Note,
		NoteType, VoteMinimum,
	};
}
