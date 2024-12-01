mod fees;

use super::{
	Balances, BitcoinUtxos, Block, BlockSeal, BlockSealSpec, Bonds, ChainTransfer, Digests,
	Domains, Grandpa, Hyperbridge, Ismp, MiningSlot, Mint, Notaries, Notebook, OriginCaller,
	Ownership, PalletInfo, PriceIndex, Runtime, RuntimeCall, RuntimeEvent, RuntimeFreezeReason,
	RuntimeHoldReason, RuntimeOrigin, RuntimeTask, System, Ticks, Timestamp, TxPause, Vaults,
	VERSION,
};
use crate::SessionKeys;
use alloc::{boxed::Box, vec::Vec};
use argon_primitives::{
	bitcoin::BitcoinHeight, notary::NotaryRecordWithState, prelude::*, BlockSealAuthorityId,
	HashOutput, Moment, TickProvider, CHANNEL_HOLD_CLAWBACK_TICKS,
};
pub use frame_support::{
	construct_runtime, derive_impl,
	pallet_prelude::*,
	parameter_types,
	traits::{
		fungible, fungible::Balanced, ConstBool, ConstU128, ConstU16, ConstU32, ConstU64, ConstU8,
		Contains, Currency, Everything, Imbalance, InsideBoth, InstanceFilter, KeyOwnerProofSystem,
		OnUnbalanced, Randomness, StorageInfo, StorageMapShim, TransformOrigin,
	},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
		},
		IdentityFee, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
	PalletId, StorageValue,
};
use ismp::{host::StateMachine, module::IsmpModule, router::IsmpRouter, Error};
use sp_consensus_grandpa::{AuthorityId as GrandpaId, AuthorityList};

use frame_system::EnsureRoot;
use pallet_bond::BitcoinVerifier;
use pallet_hyperbridge::PALLET_HYPERBRIDGE_ID;
use pallet_ismp::NoOpMmrTree;
use pallet_mining_slot::OnNewSlot;
use pallet_notebook::NotebookVerifyError;
use pallet_tx_pause::RuntimeCallNameOf;
use sp_arithmetic::{FixedU128, Perbill};
use sp_runtime::traits::BlakeTwo256;
use sp_version::RuntimeVersion;

pub type AccountData = pallet_balances::AccountData<Balance>;

/// TODO: adjust this to match measured
/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// TODO: measure this
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

pub const MAXIMUM_BLOCK_WEIGHT: Weight =
	Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(10), u64::MAX);

/// The existential deposit.
pub const EXISTENTIAL_DEPOSIT: Balance = 500_000;

pub const ARGON: Balance = 1_000_000;
pub const CENTS: Balance = ARGON / 100_000;
pub const MILLIGONS: Balance = 1_000;
pub const MICROGONS: Balance = 1;

parameter_types! {
	pub const BlockHashCount: BlockNumber = 4096;
	pub const Version: RuntimeVersion = VERSION;
	/// We allow for 60 seconds of compute with a 10 second average block time.
	pub BlockWeights:  frame_system::limits::BlockWeights =  frame_system::limits::BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = InsideBoth<BaseCallFilter, TxPause>;
	/// The block type for the runtime.
	type Block = Block;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The type for storing how many extrinsics an account has signed.
	type Nonce = Nonce;
	/// The type for hashing blocks and tries.
	type Hash = HashOutput;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = ConstU16<{ argon_primitives::ADDRESS_PREFIX }>;
	type MaxConsumers = ConstU32<16>;
}

/// Calls that cannot be paused by the tx-pause pallet.
pub struct TxPauseWhitelistedCalls;

impl Contains<RuntimeCallNameOf<Runtime>> for TxPauseWhitelistedCalls {
	fn contains(full_name: &RuntimeCallNameOf<Runtime>) -> bool {
		#[allow(clippy::match_like_matches_macro)]
		match (full_name.0.as_slice(), full_name.1.as_slice()) {
			(b"System", _) => true,
			_ => false,
		}
	}
}

