use std::collections::{BTreeMap, BTreeSet};

use binary_merkle_tree::{merkle_proof, merkle_root};
use chrono::Utc;
use codec::Encode;
use frame_support::{assert_err, assert_ok, parameter_types};
use sp_core::{
	bounded::BoundedVec, bounded_vec, crypto::AccountId32, sr25519::Signature, Blake2Hasher, H256,
};
use sp_keyring::{
	Ed25519Keyring::{Dave, Ferdie},
	Sr25519Keyring::{Alice, Bob},
};
use sp_runtime::MultiSignature;

use ulx_notary_primitives::{
	balance_change::{AccountOrigin, BalanceChange, BalanceProof},
	note::{AccountType, Note, NoteType},
	BalanceTip, ChainTransfer, MerkleProof, NewAccountOrigin, Notebook, NotebookHeader,
	NotebookNumber,
};

use crate::{
	verify_previous_balance_proof, AccountHistoryLookupError, NotebookHistoryLookup, VerifyError,
};

use super::notebook_verify;

fn empty_signature() -> MultiSignature {
	Signature([0u8; 64]).into()
}

struct TestLookup;

parameter_types! {
	pub static NotebookRoots: BTreeMap<u32, H256> = BTreeMap::new();
	pub static LastChangedNotebook: BTreeMap<AccountOrigin, u32> = BTreeMap::new();
	pub static ValidLocalchainTransfers: BTreeSet<(AccountId32, u32)> = BTreeSet::new();
}
impl NotebookHistoryLookup for TestLookup {
	fn get_account_changes_root(
		_notary_id: u32,
		notebook_number: NotebookNumber,
	) -> Result<H256, AccountHistoryLookupError> {
		NotebookRoots::get()
			.get(&notebook_number)
			.ok_or(AccountHistoryLookupError::RootNotFound)
			.cloned()
	}
	fn get_last_changed_notebook(
		_notary_id: u32,
		account_origin: AccountOrigin,
	) -> Result<u32, AccountHistoryLookupError> {
		LastChangedNotebook::get()
			.get(&account_origin)
			.cloned()
			.ok_or(AccountHistoryLookupError::LastChangeNotFound)
	}
	fn is_valid_transfer_to_localchain(
		_notary_id: u32,
		account_id: &AccountId32,
		nonce: u32,
	) -> Result<bool, AccountHistoryLookupError> {
		ValidLocalchainTransfers::get()
			.get(&(account_id.clone(), nonce))
			.cloned()
			.ok_or(AccountHistoryLookupError::InvalidTransferToLocalchain)
			.map(|_| true)
	}
}

#[test]
fn test_verify_previous_balance() {
	let mut final_balances = BTreeMap::<(AccountId32, AccountType), BalanceTip>::new();
	let account_id = Alice.to_account_id();
	let account_type = AccountType::Deposit;
	let key = (account_id.clone(), account_type.clone());

	let mut change = BalanceChange {
		account_id,
		account_type,
		change_number: 500,
		balance: 0,
		previous_balance_proof: None,
		channel_hold_note: None,
		notes: bounded_vec![],
		signature: empty_signature(),
	};
	let leaves = vec![
		BalanceTip {
			account_id: Dave.to_account_id(),
			account_type: AccountType::Deposit,
			balance: 20,
			change_number: 3,
			account_origin: AccountOrigin { notebook_number: 5, account_uid: 2 },
			channel_hold_note: None,
		}
		.encode(),
		BalanceTip {
			account_id: Bob.to_account_id(),
			account_type: AccountType::Deposit,
			balance: 100,
			change_number: 1,
			account_origin: AccountOrigin { notebook_number: 6, account_uid: 1 },
			channel_hold_note: None,
		}
		.encode(),
		BalanceTip {
			account_id: change.account_id.clone(),
			account_type: change.account_type.clone(),
			balance: 100,
			change_number: change.change_number - 1,
			account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
			channel_hold_note: None,
		}
		.encode(),
	];
	let merkle_root = merkle_root::<Blake2Hasher, _>(&leaves);
	NotebookRoots::mutate(|a| {
		a.insert(7, H256::from_slice([&[0u8], &merkle_root[0..31]].concat().as_ref()))
	});
	let origin = AccountOrigin { notebook_number: 1, account_uid: 1 };
	LastChangedNotebook::mutate(|c| c.insert(origin.clone(), 10));

	let proof = merkle_proof::<Blake2Hasher, _, _>(leaves, 2);
	change.previous_balance_proof = Some(BalanceProof {
		notary_id: 1,
		notebook_number: 7,
		notebook_proof: Some(MerkleProof {
			proof: BoundedVec::truncate_from(proof.proof),
			leaf_index: proof.leaf_index as u32,
			number_of_leaves: proof.number_of_leaves as u32,
		}),
		account_origin: origin.clone(),
		balance: 100,
	});

	assert_err!(
		verify_previous_balance_proof::<TestLookup>(
			&change.previous_balance_proof.clone().unwrap(),
			7,
			&mut final_balances,
			&change,
			&key,
		),
		VerifyError::InvalidPreviousBalanceChangeNotebook
	);

	LastChangedNotebook::mutate(|c| c.insert(origin, 7));
	assert_err!(
		verify_previous_balance_proof::<TestLookup>(
			&change.previous_balance_proof.clone().unwrap(),
			7,
			&mut final_balances,
			&change,
			&key,
		),
		VerifyError::InvalidPreviousBalanceProof
	);

	NotebookRoots::mutate(|a| a.insert(7, merkle_root));
	assert_ok!(verify_previous_balance_proof::<TestLookup>(
		&change.previous_balance_proof.clone().unwrap(),
		7,
		&mut final_balances,
		&change,
		&key,
	));
}

