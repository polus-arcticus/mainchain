use binary_merkle_tree::{merkle_proof, merkle_root};
use codec::Encode;
use frame_support::{
	assert_err, assert_ok,
	pallet_prelude::{Get, Hooks},
};
use sp_core::{bounded_vec, ed25519::Public, OpaquePeerId, H256, U256};
use sp_keyring::ed25519::Keyring;
use sp_runtime::{
	traits::{BlakeTwo256, Header},
	BoundedVec, Digest, DigestItem,
};

use ulx_primitives::{
	block_seal::{MiningAuthority, PeerId},
	digests::{
		BlockVoteDigest, NotaryNotebookDigest, SealSource, AUTHORITY_DIGEST_ID,
		BLOCK_VOTES_DIGEST_ID,
	},
	inherents::BlockSealInherent,
	localchain::{BlockVote, ChannelPass},
	BlockSealAuthorityId, BlockSealerInfo, MerkleProof, AUTHOR_DIGEST_ID,
};

use crate::{
	mock::{BlockSeal, *},
	pallet::{
		TempAuthor, TempBlockSealAuthority, TempBlockSealerInfo, TempBlockVoteDigest,
		TempSealInherent,
	},
	Error,
};

#[test]
#[should_panic(expected = "No valid account id provided for block author.")]
fn it_should_panic_if_no_block_author() {
	new_test_ext().execute_with(|| BlockSeal::on_initialize(1));
}

#[test]
fn it_should_read_the_digests() {
	new_test_ext().execute_with(|| {
		let block_vote_digest = get_block_vote_digest(5, 1);
		let authority = BlockSealAuthorityId::from(Public([0; 32]));
		let pre_digest = Digest {
			logs: vec![
				author_digest(1),
				DigestItem::PreRuntime(AUTHORITY_DIGEST_ID, authority.encode()),
				DigestItem::PreRuntime(BLOCK_VOTES_DIGEST_ID, block_vote_digest.encode()),
			],
		};

		System::reset_events();
		System::initialize(&42, &System::parent_hash(), &pre_digest);
		BlockSeal::on_initialize(42);
		assert_eq!(TempAuthor::<Test>::get(), Some(1u64));
		assert_eq!(TempBlockSealAuthority::<Test>::get(), Some(authority));
		assert_eq!(TempSealInherent::<Test>::get(), None);
		assert_eq!(TempBlockVoteDigest::<Test>::get(), Some(block_vote_digest));

		TempSealInherent::<Test>::put(BlockSealInherent::Compute);
		TempBlockSealerInfo::<Test>::put(BlockSealerInfo {
			block_vote_rewards_account: 1,
			miner_rewards_account: 1,
			notaries_included: 1,
		});
		BlockSeal::on_finalize(42);

		assert_eq!(TempAuthor::<Test>::get(), None);
		assert_eq!(TempBlockSealAuthority::<Test>::get(), None);
		assert_eq!(TempBlockVoteDigest::<Test>::get(), None);
		assert_eq!(TempSealInherent::<Test>::get(), None);
	});
}
#[test]
fn it_should_only_allow_a_single_seal() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		TempSealInherent::<Test>::put(BlockSealInherent::Compute);

		// actually panics
		assert_err!(
			BlockSeal::apply(RuntimeOrigin::none(), BlockSealInherent::Compute),
			Error::<Test>::DuplicateBlockSealProvided
		);
	});
}

#[test]
#[should_panic(expected = "No valid block seal authority id provided for block author.")]
fn it_should_panic_if_no_authority_id() {
	new_test_ext().execute_with(|| {
		System::reset_events();
		System::initialize(
			&2,
			&System::parent_hash(),
			&Digest { logs: vec![author_digest(1), vote_digest(1, 1)] },
		);
		BlockSeal::on_initialize(2);
	});
}

#[test]
fn it_should_only_allow_compute_for_first_4() {
	new_test_ext().execute_with(|| {
		setup_blocks(1);
		let inherent = BlockSealInherent::Vote {
			notary_id: 1,
			block_vote: default_vote(),
			nonce: 1.into(),
			source_notebook_proof: MerkleProof {
				proof: Default::default(),
				number_of_leaves: 1,
				leaf_index: 0,
			},
			source_notebook_number: 1,
		};

		System::initialize(
			&2,
			&System::parent_hash(),
			&Digest { logs: vec![author_digest(1), authority_digest(1), vote_digest(1, 1)] },
		);
		BlockSeal::on_initialize(2);

		assert_err!(
			BlockSeal::apply(RuntimeOrigin::none(), inherent),
			Error::<Test>::NoEligibleVotingRoot,
		);
	});
}

