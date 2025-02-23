use std::collections::BTreeMap;

use bitcoin::PublicKey;
use env_logger::{Builder, Env};
use frame_support::{derive_impl, parameter_types, traits::Currency};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_arithmetic::{FixedI128, FixedU128};
use sp_core::{ConstU32, ConstU64, H256};
use sp_runtime::{BuildStorage, DispatchError, DispatchResult};

use crate as pallet_bond;
use crate::BitcoinVerifier;
use argon_bitcoin::UtxoUnlocker;
use argon_primitives::{
	bitcoin::{
		BitcoinCosignScriptPubkey, BitcoinHeight, BitcoinNetwork, BitcoinSignature, BitcoinXPub,
		CompressedBitcoinPubkey, NetworkKind, Satoshis, UtxoId, UtxoRef,
	},
	bond::{Bond, BondError, BondType, Vault, VaultArgons, VaultProvider},
	ensure, BitcoinUtxoTracker, PriceProvider, UtxoBondedEvents, VaultId,
};

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
	pub const MinimumBondAmount:u128 = 1_000_000;
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

parameter_types! {
	pub static MaxUnlockingUtxos: u32 = 10;
	pub static BitcoinPricePerUsd: Option<FixedU128> = Some(FixedU128::from_float(62000.00));
	pub static ArgonPricePerUsd: Option<FixedU128> = Some(FixedU128::from_float(1.00));
	pub static ArgonCPI: Option<argon_primitives::ArgonCPI> = Some(FixedI128::from_float(0.1));
	pub static UtxoUnlockCosignDeadlineBlocks: BitcoinHeight = 5;
	pub static BitcoinBondReclamationBlocks: BitcoinHeight = 30;
	pub static BitcoinBondDurationBlocks: BitcoinHeight = 365;
	pub static BitcoinBlockHeight: BitcoinHeight = 0;
	pub static MinimumBondSatoshis: Satoshis = 10_000_000;
	pub static DefaultVault: Vault<u64, Balance, BlockNumberFor<Test>> = Vault {
		mining_argons: VaultArgons {
			allocated: 100_000_000_000,
			bonded: 0,
			annual_percent_rate: FixedU128::from_float(10.0),
			base_fee: 0,
		},
		bitcoin_argons: VaultArgons {
			allocated: 200_000_000_000,
			bonded: 0,
			annual_percent_rate: FixedU128::from_float(10.0),
			base_fee: 0,
		},
		operator_account_id: 1,
		securitization_percent: FixedU128::from_float(0.0),
		mining_reward_sharing_percent_take: FixedU128::from_float(0.0),
		securitized_argons: 0,
		is_closed: false,
		pending_terms: None,
	};

	pub static NextUtxoId: UtxoId = 1;
	pub static WatchedUtxosById: BTreeMap<UtxoId, (BitcoinCosignScriptPubkey, Satoshis, BitcoinHeight)> = BTreeMap::new();

	pub static GetUtxoRef: Option<UtxoRef> = None;

	pub static LastBondEvent: Option<(UtxoId, u64, Balance)> = None;
	pub static LastUnlockEvent: Option<(UtxoId, bool, Balance)> = None;

	pub static GetBitcoinNetwork: BitcoinNetwork = BitcoinNetwork::Regtest;

	pub static DefaultVaultBitcoinPubkey: PublicKey = "02e3af28965693b9ce1228f9d468149b831d6a0540b25e8a9900f71372c11fb277".parse::<PublicKey>().unwrap();
	pub static DefaultVaultReclaimBitcoinPubkey: PublicKey = "026c468be64d22761c30cd2f12cbc7de255d592d7904b1bab07236897cc4c2e766".parse::<PublicKey>().unwrap();
}

pub struct EventHandler;
impl UtxoBondedEvents<u64, Balance> for EventHandler {
	fn utxo_bonded(
		utxo_id: UtxoId,
		account_id: &u64,
		amount: Balance,
	) -> Result<(), DispatchError> {
		LastBondEvent::set(Some((utxo_id, *account_id, amount)));
		Ok(())
	}
	fn utxo_unlocked(
		utxo_id: UtxoId,
		remove_pending_mints: bool,
		amount_burned: Balance,
	) -> DispatchResult {
		LastUnlockEvent::set(Some((utxo_id, remove_pending_mints, amount_burned)));
		Ok(())
	}
}

pub struct StaticPriceProvider;
impl PriceProvider<Balance> for StaticPriceProvider {
	fn get_argon_cpi() -> Option<argon_primitives::ArgonCPI> {
		ArgonCPI::get()
	}
	fn get_latest_argon_price_in_us_cents() -> Option<FixedU128> {
		ArgonPricePerUsd::get()
	}
	fn get_latest_btc_price_in_us_cents() -> Option<FixedU128> {
		BitcoinPricePerUsd::get()
	}
}

pub struct StaticVaultProvider;

impl VaultProvider for StaticVaultProvider {
	type Balance = Balance;
	type AccountId = u64;
	type BlockNumber = BlockNumberFor<Test>;

	fn get(vault_id: VaultId) -> Option<Vault<Self::AccountId, Self::Balance, Self::BlockNumber>> {
		if vault_id == 1 {
			Some(DefaultVault::get())
		} else {
			None
		}
	}