#[test]
fn test_verify_notebook() {
	let note = Note::create(1000, NoteType::ClaimFromMainchain { account_nonce: 1 });

	let alice_balance_changeset = vec![BalanceChange {
		balance: 1000,
		change_number: 1,
		account_id: Alice.to_account_id(),
		account_type: AccountType::Deposit,
		previous_balance_proof: None,
		channel_hold_note: None,
		notes: bounded_vec![note],
		signature: empty_signature(),
	}
	.sign(Alice.pair())
	.clone()];
	let notebook_header1 = NotebookHeader {
		version: 1,
		notary_id: 1,
		notebook_number: 1,
		finalized_block_number: 100,
		pinned_to_block_number: 0,
		start_time: Utc::now().timestamp_millis() as u64 - 60_000,
		changed_accounts_root: merkle_root::<Blake2Hasher, _>(vec![BalanceTip {
			account_id: Alice.to_account_id(),
			account_type: AccountType::Deposit,
			balance: 1000,
			change_number: 1,
			account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
			channel_hold_note: None,
		}
		.encode()]),
		chain_transfers: bounded_vec![ChainTransfer::ToLocalchain {
			account_id: Alice.to_account_id(),
			account_nonce: 1,
		}],
		changed_account_origins: bounded_vec![AccountOrigin { notebook_number: 1, account_uid: 1 }],
		end_time: Utc::now().timestamp_millis() as u64,
	};

	ValidLocalchainTransfers::mutate(|a| a.insert((Alice.to_account_id(), 1)));
	let hash = notebook_header1.hash();

	let notebook1 = Notebook {
		header: notebook_header1.clone(),
		balance_changes: bounded_vec![BoundedVec::truncate_from(alice_balance_changeset.clone())],
		new_account_origins: bounded_vec![NewAccountOrigin::new(
			Alice.to_account_id(),
			AccountType::Deposit,
			1
		)],
	};

	assert_ok!(notebook_verify::<TestLookup>(&hash, &notebook1));

	let mut bad_hash = hash.clone();
	bad_hash.0[0] = 1;
	assert_err!(
		notebook_verify::<TestLookup>(&bad_hash, &notebook1),
		VerifyError::InvalidNotebookHash
	);

	let mut bad_notebook1 = notebook1.clone();
	let _ = bad_notebook1.header.chain_transfers.try_insert(
		0,
		ChainTransfer::ToLocalchain { account_id: Bob.to_account_id(), account_nonce: 2 },
	);
	assert_err!(
		notebook_verify::<TestLookup>(&hash, &bad_notebook1),
		VerifyError::InvalidChainTransfersList
	);

	let mut bad_notebook = notebook1.clone();
	bad_notebook.header.changed_accounts_root.0[0] = 1;
	assert_err!(
		notebook_verify::<TestLookup>(&hash, &bad_notebook),
		VerifyError::InvalidBalanceChangeRoot
	);
}