#[test]
fn it_requires_the_nonce_to_match() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		setup_blocks(2);
		System::set_block_number(4);
		System::reset_events();
		let block_vote = default_vote();
		let parent_voting_key = H256::random();
		ParentVotingKey::set(Some(parent_voting_key.clone()));
		let nonce =
			block_vote.calculate_block_nonce(1, parent_voting_key.clone()) + U256::from(1u32);
		System::initialize(
			&4,
			&System::parent_hash(),
			&Digest { logs: vec![author_digest(1), authority_digest(1), vote_digest(1, 1)] },
		);
		BlockSeal::on_initialize(4);

		assert_err!(
			BlockSeal::apply(
				RuntimeOrigin::none(),
				BlockSealInherent::Vote {
					notary_id: 1,
					block_vote,
					nonce,
					source_notebook_proof: Default::default(),
					source_notebook_number: 1,
				}
			),
			Error::<Test>::InvalidNonce
		);
	});
}

#[test]
fn it_should_be_able_to_submit_a_seal() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		setup_blocks(2);
		System::set_block_number(4);
		System::reset_events();
		AuthorityList::set(vec![(10, default_authority())]);
		XorClosest::set(Some(MiningAuthority {
			authority_id: default_authority(),
			peer_id: empty_peer(),
			authority_index: 0,
			rpc_hosts: bounded_vec![],
		}));

		let parent_voting_key = H256::random();
		ParentVotingKey::set(Some(parent_voting_key.clone()));
		GrandpaVoteMinimum::set(Some(500));

		let block_vote = default_vote();
		let nonce = block_vote.calculate_block_nonce(1, parent_voting_key.clone());

		let root = merkle_root::<BlakeTwo256, _>(vec![block_vote.encode()]);
		VotingRoots::mutate(|a| a.insert((1, 2), (root, 1)));
		let merkle_proof = merkle_proof::<BlakeTwo256, _, _>(vec![block_vote.encode()], 0).proof;

		let inherent = BlockSealInherent::Vote {
			notary_id: 1,
			block_vote,
			nonce,
			source_notebook_proof: MerkleProof {
				proof: BoundedVec::truncate_from(merkle_proof),
				number_of_leaves: 1,
				leaf_index: 0,
			},
			source_notebook_number: 1,
		};

		System::initialize(
			&4,
			&System::parent_hash(),
			&Digest {
				logs: vec![author_digest(10), default_authority_digest(), vote_digest(1, 1)],
			},
		);
		BlockSeal::on_initialize(4);

		assert_ok!(BlockSeal::apply(RuntimeOrigin::none(), inherent.clone()));

		assert_eq!(TempSealInherent::<Test>::get(), Some(inherent));
		assert_eq!(BlockSeal::get(), SealSource::Vote);
		BlockSeal::on_finalize(4);
	});
}

#[test]
fn it_requires_vote_proof() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		setup_blocks(2);
		System::set_block_number(3);
		System::reset_events();
		AuthorityList::set(vec![(10, BlockSealAuthorityId::from(Public([0; 32])))]);

		let mut block_vote = default_vote();
		let merkle_proof = merkle_proof::<BlakeTwo256, _, _>(vec![block_vote.encode()], 0).proof;
		let source_notebook_proof = MerkleProof {
			proof: BoundedVec::truncate_from(merkle_proof),
			number_of_leaves: 1,
			leaf_index: 0,
		};
		let root = merkle_root::<BlakeTwo256, _>(vec![block_vote.encode()]);
		VotingRoots::mutate(|a| a.insert((1, 1), (root, 2)));

		// set block to 2 - not in the history
		assert_err!(
			BlockSeal::verify_vote_source(1, 2, &block_vote, source_notebook_proof.clone(), 1,),
			Error::<Test>::NoEligibleVotingRoot
		);

		// notebook number i mismatched
		assert_err!(
			BlockSeal::verify_vote_source(1, 1, &block_vote, source_notebook_proof.clone(), 1,),
			Error::<Test>::IneligibleNotebookUsed
		);
		assert_ok!(BlockSeal::verify_vote_source(
			1,
			1,
			&block_vote,
			source_notebook_proof.clone(),
			2,
		),);

		block_vote.power = 100;
		assert_err!(
			BlockSeal::verify_vote_source(1, 1, &block_vote, source_notebook_proof.clone(), 2,),
			Error::<Test>::InvalidBlockVoteProof
		);
	});
}
#[test]
fn it_checks_that_votes_are_for_great_grandpa() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		setup_blocks(2);
		System::set_block_number(4);
		let mut vote = default_vote();
		vote.grandparent_block_hash = System::block_hash(2);
		GrandpaVoteMinimum::set(Some(500));
		assert_err!(
			BlockSeal::verify_block_vote(&vote, &1, &default_authority(), 2),
			Error::<Test>::InvalidVoteGrandparentHash
		);
	});
}