/// This pallet is intended to be used as a shortterm security measure.
impl pallet_tx_pause::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PauseOrigin = EnsureRoot<AccountId>;
	type UnpauseOrigin = EnsureRoot<AccountId>;
	type WhitelistedCalls = TxPauseWhitelistedCalls;
	type MaxNameLen = ConstU32<256>;
	type WeightInfo = pallet_tx_pause::weights::SubstrateWeight<Runtime>;
}

pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		// placeholder for filter
		match call {
			RuntimeCall::System(..) => true,
			_ => true,
		}
	}
}

parameter_types! {
	pub const TargetComputeBlockPercent: FixedU128 = FixedU128::from_rational(75, 100); // aim for less than full compute time so it can wait for notebooks
	pub const TargetBlockVotes: u32 = 50_000;
	pub const SealSpecVoteHistoryForAverage: u32 = 24 * 60; // 24 hours of history
	pub const SealSpecComputeHistoryToTrack: u32 = 6 * 60; // 6 hours of history
	pub const SealSpecComputeDifficultyChangePeriod: u32 = 60; // change difficulty every hour

	pub const DefaultChannelHoldDuration: Tick = 60;
	pub const HistoricalPaymentAddressTicksToKeep: Tick = DefaultChannelHoldDuration::get() + CHANNEL_HOLD_CLAWBACK_TICKS + 10;

	pub const ArgonsPerBlock: u32 = 5_000_000;
	pub const StartingOwnershipTokensPerBlock: u32 = 5_000_000;
	pub const HalvingBlocks: u32 = 2_100_000; // based on bitcoin, but 10x since we're 1 block per minute
	pub const MaturationBlocks: u32 = 5;
	pub const MinerPayoutPercent: FixedU128 = FixedU128::from_rational(75, 100);
	pub const DomainExpirationTicks: Tick = 60 * 24 * 365; // 1 year
}

impl pallet_block_seal_spec::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type TargetComputeBlockPercent = TargetComputeBlockPercent;
	type AuthorityProvider = MiningSlot;
	type MaxActiveNotaries = MaxActiveNotaries;
	type NotebookProvider = Notebook;
	type TickProvider = Ticks;
	type WeightInfo = pallet_block_seal_spec::weights::SubstrateWeight<Runtime>;
	type TargetBlockVotes = TargetBlockVotes;
	type HistoricalVoteBlocksForAverage = SealSpecVoteHistoryForAverage;
	type HistoricalComputeBlocksForAverage = SealSpecComputeHistoryToTrack;
	type ComputeDifficultyChangePeriod = SealSpecComputeDifficultyChangePeriod;
	type SealInherent = BlockSeal;
}

pub struct NotebookTickProvider;
impl Get<Tick> for NotebookTickProvider {
	fn get() -> Tick {
		let schedule = Ticks::voting_schedule();
		schedule.notebook_tick()
	}
}
impl pallet_block_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_block_rewards::weights::SubstrateWeight<Runtime>;
	type ArgonCurrency = Balances;
	type OwnershipCurrency = Ownership;
	type Balance = Balance;
	type BlockSealerProvider = BlockSeal;
	type BlockRewardAccountsProvider = MiningSlot;
	type NotaryProvider = Notaries;
	type NotebookProvider = Notebook;
	type NotebookTick = NotebookTickProvider;
	type ArgonsPerBlock = ArgonsPerBlock;
	type StartingOwnershipTokensPerBlock = StartingOwnershipTokensPerBlock;
	type HalvingBlocks = HalvingBlocks;
	type MinerPayoutPercent = MinerPayoutPercent;
	type MaturationBlocks = MaturationBlocks;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type EventHandler = Mint;
}