#[test]
fn test_disallows_double_claim() {
	let note1 = Note::create(1000, NoteType::ClaimFromMainchain { account_nonce: 1 });
	let note2 = Note::create(1000, NoteType::ClaimFromMainchain { account_nonce: 1 });

	let alice_balance_changeset = vec![BalanceChange {
		balance: 2000,
		change_number: 1,
		account_id: Alice.to_account_id(),
		account_type: AccountType::Deposit,
		previous_balance_proof: None,
		channel_hold_note: None,
		notes: bounded_vec![note1, note2],
		signature: empty_signature(),
	}
	.sign(Alice.pair())
	.clone()];
	let notebook_header1 = NotebookHeader {
		version: 1,
		notary_id: 1,
		notebook_number: 1,
		finalized_block_number: 100,
		pinned_to_block_number: 0,
		start_time: Utc::now().timestamp_millis() as u64 - 60_000,
		changed_accounts_root: merkle_root::<Blake2Hasher, _>(vec![BalanceTip {
			account_id: Alice.to_account_id(),
			account_type: AccountType::Deposit,
			balance: 2000,
			change_number: 1,
			account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
			channel_hold_note: None,
		}
		.encode()]),
		chain_transfers: bounded_vec![ChainTransfer::ToLocalchain {
			account_id: Alice.to_account_id(),
			account_nonce: 1,
		}],
		changed_account_origins: bounded_vec![AccountOrigin { notebook_number: 1, account_uid: 1 }],
		end_time: Utc::now().timestamp_millis() as u64,
	};

	ValidLocalchainTransfers::mutate(|a| a.insert((Alice.to_account_id(), 1)));
	let notebook1 = Notebook {
		header: notebook_header1.clone(),
		balance_changes: bounded_vec![BoundedVec::truncate_from(alice_balance_changeset.clone())],
		new_account_origins: bounded_vec![NewAccountOrigin::new(
			Alice.to_account_id(),
			AccountType::Deposit,
			1
		)],
	};
	let hash = notebook_header1.hash();

	assert_err!(
		notebook_verify::<TestLookup>(&hash, &notebook1),
		VerifyError::DuplicateChainTransfer
	);
}