#[test]
fn it_checks_tax_votes() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		setup_blocks(2);
		System::set_block_number(4);
		let vote = default_vote();
		let default_authority = default_authority();

		GrandpaVoteMinimum::set(Some(501));
		assert_err!(
			BlockSeal::verify_block_vote(&vote, &1, &default_authority, 2),
			Error::<Test>::InsufficientVotingPower
		);
		GrandpaVoteMinimum::set(Some(500));
		assert_err!(
			BlockSeal::verify_block_vote(&vote, &1, &default_authority, 2),
			Error::<Test>::UnregisteredBlockAuthor
		);
		AuthorityList::mutate(|a| a.push((1, default_authority.clone())));
		assert_err!(
			BlockSeal::verify_block_vote(&vote, &1, &default_authority, 2),
			Error::<Test>::InvalidSubmitter
		);
		XorClosest::set(Some(MiningAuthority {
			peer_id: empty_peer(),
			authority_id: default_authority.clone(),
			rpc_hosts: Default::default(),
			authority_index: 0,
		}));
		assert_ok!(BlockSeal::verify_block_vote(&vote, &1, &default_authority, 2));
	});
}

fn empty_peer() -> PeerId {
	PeerId(OpaquePeerId::default())
}
fn setup_blocks(blocks: u64) {
	let mut parent_hash = System::parent_hash();

	for i in 1..(blocks + 1) {
		System::reset_events();
		System::initialize(&i, &parent_hash, &Default::default());

		let header = System::finalize();
		parent_hash = header.hash();
		System::set_block_number(*header.number());
	}
}

fn default_authority() -> BlockSealAuthorityId {
	BlockSealAuthorityId::from(Keyring::Alice.public())
}

fn empty_channel_pass() -> ChannelPass {
	ChannelPass { miner_index: 0, zone_record_hash: H256::zero(), id: 0, at_block_height: 0 }
}

fn author_digest(author: u64) -> DigestItem {
	DigestItem::PreRuntime(AUTHOR_DIGEST_ID, author.encode())
}

fn authority_digest(authority_id: u8) -> DigestItem {
	let authority = BlockSealAuthorityId::from(Public([authority_id; 32]));
	DigestItem::PreRuntime(AUTHORITY_DIGEST_ID, authority.encode())
}
fn default_authority_digest() -> DigestItem {
	DigestItem::PreRuntime(AUTHORITY_DIGEST_ID, default_authority().encode())
}

fn vote_digest(notebooks: u32, votes: u32) -> DigestItem {
	DigestItem::PreRuntime(BLOCK_VOTES_DIGEST_ID, get_block_vote_digest(notebooks, votes).encode())
}

fn get_block_vote_digest(notebooks: u32, votes: u32) -> BlockVoteDigest {
	let numbers = (0..notebooks)
		.map(|a| NotaryNotebookDigest { notary_id: a, notebook_number: a })
		.collect::<Vec<_>>();
	BlockVoteDigest {
		notebook_numbers: BoundedVec::truncate_from(numbers),
		parent_voting_key: None,
		voting_power: 1,
		votes_count: votes,
	}
}

fn default_vote() -> BlockVote {
	BlockVote {
		grandparent_block_hash: System::block_hash(System::block_number().saturating_sub(4)),
		channel_pass: empty_channel_pass(),
		account_id: Keyring::Alice.into(),
		index: 1,
		power: 500,
	}
}
