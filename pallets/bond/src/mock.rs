use env_logger::{Builder, Env};
use frame_support::{parameter_types, traits::Currency};
use frame_support::derive_impl;
use sp_core::{ConstU32};
use sp_runtime::BuildStorage;

use crate as pallet_bond;

pub type Balance = u128;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Bonds: pallet_bond
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

parameter_types! {

	pub static ExistentialDeposit: Balance = 10;
	pub const MinimumBondAmount:u128 = 1_000;
	pub const BlocksPerYear:u32 = 1440*365;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<0>;
	type MaxReserves = ConstU32<0>;
	type ReserveIdentifier = ();
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
}

pub fn set_argons(account_id: u64, amount: Balance) {
	let _ = Balances::make_free_balance_be(&account_id, amount);
	drop(Balances::issue(amount));
}

impl pallet_bond::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Balance = Balance;
	type BondFundId = u32;
	type BondId = u64;
	type BlocksPerYear = BlocksPerYear;
	type MinimumBondAmount = MinimumBondAmount;
	type MaxConcurrentlyExpiringBonds = ConstU32<10>;
	type MaxConcurrentlyExpiringBondFunds = ConstU32<10>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let env = Env::new().default_filter_or("debug");
	let _ = Builder::from_env(env).is_test(true).try_init();
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