#[test]
fn test_multiple_changesets_in_a_notebook() {
	let alice_balance_changeset = vec![
		BalanceChange {
			balance: 0,
			change_number: 1,
			account_id: Alice.to_account_id(),
			account_type: AccountType::Deposit,
			previous_balance_proof: None,
			channel_hold_note: None,
			notes: bounded_vec![
				Note::create(1000, NoteType::ClaimFromMainchain { account_nonce: 1 }),
				Note::create(1000, NoteType::Send { to: None }),
			],
			signature: empty_signature(),
		}
		.sign(Alice.pair())
		.clone(),
		BalanceChange {
			balance: 800,
			change_number: 1,
			account_id: Bob.to_account_id(),
			account_type: AccountType::Deposit,
			previous_balance_proof: None,
			channel_hold_note: None,
			notes: bounded_vec![
				Note::create(1000, NoteType::Claim),
				Note::create(200, NoteType::Tax),
			],
			signature: empty_signature(),
		}
		.sign(Bob.pair())
		.clone(),
		BalanceChange {
			balance: 200,
			change_number: 1,
			account_id: Bob.to_account_id(),
			account_type: AccountType::Tax,
			previous_balance_proof: None,
			channel_hold_note: None,
			notes: bounded_vec![Note::create(200, NoteType::Claim),],
			signature: empty_signature(),
		}
		.sign(Bob.pair())
		.clone(),
	];

	ValidLocalchainTransfers::mutate(|a| a.insert((Alice.to_account_id(), 1)));
	// NOTE: this is in sorted order by account_id, account_type
	let mut balance_tips = BTreeMap::from([
		(
			(Alice.to_account_id(), AccountType::Deposit),
			BalanceTip {
				account_id: Alice.to_account_id(),
				account_type: AccountType::Deposit,
				balance: 0,
				change_number: 1,
				account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
				channel_hold_note: None,
			},
		),
		(
			(Bob.to_account_id(), AccountType::Deposit),
			BalanceTip {
				account_id: Bob.to_account_id(),
				account_type: AccountType::Deposit,
				balance: 800,
				change_number: 1,
				account_origin: AccountOrigin { notebook_number: 1, account_uid: 2 },
				channel_hold_note: None,
			},
		),
		(
			(Bob.to_account_id(), AccountType::Tax),
			BalanceTip {
				account_id: Bob.to_account_id(),
				account_type: AccountType::Tax,
				balance: 200,
				change_number: 1,
				account_origin: AccountOrigin { notebook_number: 1, account_uid: 3 },
				channel_hold_note: None,
			},
		),
	]);

	let mut notebook = Notebook {
		header: NotebookHeader {
			version: 1,
			notary_id: 1,
			notebook_number: 1,
			finalized_block_number: 100,
			pinned_to_block_number: 0,
			start_time: Utc::now().timestamp_millis() as u64 - 60_000,
			changed_accounts_root: merkle_root::<Blake2Hasher, _>(
				balance_tips.iter().map(|(_, v)| v.encode()).collect::<Vec<_>>(),
			),
			chain_transfers: bounded_vec![ChainTransfer::ToLocalchain {
				account_id: Alice.to_account_id(),
				account_nonce: 1,
			}],
			changed_account_origins: bounded_vec![
				AccountOrigin { notebook_number: 1, account_uid: 1 },
				AccountOrigin { notebook_number: 1, account_uid: 2 },
				AccountOrigin { notebook_number: 1, account_uid: 3 }
			],
			end_time: Utc::now().timestamp_millis() as u64,
		},
		balance_changes: bounded_vec![BoundedVec::truncate_from(alice_balance_changeset),],
		new_account_origins: bounded_vec![
			NewAccountOrigin::new(Alice.to_account_id(), AccountType::Deposit, 1),
			NewAccountOrigin::new(Bob.to_account_id(), AccountType::Deposit, 2),
			NewAccountOrigin::new(Bob.to_account_id(), AccountType::Tax, 3)
		],
	};

	assert_ok!(notebook_verify::<TestLookup>(&notebook.header.hash(), &notebook),);

	let changeset2 = vec![
		BalanceChange {
			balance: 0,
			change_number: 2,
			account_id: Bob.to_account_id(),
			account_type: AccountType::Deposit,
			previous_balance_proof: None,
			channel_hold_note: None,
			notes: bounded_vec![Note::create(800, NoteType::Send { to: None }),],
			signature: empty_signature(),
		}
		.sign(Bob.pair())
		.clone(),
		BalanceChange {
			balance: 600,
			change_number: 2,
			account_id: Alice.to_account_id(),
			account_type: AccountType::Deposit,
			previous_balance_proof: None,
			channel_hold_note: None,
			notes: bounded_vec![
				Note::create(800, NoteType::Claim),
				Note::create(200, NoteType::Tax),
			],
			signature: empty_signature(),
		}
		.sign(Alice.pair())
		.clone(),
		BalanceChange {
			balance: 200,
			change_number: 1,
			account_id: Alice.to_account_id(),
			account_type: AccountType::Tax,
			previous_balance_proof: None,
			channel_hold_note: None,
			notes: bounded_vec![Note::create(200, NoteType::Claim),],
			signature: empty_signature(),
		}
		.sign(Alice.pair())
		.clone(),
	];
	notebook.header.changed_accounts_root = merkle_root::<Blake2Hasher, _>(
		balance_tips.iter().map(|(_, v)| v.encode()).collect::<Vec<_>>(),
	);
	notebook
		.balance_changes
		.try_push(BoundedVec::truncate_from(changeset2))
		.expect("should insert");
	if let Some(tip) = balance_tips.get_mut(&(Bob.to_account_id(), AccountType::Deposit)) {
		tip.change_number = 2;
		tip.balance = 0;
	}
	if let Some(tip) = balance_tips.get_mut(&(Alice.to_account_id(), AccountType::Deposit)) {
		tip.change_number = 2;
		tip.balance = 600;
	}
	balance_tips.insert(
		(Alice.to_account_id(), AccountType::Tax),
		BalanceTip {
			balance: 200,
			change_number: 1,
			account_id: Alice.to_account_id(),
			account_type: AccountType::Tax,
			account_origin: AccountOrigin { notebook_number: 1, account_uid: 4 },
			channel_hold_note: None,
		},
	);
	notebook
		.new_account_origins
		.try_push(NewAccountOrigin::new(Alice.to_account_id(), AccountType::Tax, 4))
		.expect("should insert");
	assert_err!(
		notebook_verify::<TestLookup>(&notebook.header.hash(), &notebook),
		VerifyError::MissingBalanceProof
	);
	notebook.header.changed_accounts_root = merkle_root::<Blake2Hasher, _>(
		balance_tips.iter().map(|(_, v)| v.encode()).collect::<Vec<_>>(),
	);
	notebook.balance_changes[1][0].previous_balance_proof = Some(BalanceProof {
		notary_id: 1,
		notebook_number: 1,
		notebook_proof: None,
		account_origin: AccountOrigin { notebook_number: 1, account_uid: 2 },
		balance: 800,
	});
	notebook.balance_changes[1][1].previous_balance_proof = Some(BalanceProof {
		notary_id: 1,
		notebook_number: 1,
		notebook_proof: None,
		account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
		balance: 0,
	});
	notebook.balance_changes[1][2].previous_balance_proof = Some(BalanceProof {
		notary_id: 1,
		notebook_number: 1,
		notebook_proof: None,
		account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
		balance: 0,
	});
	assert_err!(
		notebook_verify::<TestLookup>(&notebook.header.hash(), &notebook),
		VerifyError::InvalidPreviousBalanceProof
	);
	notebook
		.header
		.changed_account_origins
		.try_push(AccountOrigin { notebook_number: 1, account_uid: 4 })
		.expect("should insert");

	notebook.balance_changes[1][2].previous_balance_proof = None;
	assert_ok!(notebook_verify::<TestLookup>(&notebook.header.hash(), &notebook),);
}

