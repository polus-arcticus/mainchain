use crate::{
	mock::{DataDomain as DataDomainPallet, *},
	pallet::{DomainPaymentAddressHistory, ExpiringDomainsByBlock, RegisteredDataDomains},
	DataDomainRegistration, Error, Event,
};
use frame_support::{assert_err, assert_ok, traits::Hooks};
use sp_keyring::AccountKeyring::{Alice, Bob};
use sp_runtime::{testing::H256, BoundedVec};
use std::{collections::BTreeMap, net::Ipv4Addr};
use ulx_primitives::{
	host::Host, notebook::NotebookHeader, tick::Tick, AccountId, DataDomain, DataDomainProvider,
	DataTLD, NotebookEventHandler, Semver, VersionHost, ZoneRecord,
};

#[test]
fn it_can_register_domains() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let domain = DataDomain {
			top_level_domain: DataTLD::Analytics,
			domain_name: BoundedVec::truncate_from(b"test".to_vec()),
		};
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			1,
			vec![(domain.clone(), Bob.to_account_id(),),]
		)));
		assert_eq!(
			RegisteredDataDomains::<Test>::get(&domain),
			Some(DataDomainRegistration { account_id: Bob.to_account_id(), registered_at_tick: 1 })
		);
		assert_eq!(ExpiringDomainsByBlock::<Test>::get(1001).len(), 1);
		System::assert_last_event(
			Event::DataDomainRegistered {
				domain: domain.clone(),
				registration: DataDomainRegistration {
					account_id: Bob.to_account_id(),
					registered_at_tick: 1,
				},
			}
			.into(),
		);
	});
}
#[test]
fn it_cancels_conflicting_domains() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let domain = DataDomain {
			top_level_domain: DataTLD::Analytics,
			domain_name: BoundedVec::truncate_from(b"test".to_vec()),
		};
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			1,
			vec![(domain.clone(), Bob.to_account_id(),), (domain.clone(), Alice.to_account_id(),),]
		)));
		assert_eq!(RegisteredDataDomains::<Test>::get(&domain), None);
		assert_eq!(ExpiringDomainsByBlock::<Test>::get(1001).len(), 0);
		System::assert_last_event(
			Event::DataDomainRegistrationCanceled {
				domain: domain.clone(),
				registration: DataDomainRegistration {
					account_id: Bob.to_account_id(),
					registered_at_tick: 1,
				},
			}
			.into(),
		);
	});
}
#[test]
fn it_renews_domains() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let domain = DataDomain {
			top_level_domain: DataTLD::Analytics,
			domain_name: BoundedVec::truncate_from(b"test".to_vec()),
		};
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			1,
			vec![(domain.clone(), Bob.to_account_id(),)]
		)));
		assert_eq!(ExpiringDomainsByBlock::<Test>::get(1001).len(), 1);

		System::set_block_number(100);
		CurrentTick::set(100);
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			100,
			vec![(domain.clone(), Bob.to_account_id(),)]
		)));
		assert_eq!(ExpiringDomainsByBlock::<Test>::get(1001).len(), 0);
		assert_eq!(ExpiringDomainsByBlock::<Test>::get(1100).len(), 1);
		System::assert_last_event(Event::DataDomainRenewed { domain: domain.clone() }.into());
	});
}
#[test]
fn it_ignores_duplicated_domains() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let domain = DataDomain {
			top_level_domain: DataTLD::Analytics,
			domain_name: BoundedVec::truncate_from(b"test".to_vec()),
		};
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			1,
			vec![(domain.clone(), Bob.to_account_id(),)]
		)));
		let registered_to_bob =
			DataDomainRegistration { account_id: Bob.to_account_id(), registered_at_tick: 1 };
		assert_eq!(RegisteredDataDomains::<Test>::get(&domain), Some(registered_to_bob.clone()));

		System::set_block_number(2);
		CurrentTick::set(2);
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			2,
			vec![(domain.clone(), Alice.to_account_id(),)]
		)));
		assert_eq!(RegisteredDataDomains::<Test>::get(&domain), Some(registered_to_bob));
	});
}
#[test]
fn it_registers_zone_records() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		CurrentTick::set(2);
		let domain = DataDomain {
			top_level_domain: DataTLD::Analytics,
			domain_name: BoundedVec::truncate_from(b"test".to_vec()),
		};
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			1,
			vec![(domain.clone(), Bob.to_account_id(),)]
		)));

		let zone = ZoneRecord {
			payment_account: Bob.to_account_id(),
			versions: BTreeMap::from([(
				Semver::new(1, 0, 0),
				VersionHost {
					host: Host {
						ip: Ipv4Addr::new(127, 0, 0, 1).into(),
						port: 8080,
						is_secure: true,
					},
					datastore_id: BoundedVec::truncate_from(b"test".to_vec()),
				},
			)]),
		};

		assert_ok!(DataDomainPallet::set_zone_record(
			RuntimeOrigin::signed(Bob.to_account_id()),
			domain.clone(),
			zone.clone()
		));
		assert_eq!(
			DomainPaymentAddressHistory::<Test>::get(&domain).to_vec(),
			vec![(Bob.to_account_id(), 2 as Tick)]
		);
		System::assert_last_event(
			Event::ZoneRecordUpdated { domain: domain.clone(), zone_record: zone.clone() }.into(),
		);
		assert_err!(
			DataDomainPallet::set_zone_record(
				RuntimeOrigin::signed(Alice.to_account_id()),
				domain.clone(),
				zone.clone()
			),
			Error::<Test>::NotDomainOwner
		);

		assert_err!(
			DataDomainPallet::set_zone_record(
				RuntimeOrigin::signed(Bob.to_account_id()),
				DataDomain {
					top_level_domain: DataTLD::Automotive,
					domain_name: BoundedVec::truncate_from(b"test".to_vec()),
				},
				zone.clone()
			),
			Error::<Test>::DomainNotRegistered
		);
	});
}