impl pallet_domains::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_domains::weights::SubstrateWeight<Runtime>;
	type DomainExpirationTicks = DomainExpirationTicks;
	type NotebookTick = NotebookTickProvider;
	type HistoricalPaymentAddressTicksToKeep = HistoricalPaymentAddressTicksToKeep;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = Digests;
	type EventHandler = ();
}

impl pallet_digests::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_digests::weights::SubstrateWeight<Runtime>;
	type NotebookVerifyError = NotebookVerifyError;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = (BlockSealSpec, Ticks);
	type MinimumPeriod = ConstU64<1000>;
	type WeightInfo = ();
}

pub struct MultiBlockPerTickEnabled;
impl Get<bool> for MultiBlockPerTickEnabled {
	fn get() -> bool {
		!MiningSlot::is_registered_mining_active()
	}
}

impl pallet_ticks::Config for Runtime {
	type WeightInfo = ();
	type AllowMultipleBlockPerTick = MultiBlockPerTickEnabled;
}

parameter_types! {
	pub const MaxCohortSize: u32 = 1_000; // this means mining_slots last 10 days
	pub const MaxMiners: u32 = 10_000; // must multiply cleanly by MaxCohortSize
	pub const OwnershipPercentAdjustmentDamper: FixedU128 = FixedU128::from_rational(20, 100);
	pub const TargetBidsPerSlot: u32 = 1_200; // 20% extra bids

	pub const MaxConcurrentlyExpiringBonds: u32 = 1_000;
	pub const MinimumBondAmount: u128 = 1_000_000;
	pub const BlocksPerDay: u32 = 1440;
	pub const BlocksPerYear: u32 = 1440 * 365;

	const BitcoinBlocksPerDay: BitcoinHeight = 6 * 24;
	pub const BitcoinBondDurationBlocks: BitcoinHeight = BitcoinBlocksPerDay::get() * 365; // 1 year
	pub const BitcoinBondReclamationBlocks: BitcoinHeight = BitcoinBlocksPerDay::get() * 30; // 30 days
	pub const UtxoUnlockCosignDeadlineBlocks: BitcoinHeight = BitcoinBlocksPerDay::get() * 5; // 5 days

	pub const MaxSetIdSessionEntries: u32 = 2u32;

	pub const MaxUnlockingUtxos: u32 = 1000;
	pub const MaxPendingTermModificationsPerBlock: u32 = 100;
	pub const MinTermsModificationBlockDelay: u32 = 1439; // must be at least one slot (day)
}

impl pallet_vaults::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_vaults::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
	type Balance = Balance;
	type RuntimeHoldReason = RuntimeHoldReason;
	type MinimumBondAmount = MinimumBondAmount;
	type BlocksPerDay = BlocksPerDay;
	type MaxPendingTermModificationsPerBlock = MaxPendingTermModificationsPerBlock;
	type MinTermsModificationBlockDelay = MinTermsModificationBlockDelay;
	type MiningSlotProvider = MiningSlot;
	type GetBitcoinNetwork = BitcoinUtxos;
}

pub struct BitcoinSignatureVerifier;
impl BitcoinVerifier<Runtime> for BitcoinSignatureVerifier {}
impl pallet_bond::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_bond::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
	type Balance = Balance;
	type RuntimeHoldReason = RuntimeHoldReason;
	type BondEvents = Mint;
	type BitcoinUtxoTracker = BitcoinUtxos;
	type PriceProvider = PriceIndex;
	type BitcoinSignatureVerifier = BitcoinSignatureVerifier;
	type BitcoinBlockHeight = BitcoinUtxos;
	type GetBitcoinNetwork = BitcoinUtxos;
	type VaultProvider = Vaults;
	type MinimumBondAmount = MinimumBondAmount;
	type ArgonBlocksPerDay = BlocksPerDay;
	type MaxUnlockingUtxos = MaxUnlockingUtxos;
	type MaxConcurrentlyExpiringBonds = MaxConcurrentlyExpiringBonds;
	type BitcoinBondDurationBlocks = BitcoinBondDurationBlocks;
	type BitcoinBondReclamationBlocks = BitcoinBondReclamationBlocks;
	type UtxoUnlockCosignDeadlineBlocks = UtxoUnlockCosignDeadlineBlocks;
}