#[test]
fn test_cannot_remove_lock_between_changesets_in_a_notebook() {
	let alice_balance_changeset = vec![BalanceChange {
		balance: 1000,
		change_number: 1,
		account_id: Alice.to_account_id(),
		account_type: AccountType::Deposit,
		previous_balance_proof: None,
		channel_hold_note: None,
		notes: bounded_vec![Note::create(1000, NoteType::ClaimFromMainchain { account_nonce: 1 }),],
		signature: empty_signature(),
	}
	.sign(Alice.pair())
	.clone()];
	let alice_balance_changeset2 = vec![BalanceChange {
		balance: 1000,
		change_number: 2,
		account_id: Alice.to_account_id(),
		account_type: AccountType::Deposit,
		previous_balance_proof: Some(BalanceProof {
			notary_id: 1,
			notebook_number: 1,
			notebook_proof: None,
			account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
			balance: 1000,
		}),
		channel_hold_note: None,
		notes: bounded_vec![Note::create(
			1000,
			NoteType::ChannelHold { recipient: Bob.to_account_id() }
		)],
		signature: empty_signature(),
	}
	.sign(Alice.pair())
	.clone()];

	ValidLocalchainTransfers::mutate(|a| a.insert((Alice.to_account_id(), 1)));
	let mut notebook = Notebook {
		header: NotebookHeader {
			version: 1,
			notary_id: 1,
			notebook_number: 1,
			finalized_block_number: 100,
			pinned_to_block_number: 0,
			start_time: Utc::now().timestamp_millis() as u64 - 60_000,
			changed_accounts_root: merkle_root::<Blake2Hasher, _>(vec![BalanceTip {
				account_id: Alice.to_account_id(),
				account_type: AccountType::Deposit,
				balance: 1000,
				change_number: 2,
				account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
				channel_hold_note: None,
			}
			.encode()]),
			chain_transfers: bounded_vec![ChainTransfer::ToLocalchain {
				account_id: Alice.to_account_id(),
				account_nonce: 1,
			}],
			changed_account_origins: bounded_vec![AccountOrigin {
				notebook_number: 1,
				account_uid: 1
			}],
			end_time: Utc::now().timestamp_millis() as u64,
		},
		balance_changes: bounded_vec![
			BoundedVec::truncate_from(alice_balance_changeset),
			BoundedVec::truncate_from(alice_balance_changeset2),
		],
		new_account_origins: bounded_vec![NewAccountOrigin::new(
			Alice.to_account_id(),
			AccountType::Deposit,
			1
		)],
	};

	// test that the change root records the hold note
	assert_err!(
		notebook_verify::<TestLookup>(&notebook.header.hash(), &notebook),
		VerifyError::InvalidBalanceChangeRoot
	);

	let hold_note = notebook.balance_changes[1][0].notes[0].clone();

	notebook.header.changed_accounts_root = merkle_root::<Blake2Hasher, _>(vec![BalanceTip {
		account_id: Alice.to_account_id(),
		account_type: AccountType::Deposit,
		balance: 1000,
		change_number: 2,
		account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
		channel_hold_note: Some(hold_note),
	}
	.encode()]);
	assert_ok!(notebook_verify::<TestLookup>(&notebook.header.hash(), &notebook),);

	// now confirm we can't remove the hold in the same set of changes
	{
		// Try 1: pretend it didn't happen
		let alice_balance_changeset3 = vec![BalanceChange {
			balance: 1000,
			change_number: 3,
			account_id: Alice.to_account_id(),
			account_type: AccountType::Deposit,
			previous_balance_proof: Some(BalanceProof {
				notary_id: 1,
				notebook_number: 1,
				notebook_proof: None,
				account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
				balance: 1000,
			}),
			channel_hold_note: None,
			notes: bounded_vec![Note::create(
				1000,
				NoteType::ChannelHold { recipient: Ferdie.to_account_id() }
			)],
			signature: empty_signature(),
		}
		.sign(Alice.pair())
		.clone()];
		let mut notebook = notebook.clone();
		let _ = notebook
			.balance_changes
			.try_push(BoundedVec::truncate_from(alice_balance_changeset3));
		let hold_note = notebook.balance_changes[2][0].notes[0].clone();
		notebook.header.changed_accounts_root = merkle_root::<Blake2Hasher, _>(vec![BalanceTip {
			account_id: Alice.to_account_id(),
			account_type: AccountType::Deposit,
			balance: 1000,
			change_number: 3,
			account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
			channel_hold_note: Some(hold_note),
		}
		.encode()]);
		assert_err!(
			notebook_verify::<TestLookup>(&notebook.header.hash(), &notebook),
			VerifyError::InvalidChannelHoldNote
		);
	}
	{
		// Try 2: try to remove the hold
		let alice_balance_changeset3 = vec![BalanceChange {
			balance: 1000,
			change_number: 3,
			account_id: Alice.to_account_id(),
			account_type: AccountType::Deposit,
			previous_balance_proof: Some(BalanceProof {
				notary_id: 1,
				notebook_number: 1,
				notebook_proof: None,
				account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
				balance: 1000,
			}),
			channel_hold_note: Some(Note::create(
				1000,
				NoteType::ChannelHold { recipient: Bob.to_account_id() },
			)),
			notes: bounded_vec![Note::create(0, NoteType::ChannelSettle)],
			signature: empty_signature(),
		}
		.sign(Alice.pair())
		.clone()];

		let mut notebook = notebook.clone();
		let _ = notebook
			.balance_changes
			.try_push(BoundedVec::truncate_from(alice_balance_changeset3));
		let hold_note = notebook.balance_changes[2][0].notes[0].clone();

		notebook.header.changed_accounts_root = merkle_root::<Blake2Hasher, _>(vec![BalanceTip {
			account_id: Alice.to_account_id(),
			account_type: AccountType::Deposit,
			balance: 1000,
			change_number: 3,
			account_origin: AccountOrigin { notebook_number: 1, account_uid: 1 },
			channel_hold_note: Some(hold_note),
		}
		.encode()]);
		assert_err!(
			notebook_verify::<TestLookup>(&notebook.header.hash(), &notebook),
			VerifyError::ChannelHoldNotReadyForClaim
		);
	}
}