#[test]
fn it_tracks_historical_payment() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		CurrentTick::set(2);
		let domain = DataDomain {
			top_level_domain: DataTLD::Analytics,
			domain_name: BoundedVec::truncate_from(b"test".to_vec()),
		};
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			1,
			vec![(domain.clone(), Bob.to_account_id(),)]
		)));

		let mut zone =
			ZoneRecord { payment_account: Bob.to_account_id(), versions: BTreeMap::new() };

		assert_ok!(DataDomainPallet::set_zone_record(
			RuntimeOrigin::signed(Bob.to_account_id()),
			domain.clone(),
			zone.clone()
		));
		assert_eq!(
			DomainPaymentAddressHistory::<Test>::get(&domain).to_vec(),
			vec![(Bob.to_account_id(), 2 as Tick)]
		);
		CurrentTick::set(4);
		zone.payment_account = Alice.to_account_id();
		assert_ok!(DataDomainPallet::set_zone_record(
			RuntimeOrigin::signed(Bob.to_account_id()),
			domain.clone(),
			zone.clone()
		));
		assert_eq!(
			DomainPaymentAddressHistory::<Test>::get(&domain).to_vec(),
			vec![(Bob.to_account_id(), 2 as Tick), (Alice.to_account_id(), 4 as Tick)]
		);
		assert_eq!(
			DataDomainPallet::is_registered_payment_account(&domain, &Bob.to_account_id(), (2, 3)),
			true
		);
		assert_eq!(
			DataDomainPallet::is_registered_payment_account(&domain, &Bob.to_account_id(), (4, 5)),
			false
		);
		assert_eq!(
			DataDomainPallet::is_registered_payment_account(
				&domain,
				&Alice.to_account_id(),
				(4, 5)
			),
			true
		);
	});
}

#[test]
fn it_expires_domains() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let domain = DataDomain {
			top_level_domain: DataTLD::Analytics,
			domain_name: BoundedVec::truncate_from(b"test".to_vec()),
		};
		assert_ok!(DataDomainPallet::notebook_submitted(&create_notebook(
			1,
			vec![(domain.clone(), Bob.to_account_id(),)]
		)));

		System::set_block_number(1001);
		CurrentTick::set(1001);
		DataDomainPallet::on_initialize(1001);
		assert_eq!(RegisteredDataDomains::<Test>::get(&domain), None);
	});
}

fn create_notebook(tick: Tick, domains: Vec<(DataDomain, AccountId)>) -> NotebookHeader {
	NotebookHeader {
		version: 1,
		notary_id: 1,
		notebook_number: 1,
		tick,
		finalized_block_number: 1,
		changed_accounts_root: Default::default(),
		chain_transfers: Default::default(),
		changed_account_origins: Default::default(),
		tax: 0,
		// Block Votes
		parent_secret: None,
		secret_hash: H256::from_slice(&[0u8; 32]),
		block_voting_power: 1,
		block_votes_root: H256::from_slice(&[0u8; 32]),
		block_votes_count: 1,
		blocks_with_votes: Default::default(),
		data_domains: BoundedVec::truncate_from(domains),
	}
}
