use futures::channel::oneshot;
use sc_consensus::ImportResult;
use sp_blockchain::Error as BlockchainError;
use sp_consensus::Error as ConsensusError;
use sp_inherents::Error as InherentsError;
use sp_runtime::{
	traits::{Block as BlockT, NumberFor},
	RuntimeString,
};

#[derive(thiserror::Error, std::fmt::Debug)]
pub enum Error<B: BlockT> {
	#[error("Header uses the wrong engine {0:?}")]
	WrongEngine([u8; 4]),
	#[error("Header {0:?} is unsealed")]
	HeaderUnsealed(B::Hash),
	#[error("PoW validation error: invalid seal")]
	InvalidSeal,
	#[error("PoW validation error: nonce does not solve difficulty")]
	InvalidNonceDifficulty,
	#[error("Block seal signature missing or invalid")]
	InvalidSealSignature,
	#[error("Block sealer is not a valid authority")]
	InvalidSealSignerAuthority,
	#[error("PoW validation error: preliminary verification failed")]
	FailedPreliminaryVerify,
	#[error("Rejecting block too far in future")]
	TooFarInFuture,
	#[error("Invalid finalized block in predigests")]
	InvalidFinalizedBlockDigest,
	#[error("Pending download of finalized block in predigests")]
	PendingFinalizedBlockDigest(B::Hash, NumberFor<B>),
	#[error("Fetching best header failed using select chain: {0}")]
	BestHeaderSelectChain(ConsensusError),
	#[error("Fetching best header failed: {0}")]
	BestHeader(sp_blockchain::Error),
	#[error("Best header does not exist")]
	NoBestHeader,
	#[error("Block proposing error: {0}")]
	BlockProposingError(String),
	#[error("The miner is ineligible to submit this block seal")]
	InvalidBlockSubmitter,
	#[error("Cannot produce blocks as there are no active authorities in the keystore")]
	NoActiveAuthorityInKeystore,
	#[error("Fetch best hash failed via select chain: {0}")]
	BestHashSelectChain(ConsensusError),
	#[error("Error with block built on {0:?}: {1}")]
	BlockBuiltError(B::Hash, ConsensusError),
	#[error("Creating inherents failed: {0}")]
	CreateInherents(sp_inherents::Error),
	#[error("Checking inherents failed: {0}")]
	CheckInherents(sp_inherents::Error),
	#[error("Couldnt get the next work from the runtime")]
	CantGetNextWork,
	#[error(
	"Checking inherents unknown error for identifier: {}",
	String::from_utf8_lossy(.0)
	)]
	CheckInherentsUnknownError(sp_inherents::InherentIdentifier),
	#[error("Multiple pre-runtime digests")]
	MultiplePreRuntimeDigests,
	#[error("Missing pre-runtime digest")]
	MissingPreRuntimeDigest,
	#[error("Missing pre-runtime finalized block digest")]
	MissingFinalizedHeightDigest,
	#[error("Invalid work type proposed")]
	InvalidPredigestWorkType,
	#[error("Invalid difficulty proposed")]
	InvalidPredigestDifficulty,
	#[error(transparent)]
	Client(sp_blockchain::Error),
	#[error(transparent)]
	Codec(codec::Error),
	#[error("{0}")]
	Environment(String),
	#[error("{0}")]
	Runtime(RuntimeString),
	/// An error occurred while importing the block
	#[error("Block import failed: {0:?}")]
	BlockImportError(ImportResult),
	/// Transaction pool is empty, cannot create a block
	#[error(
		"Transaction pool is empty, set create_empty to true, if you want to create empty blocks"
	)]
	EmptyTransactionPool,
	/// encountered during creation of Proposer.
	#[error("Consensus Error: {0}")]
	ConsensusError(#[from] ConsensusError),
	/// Failed to create Inherents data
	#[error("Inherents Error: {0}")]
	InherentError(#[from] InherentsError),
	/// error encountered during finalization
	#[error("Finalization Error: {0}")]
	BlockchainError(#[from] BlockchainError),
	/// Supplied parent_hash doesn't exist in chain
	#[error("Supplied parent_hash: {0} doesn't exist in chain")]
	BlockNotFound(String),
	/// Some string error
	#[error("{0}")]
	StringError(String),
	/// send error
	#[error("Consensus process is terminating")]
	Canceled(#[from] oneshot::Canceled),
}

#[cfg(test)]
impl<B: BlockT> PartialEq for Error<B> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Error::WrongEngine(s1), Error::WrongEngine(s2)) => s1 == s2,
			(Error::HeaderUnsealed(s1), Error::HeaderUnsealed(s2)) => s1 == s2,
			(Error::InvalidSeal, Error::InvalidSeal) => true,
			(Error::InvalidNonceDifficulty, Error::InvalidNonceDifficulty) => true,
			(Error::InvalidSealSignature, Error::InvalidSealSignature) => true,
			(Error::InvalidSealSignerAuthority, Error::InvalidSealSignerAuthority) => true,
			(Error::FailedPreliminaryVerify, Error::FailedPreliminaryVerify) => true,
			(Error::TooFarInFuture, Error::TooFarInFuture) => true,
			(Error::BestHeaderSelectChain(s1), Error::BestHeaderSelectChain(s2)) =>
				eq_consensus_err(s1, s2),
			// (Error::BestHeader(s1), Error::BestHeader(s2)) => eq_consensus_err(s1, s2),
			(Error::NoBestHeader, Error::NoBestHeader) => true,
			(Error::BlockProposingError(s1), Error::BlockProposingError(s2)) => s1 == s2,
			(Error::InvalidBlockSubmitter, Error::InvalidBlockSubmitter) => true,
			(Error::NoActiveAuthorityInKeystore, Error::NoActiveAuthorityInKeystore) => true,
			(Error::BestHashSelectChain(s1), Error::BestHashSelectChain(s2)) =>
				eq_consensus_err(s1, s2),
			(Error::BlockBuiltError(s1, s2), Error::BlockBuiltError(s3, s4)) =>
				s1 == s3 && eq_consensus_err(s2, s4),
			(Error::CreateInherents(s1), Error::CreateInherents(s2)) => eq_inherent_err(s1, s2),
			(Error::CheckInherents(s1), Error::CheckInherents(s2)) => eq_inherent_err(s1, s2),
			(Error::CantGetNextWork, Error::CantGetNextWork) => true,
			(Error::CheckInherentsUnknownError(s1), Error::CheckInherentsUnknownError(s2)) =>
				s1 == s2,
			(Error::MultiplePreRuntimeDigests, Error::MultiplePreRuntimeDigests) => true,
			(Error::MissingPreRuntimeDigest, Error::MissingPreRuntimeDigest) => true,
			(Error::InvalidPredigestWorkType, Error::InvalidPredigestWorkType) => true,
			(Error::InvalidPredigestDifficulty, Error::InvalidPredigestDifficulty) => true,
			(Error::BlockImportError(s1), Error::BlockImportError(s2)) => s1 == s2,
			(Error::EmptyTransactionPool, Error::EmptyTransactionPool) => true,
			(Error::ConsensusError(s1), Error::ConsensusError(s2)) => eq_consensus_err(s1, s2),
			(Error::BlockNotFound(s1), Error::BlockNotFound(s2)) => s1 == s2,
			(Error::StringError(s1), Error::StringError(s2)) => s1 == s2,
			(Error::Canceled(_), Error::Canceled(_)) => true,

			_ => false,
		}
	}
}
#[cfg(test)]
fn eq_consensus_err(a: &ConsensusError, b: &ConsensusError) -> bool {
	match (a, b) {
		(sp_consensus::Error::StateUnavailable(s), sp_consensus::Error::StateUnavailable(s1)) =>
			s == s1,
		(sp_consensus::Error::NoIntermediate, sp_consensus::Error::NoIntermediate) => true,
		(sp_consensus::Error::InvalidIntermediate, sp_consensus::Error::InvalidIntermediate) =>
			true,
		(
			sp_consensus::Error::InvalidSignature(s, auth),
			sp_consensus::Error::InvalidSignature(s2, auth2),
		) => s == s2 && auth == auth2,
		(
			sp_consensus::Error::InvalidAuthoritiesSet,
			sp_consensus::Error::InvalidAuthoritiesSet,
		) => true,
		(sp_consensus::Error::InvalidJustification, sp_consensus::Error::InvalidJustification) =>
			true,
		(sp_consensus::Error::ClientImport(s), sp_consensus::Error::ClientImport(s1)) => s == s1,
		(sp_consensus::Error::Other(s), sp_consensus::Error::Other(s1)) =>
			s.to_string() == s1.to_string(),
		_ => false,
	}
}
#[cfg(test)]
fn eq_inherent_err(a: &InherentsError, b: &InherentsError) -> bool {
	match (a, b) {
		(InherentsError::InherentDataExists(s1), InherentsError::InherentDataExists(s2)) =>
			s1 == s2,
		(InherentsError::FatalErrorReported, InherentsError::FatalErrorReported) => true,
		(InherentsError::Application(s1), InherentsError::Application(s2)) =>
			s1.to_string() == s2.to_string(),
		(
			InherentsError::DecodingFailed(s1_err, id1),
			InherentsError::DecodingFailed(s2_err, id2),
		) => s1_err == s2_err && id1 == id2,
		_ => false,
	}
}

impl<B: BlockT> From<ImportResult> for Error<B> {
	fn from(err: ImportResult) -> Self {
		Error::BlockImportError(err)
	}
}

impl<B: BlockT> From<String> for Error<B> {
	fn from(s: String) -> Self {
		Error::StringError(s)
	}
}

impl<B: BlockT> From<Error<B>> for String {
	fn from(error: Error<B>) -> String {
		error.to_string()
	}
}

impl<B: BlockT> From<Error<B>> for ConsensusError {
	fn from(error: Error<B>) -> ConsensusError {
		match error {
			Error::PendingFinalizedBlockDigest(_, _) => ConsensusError::Other(Box::new(error)),
			_ => ConsensusError::ClientImport(error.to_string()),
		}
	}
}