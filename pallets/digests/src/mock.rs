use argon_notary_audit::VerifyError;
use env_logger::{Builder, Env};
use frame_support::{derive_impl, parameter_types};
use sp_runtime::{traits::IdentityLookup, BuildStorage};

use argon_primitives::tick::Tick;

use crate as pallet_digests;

pub(crate) type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Digests: pallet_digests
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = ();
}

parameter_types! {

	pub static DomainExpirationTicks :u32 = 1000;
	pub static NotebookTick: Tick = 0;
	pub static HistoricalPaymentAddressTicksToKeep: u32 = 100;
}

impl pallet_digests::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type NotebookVerifyError = VerifyError;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let env = Env::new().default_filter_or("debug");
	let _ = Builder::from_env(env).is_test(true).try_init();
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