pub struct GrandpaSlotRotation;

impl OnNewSlot<AccountId> for GrandpaSlotRotation {
	type Key = GrandpaId;
	fn on_new_slot(
		removed_authorities: Vec<(&AccountId, Self::Key)>,
		added_authorities: Vec<(&AccountId, Self::Key)>,
	) {
		if removed_authorities.is_empty() && added_authorities.is_empty() {
			return;
		}
		let mut next_authorities: AuthorityList = Grandpa::grandpa_authorities();
		for (_, authority_id) in removed_authorities {
			if let Some(index) = next_authorities.iter().position(|x| x.0 == authority_id) {
				next_authorities.remove(index);
			}
		}
		for (_, authority_id) in added_authorities {
			next_authorities.push((authority_id, 1));
		}

		// TODO: we need to be able to run multiple grandpas on a single miner before activating
		// 	changing the authorities. We want to activate a trailing 3 hours of miners who closed
		// blocks 	to activate a more decentralized grandpa process
		//
		// if let Err(err) = Grandpa::schedule_change(next_authorities, Zero::zero(), None) {
		// 	log::error!("Failed to schedule grandpa change: {:?}", err);
		// }
	}
}

impl pallet_mining_slot::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_mining_slot::weights::SubstrateWeight<Runtime>;
	type MaxMiners = MaxMiners;
	type MaxCohortSize = MaxCohortSize;
	type MinimumBondAmount = ConstU128<EXISTENTIAL_DEPOSIT>;
	type OwnershipPercentAdjustmentDamper = OwnershipPercentAdjustmentDamper;
	type TargetBidsPerSlot = TargetBidsPerSlot;
	type Balance = Balance;
	type OwnershipCurrency = Ownership;
	type RuntimeHoldReason = RuntimeHoldReason;
	type BondProvider = Bonds;
	type SlotEvents = (GrandpaSlotRotation,);
	type MiningAuthorityId = BlockSealAuthorityId;
	type Keys = SessionKeys;
}

impl pallet_block_seal::Config for Runtime {
	type AuthorityId = BlockSealAuthorityId;
	type WeightInfo = pallet_block_seal::weights::SubstrateWeight<Runtime>;
	type AuthorityProvider = MiningSlot;
	type NotebookProvider = Notebook;
	type BlockSealSpecProvider = BlockSealSpec;
	type TickProvider = Ticks;
	type EventHandler = MiningSlot;
	type Digests = Digests;
	type FindAuthor = Digests;
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxAuthorities = MaxMiners;
	type MaxNominators = ConstU32<0>;
	type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
	type KeyOwnerProof = sp_core::Void;
	type EquivocationReportSystem = ();
}

parameter_types! {
	pub const ChainTransferPalletId: PalletId = PalletId(*b"transfer");

	/// How long a transfer should remain in storage before returning.
	pub const TransferExpirationTicks: u32 = 1400 * 10;

	/// How many transfers out can be queued per block
	pub const MaxPendingTransfersOutPerBlock: u32 = 1000;
}

impl pallet_chain_transfer::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_chain_transfer::weights::SubstrateWeight<Runtime>;
	type Argon = Balances;
	type OwnershipTokens = Ownership;
	type Dispatcher = Hyperbridge;
	type Balance = Balance;
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type NotebookProvider = Notebook;
	type NotebookTick = NotebookTickProvider;
	type EventHandler = Mint;
	type PalletId = ChainTransferPalletId;
	type TransferExpirationTicks = TransferExpirationTicks;
	type MaxPendingTransfersOutPerBlock = MaxPendingTransfersOutPerBlock;
}

