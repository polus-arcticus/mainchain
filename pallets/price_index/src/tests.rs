use std::time::SystemTime;

use frame_support::{assert_err, assert_ok};
use sp_arithmetic::{FixedI128, FixedU128};
use sp_runtime::traits::Zero;

use ulx_primitives::{bitcoin::SATOSHIS_PER_BITCOIN, ArgonCPI, Moment, PriceProvider};

use crate::{mock::*, Current, Operator, PriceIndex as PriceIndexEntry};

type Event = crate::Event<Test>;
type Error = crate::Error<Test>;

#[test]
fn should_require_an_operator_to_submit() {
	new_test_ext(None).execute_with(|| {
		System::set_block_number(1);
		assert_err!(
			PriceIndex::submit(RuntimeOrigin::signed(1), create_index()),
			Error::NotAuthorizedOperator
		);

		assert!(System::events().is_empty());
	});
}

#[test]
fn can_set_an_operator() {
	new_test_ext(None).execute_with(|| {
		System::set_block_number(1);
		assert_err!(
			PriceIndex::submit(RuntimeOrigin::signed(1), create_index()),
			Error::NotAuthorizedOperator
		);

		assert_ok!(PriceIndex::set_operator(RuntimeOrigin::root(), 1));

		assert_eq!(Operator::<Test>::get(), Some(1));
		System::assert_last_event(Event::OperatorChanged { operator_id: 1 }.into());
	});
}

#[test]
fn can_set_a_price_index() {
	new_test_ext(Some(1)).execute_with(|| {
		System::set_block_number(1);
		let entry = create_index();
		assert_ok!(PriceIndex::submit(RuntimeOrigin::signed(1), entry),);
		assert_eq!(Current::<Test>::get(), Some(entry));

		System::assert_last_event(Event::NewIndex.into());
	});
}

#[test]
fn uses_latest_as_current() {
	new_test_ext(Some(1)).execute_with(|| {
		System::set_block_number(1);

		let start = now();
		let mut entry = create_index();
		entry.timestamp = start;
		assert_ok!(PriceIndex::submit(RuntimeOrigin::signed(1), entry),);
		assert_eq!(Current::<Test>::get(), Some(entry));
		System::assert_last_event(Event::NewIndex.into());

		let mut entry2 = entry;
		entry2.argon_cpi = ArgonCPI::from_float(1.0);
		entry2.timestamp = start + 4;
		assert_ok!(PriceIndex::submit(RuntimeOrigin::signed(1), entry2),);
		assert_eq!(Current::<Test>::get(), Some(entry2));

		let mut entry_backwards = entry;
		entry_backwards.argon_cpi = ArgonCPI::from_float(2.0);
		entry_backwards.timestamp = start + 1;
		assert_ok!(PriceIndex::submit(RuntimeOrigin::signed(1), entry_backwards),);
		assert_eq!(Current::<Test>::get(), Some(entry2));
	});
}

#[test]
fn doesnt_use_expired_values() {
	new_test_ext(Some(1)).execute_with(|| {
		System::set_block_number(1);
		OldestHistoryToKeep::set(10);
		let mut entry = create_index();
		entry.timestamp = now() - 11;
		assert_err!(PriceIndex::submit(RuntimeOrigin::signed(1), entry), Error::PricesTooOld);
		assert_eq!(Current::<Test>::get(), None);
	});
}

#[test]
fn can_convert_argon_prices() {
	new_test_ext(Some(1)).execute_with(|| {
		System::set_block_number(1);
		let mut index = PriceIndexEntry {
			timestamp: now(),
			btc_usd_price: FixedU128::from_float(62_000.00), // 62,000.00
			argon_usd_price: FixedU128::from_float(1.00),    // 100 cents
			argon_cpi: ArgonCPI::zero(),
		};
		Current::<Test>::put(index);

		assert_eq!(
			<PriceIndex as PriceProvider<u128>>::get_bitcoin_argon_price(SATOSHIS_PER_BITCOIN),
			Some(62_000 * 1000),
			"price in milligons"
		);

		index.argon_usd_price = FixedU128::from_float(1.01);
		Current::<Test>::put(index);

		assert_eq!(
			<PriceIndex as PriceProvider<u128>>::get_bitcoin_argon_price(SATOSHIS_PER_BITCOIN),
			Some(1000 * (62_000 * 100) / 101),
		);
	});
}

fn create_index() -> PriceIndexEntry<u64> {
	PriceIndexEntry {
		timestamp: now(),
		btc_usd_price: FixedU128::from_float(62_000.00),
		argon_usd_price: FixedU128::from_float(1_000.00),
		argon_cpi: FixedI128::from_float(0.0),
	}
}

fn now() -> Moment {
	SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as Moment
}