	fn compensate_lost_bitcoin(
		_bond: &Bond<Self::AccountId, Self::Balance, Self::BlockNumber>,
		market_rate: Self::Balance,
	) -> Result<Self::Balance, BondError> {
		DefaultVault::mutate(|a| {
			a.bitcoin_argons.destroy_bond_funds(market_rate).expect("should not fail");
		});
		Ok(market_rate)
	}

	fn burn_vault_bitcoin_funds(
		_bond: &Bond<Self::AccountId, Self::Balance, Self::BlockNumber>,
		amount_to_burn: Self::Balance,
	) -> Result<(), BondError> {
		DefaultVault::mutate(|a| {
			a.bitcoin_argons.destroy_bond_funds(amount_to_burn).expect("should not fail")
		});

		Ok(())
	}

	fn bond_funds(
		_vault_id: VaultId,
		amount: Self::Balance,
		bond_type: BondType,
		_blocks: Self::BlockNumber,
		_bond_account_id: &Self::AccountId,
	) -> Result<(Self::Balance, Self::Balance), BondError> {
		ensure!(
			DefaultVault::get().mut_argons(&bond_type).allocated >= amount,
			BondError::InsufficientVaultFunds
		);
		DefaultVault::mutate(|a| a.mut_argons(&bond_type).bonded += amount);
		Ok((0, 0))
	}

	fn release_bonded_funds(
		bond: &Bond<Self::AccountId, Self::Balance, Self::BlockNumber>,
	) -> Result<Self::Balance, BondError> {
		DefaultVault::mutate(|a| a.mut_argons(&bond.bond_type).reduce_bonded(bond.amount));
		Ok(bond.total_fee.saturating_sub(bond.prepaid_fee))
	}

	fn create_utxo_script_pubkey(
		_vault_id: VaultId,
		_utxo_id: UtxoId,
		_owner_pubkey: CompressedBitcoinPubkey,
		_vault_claim_height: BitcoinHeight,
		_open_claim_height: BitcoinHeight,
		_current_height: BitcoinHeight,
	) -> Result<(BitcoinXPub, BitcoinXPub, BitcoinCosignScriptPubkey), BondError> {
		Ok((
			BitcoinXPub {
				public_key: DefaultVaultBitcoinPubkey::get().into(),
				chain_code: [0; 32],
				depth: 0,
				parent_fingerprint: [0; 4],
				child_number: 0,
				network: NetworkKind::Test,
			},
			BitcoinXPub {
				public_key: DefaultVaultReclaimBitcoinPubkey::get().into(),
				chain_code: [0; 32],
				depth: 0,
				parent_fingerprint: [0; 4],
				child_number: 1,
				network: NetworkKind::Test,
			},
			BitcoinCosignScriptPubkey::P2WSH { wscript_hash: H256::from([0; 32]) },
		))
	}
}

pub struct StaticBitcoinVerifier;
impl BitcoinVerifier<Test> for StaticBitcoinVerifier {
	fn verify_signature(
		_utxo_unlocker: UtxoUnlocker,
		_pubkey: CompressedBitcoinPubkey,
		_signature: &BitcoinSignature,
	) -> Result<bool, DispatchError> {
		Ok(true)
	}
}

pub struct StaticBitcoinUtxoTracker;
impl BitcoinUtxoTracker for StaticBitcoinUtxoTracker {
	fn new_utxo_id() -> UtxoId {
		let id = NextUtxoId::get();
		NextUtxoId::set(id + 1);
		id
	}

	fn get(_utxo_id: UtxoId) -> Option<UtxoRef> {
		GetUtxoRef::get()
	}

	fn watch_for_utxo(
		utxo_id: UtxoId,
		script_pubkey: BitcoinCosignScriptPubkey,
		satoshis: Satoshis,
		watch_for_spent_until: BitcoinHeight,
	) -> Result<(), DispatchError> {
		WatchedUtxosById::mutate(|watched_utxos| {
			watched_utxos.insert(utxo_id, (script_pubkey, satoshis, watch_for_spent_until));
		});
		Ok(())
	}

	fn unwatch(utxo_id: UtxoId) {
		WatchedUtxosById::mutate(|watched_utxos| {
			watched_utxos.remove(&utxo_id);
		});
	}
}

impl pallet_bond::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Balance = Balance;
	type ArgonBlocksPerDay = ConstU64<1440>;
	type MinimumBondAmount = MinimumBondAmount;
	type MaxConcurrentlyExpiringBonds = ConstU32<10>;
	type BondEvents = EventHandler;
	type PriceProvider = StaticPriceProvider;
	type VaultProvider = StaticVaultProvider;
	type MaxUnlockingUtxos = MaxUnlockingUtxos;
	type UtxoUnlockCosignDeadlineBlocks = UtxoUnlockCosignDeadlineBlocks;
	type BitcoinUtxoTracker = StaticBitcoinUtxoTracker;
	type BitcoinBondReclamationBlocks = BitcoinBondReclamationBlocks;
	type BitcoinBondDurationBlocks = BitcoinBondDurationBlocks;
	type BitcoinBlockHeight = BitcoinBlockHeight;
	type BitcoinSignatureVerifier = StaticBitcoinVerifier;
	type GetBitcoinNetwork = GetBitcoinNetwork;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let env = Env::new().default_filter_or("debug");
	let _ = Builder::from_env(env).is_test(true).try_init();

	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_bond::GenesisConfig::<Test> {
		minimum_bitcoin_bond_satoshis: MinimumBondSatoshis::get(),
		_phantom: Default::default(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	sp_io::TestExternalities::new(t)
}