impl pallet_notebook::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_notebook::weights::SubstrateWeight<Runtime>;
	type EventHandler = (ChainTransfer, BlockSealSpec, Domains);
	type NotaryProvider = Notaries;
	type ChainTransferLookup = ChainTransfer;
	type BlockSealSpecProvider = BlockSealSpec;
	type TickProvider = Ticks;
	type Digests = Digests;
}

parameter_types! {
	pub const MaxActiveNotaries: u32 = 25; // arbitrarily set
	pub const MaxProposalHoldBlocks: u32 = 1440 * 14; // 2 weeks to approve
	pub const MaxProposalsPerBlock: u32 = 10;
	pub const MetaChangesTickDelay: u64 = 6; // delay pubkey changes for minimum of an hour
	pub const MaxTicksForKeyHistory: u32 = 1440 * 2; // keep for 2 days.. only used for notebook submission
	/// Max host ips a notary can provide
	pub const MaxNotaryHosts: u32 = 4;
}

pub type NotaryRecordT =
	NotaryRecordWithState<AccountId, BlockNumber, MaxNotaryHosts, NotebookVerifyError>;

impl pallet_notaries::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_notaries::weights::SubstrateWeight<Runtime>;
	type MaxActiveNotaries = MaxActiveNotaries;
	type MaxProposalHoldBlocks = MaxProposalHoldBlocks;
	type MaxProposalsPerBlock = MaxProposalsPerBlock;
	type MetaChangesTickDelay = MetaChangesTickDelay;
	type MaxTicksForKeyHistory = MaxTicksForKeyHistory;
	type MaxNotaryHosts = MaxNotaryHosts;
	type TickProvider = Ticks;
}
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub type ArgonToken = pallet_balances::Instance1;
impl pallet_balances::Config<ArgonToken> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type MaxFreezes = ConstU32<2>;
}

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 100 * CENTS + (bytes as Balance) * 5 * MICROGONS
}
parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = deposit(1, 40);
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = deposit(0, 33);
	// One storage item; key size 32, value size 16
	pub const AnnouncementDepositBase: Balance = deposit(1, 48);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
	pub const MaxProxies: u16 = 32;
	pub const MaxPending: u16 = 32;
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	TypeInfo,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	PriceIndex,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(
				c,
				RuntimeCall::Balances(..) |
					RuntimeCall::Ownership(..) |
					RuntimeCall::ChainTransfer(..)
			),
			ProxyType::PriceIndex => matches!(c, RuntimeCall::PriceIndex(..)),
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = deposit(1, 88);
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = deposit(0, 32);
	pub const MaxSignatories: u32 = 100;
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}
impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = MaxProxies;
	type WeightInfo = pallet_proxy::weights::SubstrateWeight<Runtime>;
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}

parameter_types! {
	pub const BitcoinBondDuration: u32 = 60 * 24 * 365; // 1 year
	pub const MaxPendingMintUtxos: u32 = 10_000;
	pub const MaxTrackedUtxos: u32 = 1_000_000_000;

	pub const MaxDowntimeTicksBeforeReset: Tick = 60; // 1 hour
	pub const MaxHistoryToKeep: u32 = 24 * 60; // 1 day worth of prices
	pub const MaxPriceAgeInTicks: Tick = 24 * 60; // 1 day
	pub const MaxArgonChangePerTickAwayFromTarget: FixedU128 = FixedU128::from_rational(1, 100); // 1 centagon
	pub const MaxArgonTargetChangePerTick: FixedU128 = FixedU128::from_rational(1, 100); // 1 centagon

	pub const MaxPendingConfirmationBlocks: BitcoinHeight = 10 * (6 * 24); // 10 days of bitcoin blocks

	pub const MaxPendingConfirmationUtxos: u32 = 10_000;
	pub const MaxBitcoinBirthBlocksOld: BitcoinHeight = 10 * (6 * 24); // 10 days of bitcoin blocks
}

impl pallet_price_index::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = pallet_price_index::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type MaxDowntimeTicksBeforeReset = MaxDowntimeTicksBeforeReset;
	type MaxPriceAgeInTicks = MaxPriceAgeInTicks;
	type CurrentTick = Ticks;
	type MaxArgonChangePerTickAwayFromTarget = MaxArgonChangePerTickAwayFromTarget;
	type MaxArgonTargetChangePerTick = MaxArgonTargetChangePerTick;
}

impl pallet_bitcoin_utxos::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_bitcoin_utxos::weights::SubstrateWeight<Runtime>;
	type EventHandler = Bonds;
	type MaxPendingConfirmationUtxos = MaxPendingConfirmationUtxos;
	type MaxPendingConfirmationBlocks = MaxPendingConfirmationBlocks;
}

impl pallet_mint::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_mint::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
	type PriceProvider = PriceIndex;
	type Balance = Balance;
	type MaxPendingMintUtxos = MaxPendingMintUtxos;
	type BlockRewardAccountsProvider = MiningSlot;
}

type OwnershipToken = pallet_balances::Instance2;
impl pallet_balances::Config<OwnershipToken> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	/// The type for recording an account's balance.
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = StorageMapShim<
		pallet_balances::Account<Runtime, OwnershipToken>,
		AccountId,
		pallet_balances::AccountData<Balance>,
	>;

	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type MaxFreezes = ConstU32<2>;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	// The host state machine of this pallet
	pub const HostStateMachine: StateMachine = StateMachine::Substrate(*b"argn");
}

#[cfg(not(feature = "canary"))]
parameter_types! {
	// The hyperbridge parachain on Polkadot
	pub const Coprocessor: Option<StateMachine> = Some(StateMachine::Polkadot(pallet_chain_transfer::ISMP_POLKADOT_PARACHAIN_ID));
}

#[cfg(feature = "canary")]
parameter_types! {
	pub const Coprocessor: Option<StateMachine> = Some(StateMachine::Kusama(pallet_chain_transfer::ISMP_PASEO_PARACHAIN_ID));
}

impl pallet_ismp::Config for Runtime {
	// configure the runtime event
	type RuntimeEvent = RuntimeEvent;
	// Permissioned origin who can create or update consensus clients
	type AdminOrigin = EnsureRoot<AccountId>;
	// The pallet_timestamp pallet
	type TimestampProvider = Timestamp;
	// The balance type for the currency implementation
	type Balance = Balance;
	// The currency implementation that is offered to relayers
	type Currency = Balances;
	// The state machine identifier for this state machine
	type HostStateMachine = HostStateMachine;
	// Optional coprocessor for incoming requests/responses
	type Coprocessor = Coprocessor;
	// Router implementation for routing requests/responses to their respective modules
	type Router = Router;
	// Supported consensus clients
	type ConsensusClients = (
		// Add the grandpa or beefy consensus client here
		ismp_grandpa::consensus::GrandpaConsensusClient<Runtime>,
	);
	// Weight provider for local modules
	type WeightProvider = ();
	// Optional merkle mountain range overlay tree, for cheaper outgoing request proofs.
	// You most likely don't need it, just use the `NoOpMmrTree`
	type Mmr = NoOpMmrTree<Runtime>;
}

impl ismp_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IsmpHost = Ismp;
}

impl pallet_hyperbridge::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IsmpHost = Ismp;
}

// Add the token gateway pallet to your ISMP router
#[derive(Default)]
pub struct Router;
impl IsmpRouter for Router {
	fn module_for_id(&self, id: Vec<u8>) -> Result<Box<dyn IsmpModule>, anyhow::Error> {
		match id.as_slice() {
			id if id == PALLET_HYPERBRIDGE_ID => Ok(Box::new(Hyperbridge::default())),
			id if ChainTransfer::is_ismp_module_id(id) => Ok(Box::new(ChainTransfer::default())),
			_ => Err(Error::ModuleNotFound(id))?,
		}
	}
}
