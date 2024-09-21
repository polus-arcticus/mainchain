// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type { ArgonNodeRuntimeOpaqueSessionKeys, ArgonNodeRuntimeOriginCaller, ArgonNodeRuntimeProxyType, ArgonNodeRuntimeRuntime, ArgonNodeRuntimeRuntimeFreezeReason, ArgonNodeRuntimeRuntimeHoldReason, ArgonNotaryAuditAccountHistoryLookupError, ArgonNotaryAuditErrorVerifyError, ArgonPrimitivesAccountAccountType, ArgonPrimitivesBalanceChangeAccountOrigin, ArgonPrimitivesBalanceChangeMerkleProof, ArgonPrimitivesBitcoinBitcoinBlock, ArgonPrimitivesBitcoinBitcoinCosignScriptPubkey, ArgonPrimitivesBitcoinBitcoinNetwork, ArgonPrimitivesBitcoinBitcoinRejectedReason, ArgonPrimitivesBitcoinBitcoinXPub, ArgonPrimitivesBitcoinCompressedBitcoinPubkey, ArgonPrimitivesBitcoinH256Le, ArgonPrimitivesBitcoinNetworkKind, ArgonPrimitivesBitcoinOpaqueBitcoinXpub, ArgonPrimitivesBitcoinUtxoRef, ArgonPrimitivesBitcoinUtxoValue, ArgonPrimitivesBlockSealAppPublic, ArgonPrimitivesBlockSealAppSignature, ArgonPrimitivesBlockSealBlockPayout, ArgonPrimitivesBlockSealMiningRegistration, ArgonPrimitivesBlockSealMiningSlotConfig, ArgonPrimitivesBlockSealRewardDestination, ArgonPrimitivesBlockSealRewardSharing, ArgonPrimitivesBlockVoteBlockVoteT, ArgonPrimitivesBond, ArgonPrimitivesBondBondError, ArgonPrimitivesBondBondExpiration, ArgonPrimitivesBondBondType, ArgonPrimitivesBondVault, ArgonPrimitivesBondVaultArgons, ArgonPrimitivesBondVaultTerms, ArgonPrimitivesDigestsBlockVoteDigest, ArgonPrimitivesDigestsNotebookDigest, ArgonPrimitivesDigestsNotebookDigestRecord, ArgonPrimitivesDigestsParentVotingKeyDigest, ArgonPrimitivesDomainSemver, ArgonPrimitivesDomainVersionHost, ArgonPrimitivesDomainZoneRecord, ArgonPrimitivesInherentsBitcoinUtxoSync, ArgonPrimitivesInherentsBlockSealInherent, ArgonPrimitivesNotaryNotaryMeta, ArgonPrimitivesNotaryNotaryNotebookKeyDetails, ArgonPrimitivesNotaryNotaryNotebookVoteDigestDetails, ArgonPrimitivesNotaryNotaryRecord, ArgonPrimitivesNotebookChainTransfer, ArgonPrimitivesNotebookNotebookHeader, ArgonPrimitivesNotebookSignedNotebookHeader, ArgonPrimitivesProvidersBlockSealerInfo, ArgonPrimitivesTickTicker, FinalityGrandpaEquivocationPrecommit, FinalityGrandpaEquivocationPrevote, FinalityGrandpaPrecommit, FinalityGrandpaPrevote, FrameMetadataHashExtensionCheckMetadataHash, FrameMetadataHashExtensionMode, FrameSupportDispatchDispatchClass, FrameSupportDispatchDispatchInfo, FrameSupportDispatchPays, FrameSupportDispatchPerDispatchClassU32, FrameSupportDispatchPerDispatchClassWeight, FrameSupportDispatchPerDispatchClassWeightsPerClass, FrameSupportDispatchRawOrigin, FrameSupportPalletId, FrameSupportTokensMiscBalanceStatus, FrameSupportTokensMiscIdAmountRuntimeFreezeReason, FrameSupportTokensMiscIdAmountRuntimeHoldReason, FrameSystemAccountInfo, FrameSystemCall, FrameSystemCodeUpgradeAuthorization, FrameSystemError, FrameSystemEvent, FrameSystemEventRecord, FrameSystemExtensionsCheckGenesis, FrameSystemExtensionsCheckNonZeroSender, FrameSystemExtensionsCheckNonce, FrameSystemExtensionsCheckSpecVersion, FrameSystemExtensionsCheckTxVersion, FrameSystemExtensionsCheckWeight, FrameSystemLastRuntimeUpgradeInfo, FrameSystemLimitsBlockLength, FrameSystemLimitsBlockWeights, FrameSystemLimitsWeightsPerClass, FrameSystemPhase, PalletBalancesAccountData, PalletBalancesAdjustmentDirection, PalletBalancesBalanceLock, PalletBalancesCall, PalletBalancesError, PalletBalancesEvent, PalletBalancesReasons, PalletBalancesReserveData, PalletBitcoinUtxosCall, PalletBitcoinUtxosError, PalletBitcoinUtxosEvent, PalletBlockRewardsCall, PalletBlockRewardsError, PalletBlockRewardsEvent, PalletBlockRewardsFreezeReason, PalletBlockRewardsHoldReason, PalletBlockSealCall, PalletBlockSealError, PalletBlockSealSpecCall, PalletBlockSealSpecError, PalletBlockSealSpecEvent, PalletBondCall, PalletBondError, PalletBondEvent, PalletBondHoldReason, PalletBondUtxoCosignRequest, PalletBondUtxoState, PalletChainTransferCall, PalletChainTransferError, PalletChainTransferEvent, PalletChainTransferQueuedTransferOut, PalletDomainsCall, PalletDomainsDomainRegistration, PalletDomainsError, PalletDomainsEvent, PalletGrandpaCall, PalletGrandpaError, PalletGrandpaEvent, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletMiningSlotCall, PalletMiningSlotError, PalletMiningSlotEvent, PalletMiningSlotHoldReason, PalletMiningSlotMinerHistory, PalletMiningSlotMiningSlotBid, PalletMintCall, PalletMintError, PalletMintEvent, PalletMintMintType, PalletMultisigCall, PalletMultisigError, PalletMultisigEvent, PalletMultisigMultisig, PalletMultisigTimepoint, PalletNotariesCall, PalletNotariesError, PalletNotariesEvent, PalletNotebookCall, PalletNotebookError, PalletNotebookEvent, PalletOffencesEvent, PalletPriceIndexCall, PalletPriceIndexError, PalletPriceIndexEvent, PalletPriceIndexPriceIndex, PalletProxyAnnouncement, PalletProxyCall, PalletProxyError, PalletProxyEvent, PalletProxyProxyDefinition, PalletSessionCall, PalletSessionError, PalletSessionEvent, PalletSudoCall, PalletSudoError, PalletSudoEvent, PalletTicksCall, PalletTicksError, PalletTimestampCall, PalletTransactionPaymentChargeTransactionPayment, PalletTransactionPaymentEvent, PalletTransactionPaymentReleases, PalletTxPauseCall, PalletTxPauseError, PalletTxPauseEvent, PalletUtilityCall, PalletUtilityError, PalletUtilityEvent, PalletVaultsCall, PalletVaultsError, PalletVaultsEvent, PalletVaultsHoldReason, PalletVaultsVaultConfig, SpArithmeticArithmeticError, SpConsensusGrandpaAppPublic, SpConsensusGrandpaAppSignature, SpConsensusGrandpaEquivocation, SpConsensusGrandpaEquivocationProof, SpCoreCryptoKeyTypeId, SpCoreVoid, SpRuntimeDigest, SpRuntimeDigestDigestItem, SpRuntimeDispatchError, SpRuntimeModuleError, SpRuntimeMultiSignature, SpRuntimeTokenError, SpRuntimeTransactionalError, SpSessionMembershipProof, SpStakingOffenceOffenceDetails, SpVersionRuntimeVersion, SpWeightsRuntimeDbWeight, SpWeightsWeightV2Weight } from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
  interface InterfaceTypes {
    ArgonNodeRuntimeOpaqueSessionKeys: ArgonNodeRuntimeOpaqueSessionKeys;
    ArgonNodeRuntimeOriginCaller: ArgonNodeRuntimeOriginCaller;
    ArgonNodeRuntimeProxyType: ArgonNodeRuntimeProxyType;
    ArgonNodeRuntimeRuntime: ArgonNodeRuntimeRuntime;
    ArgonNodeRuntimeRuntimeFreezeReason: ArgonNodeRuntimeRuntimeFreezeReason;
    ArgonNodeRuntimeRuntimeHoldReason: ArgonNodeRuntimeRuntimeHoldReason;
    ArgonNotaryAuditAccountHistoryLookupError: ArgonNotaryAuditAccountHistoryLookupError;
    ArgonNotaryAuditErrorVerifyError: ArgonNotaryAuditErrorVerifyError;
    ArgonPrimitivesAccountAccountType: ArgonPrimitivesAccountAccountType;
    ArgonPrimitivesBalanceChangeAccountOrigin: ArgonPrimitivesBalanceChangeAccountOrigin;
    ArgonPrimitivesBalanceChangeMerkleProof: ArgonPrimitivesBalanceChangeMerkleProof;
    ArgonPrimitivesBitcoinBitcoinBlock: ArgonPrimitivesBitcoinBitcoinBlock;
    ArgonPrimitivesBitcoinBitcoinCosignScriptPubkey: ArgonPrimitivesBitcoinBitcoinCosignScriptPubkey;
    ArgonPrimitivesBitcoinBitcoinNetwork: ArgonPrimitivesBitcoinBitcoinNetwork;
    ArgonPrimitivesBitcoinBitcoinRejectedReason: ArgonPrimitivesBitcoinBitcoinRejectedReason;
    ArgonPrimitivesBitcoinBitcoinXPub: ArgonPrimitivesBitcoinBitcoinXPub;
    ArgonPrimitivesBitcoinCompressedBitcoinPubkey: ArgonPrimitivesBitcoinCompressedBitcoinPubkey;
    ArgonPrimitivesBitcoinH256Le: ArgonPrimitivesBitcoinH256Le;
    ArgonPrimitivesBitcoinNetworkKind: ArgonPrimitivesBitcoinNetworkKind;
    ArgonPrimitivesBitcoinOpaqueBitcoinXpub: ArgonPrimitivesBitcoinOpaqueBitcoinXpub;
    ArgonPrimitivesBitcoinUtxoRef: ArgonPrimitivesBitcoinUtxoRef;
    ArgonPrimitivesBitcoinUtxoValue: ArgonPrimitivesBitcoinUtxoValue;
    ArgonPrimitivesBlockSealAppPublic: ArgonPrimitivesBlockSealAppPublic;
    ArgonPrimitivesBlockSealAppSignature: ArgonPrimitivesBlockSealAppSignature;
    ArgonPrimitivesBlockSealBlockPayout: ArgonPrimitivesBlockSealBlockPayout;
    ArgonPrimitivesBlockSealMiningRegistration: ArgonPrimitivesBlockSealMiningRegistration;
    ArgonPrimitivesBlockSealMiningSlotConfig: ArgonPrimitivesBlockSealMiningSlotConfig;
    ArgonPrimitivesBlockSealRewardDestination: ArgonPrimitivesBlockSealRewardDestination;
    ArgonPrimitivesBlockSealRewardSharing: ArgonPrimitivesBlockSealRewardSharing;
    ArgonPrimitivesBlockVoteBlockVoteT: ArgonPrimitivesBlockVoteBlockVoteT;
    ArgonPrimitivesBond: ArgonPrimitivesBond;
    ArgonPrimitivesBondBondError: ArgonPrimitivesBondBondError;
    ArgonPrimitivesBondBondExpiration: ArgonPrimitivesBondBondExpiration;
    ArgonPrimitivesBondBondType: ArgonPrimitivesBondBondType;
    ArgonPrimitivesBondVault: ArgonPrimitivesBondVault;
    ArgonPrimitivesBondVaultArgons: ArgonPrimitivesBondVaultArgons;
    ArgonPrimitivesBondVaultTerms: ArgonPrimitivesBondVaultTerms;
    ArgonPrimitivesDigestsBlockVoteDigest: ArgonPrimitivesDigestsBlockVoteDigest;
    ArgonPrimitivesDigestsNotebookDigest: ArgonPrimitivesDigestsNotebookDigest;
    ArgonPrimitivesDigestsNotebookDigestRecord: ArgonPrimitivesDigestsNotebookDigestRecord;
    ArgonPrimitivesDigestsParentVotingKeyDigest: ArgonPrimitivesDigestsParentVotingKeyDigest;
    ArgonPrimitivesDomainSemver: ArgonPrimitivesDomainSemver;
    ArgonPrimitivesDomainVersionHost: ArgonPrimitivesDomainVersionHost;
    ArgonPrimitivesDomainZoneRecord: ArgonPrimitivesDomainZoneRecord;
    ArgonPrimitivesInherentsBitcoinUtxoSync: ArgonPrimitivesInherentsBitcoinUtxoSync;
    ArgonPrimitivesInherentsBlockSealInherent: ArgonPrimitivesInherentsBlockSealInherent;
    ArgonPrimitivesNotaryNotaryMeta: ArgonPrimitivesNotaryNotaryMeta;
    ArgonPrimitivesNotaryNotaryNotebookKeyDetails: ArgonPrimitivesNotaryNotaryNotebookKeyDetails;
    ArgonPrimitivesNotaryNotaryNotebookVoteDigestDetails: ArgonPrimitivesNotaryNotaryNotebookVoteDigestDetails;
    ArgonPrimitivesNotaryNotaryRecord: ArgonPrimitivesNotaryNotaryRecord;
    ArgonPrimitivesNotebookChainTransfer: ArgonPrimitivesNotebookChainTransfer;
    ArgonPrimitivesNotebookNotebookHeader: ArgonPrimitivesNotebookNotebookHeader;
    ArgonPrimitivesNotebookSignedNotebookHeader: ArgonPrimitivesNotebookSignedNotebookHeader;
    ArgonPrimitivesProvidersBlockSealerInfo: ArgonPrimitivesProvidersBlockSealerInfo;
    ArgonPrimitivesTickTicker: ArgonPrimitivesTickTicker;
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FrameMetadataHashExtensionCheckMetadataHash: FrameMetadataHashExtensionCheckMetadataHash;
    FrameMetadataHashExtensionMode: FrameMetadataHashExtensionMode;
    FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
    FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
    FrameSupportDispatchPays: FrameSupportDispatchPays;
    FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
    FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
    FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
    FrameSupportPalletId: FrameSupportPalletId;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSupportTokensMiscIdAmountRuntimeFreezeReason: FrameSupportTokensMiscIdAmountRuntimeFreezeReason;
    FrameSupportTokensMiscIdAmountRuntimeHoldReason: FrameSupportTokensMiscIdAmountRuntimeHoldReason;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemCodeUpgradeAuthorization: FrameSystemCodeUpgradeAuthorization;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesAdjustmentDirection: PalletBalancesAdjustmentDirection;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletBitcoinUtxosCall: PalletBitcoinUtxosCall;
    PalletBitcoinUtxosError: PalletBitcoinUtxosError;
    PalletBitcoinUtxosEvent: PalletBitcoinUtxosEvent;
    PalletBlockRewardsCall: PalletBlockRewardsCall;
    PalletBlockRewardsError: PalletBlockRewardsError;
    PalletBlockRewardsEvent: PalletBlockRewardsEvent;
    PalletBlockRewardsFreezeReason: PalletBlockRewardsFreezeReason;
    PalletBlockRewardsHoldReason: PalletBlockRewardsHoldReason;
    PalletBlockSealCall: PalletBlockSealCall;
    PalletBlockSealError: PalletBlockSealError;
    PalletBlockSealSpecCall: PalletBlockSealSpecCall;
    PalletBlockSealSpecError: PalletBlockSealSpecError;
    PalletBlockSealSpecEvent: PalletBlockSealSpecEvent;
    PalletBondCall: PalletBondCall;
    PalletBondError: PalletBondError;
    PalletBondEvent: PalletBondEvent;
    PalletBondHoldReason: PalletBondHoldReason;
    PalletBondUtxoCosignRequest: PalletBondUtxoCosignRequest;
    PalletBondUtxoState: PalletBondUtxoState;
    PalletChainTransferCall: PalletChainTransferCall;
    PalletChainTransferError: PalletChainTransferError;
    PalletChainTransferEvent: PalletChainTransferEvent;
    PalletChainTransferQueuedTransferOut: PalletChainTransferQueuedTransferOut;
    PalletDomainsCall: PalletDomainsCall;
    PalletDomainsDomainRegistration: PalletDomainsDomainRegistration;
    PalletDomainsError: PalletDomainsError;
    PalletDomainsEvent: PalletDomainsEvent;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletMiningSlotCall: PalletMiningSlotCall;
    PalletMiningSlotError: PalletMiningSlotError;
    PalletMiningSlotEvent: PalletMiningSlotEvent;
    PalletMiningSlotHoldReason: PalletMiningSlotHoldReason;
    PalletMiningSlotMinerHistory: PalletMiningSlotMinerHistory;
    PalletMiningSlotMiningSlotBid: PalletMiningSlotMiningSlotBid;
    PalletMintCall: PalletMintCall;
    PalletMintError: PalletMintError;
    PalletMintEvent: PalletMintEvent;
    PalletMintMintType: PalletMintMintType;
    PalletMultisigCall: PalletMultisigCall;
    PalletMultisigError: PalletMultisigError;
    PalletMultisigEvent: PalletMultisigEvent;
    PalletMultisigMultisig: PalletMultisigMultisig;
    PalletMultisigTimepoint: PalletMultisigTimepoint;
    PalletNotariesCall: PalletNotariesCall;
    PalletNotariesError: PalletNotariesError;
    PalletNotariesEvent: PalletNotariesEvent;
    PalletNotebookCall: PalletNotebookCall;
    PalletNotebookError: PalletNotebookError;
    PalletNotebookEvent: PalletNotebookEvent;
    PalletOffencesEvent: PalletOffencesEvent;
    PalletPriceIndexCall: PalletPriceIndexCall;
    PalletPriceIndexError: PalletPriceIndexError;
    PalletPriceIndexEvent: PalletPriceIndexEvent;
    PalletPriceIndexPriceIndex: PalletPriceIndexPriceIndex;
    PalletProxyAnnouncement: PalletProxyAnnouncement;
    PalletProxyCall: PalletProxyCall;
    PalletProxyError: PalletProxyError;
    PalletProxyEvent: PalletProxyEvent;
    PalletProxyProxyDefinition: PalletProxyProxyDefinition;
    PalletSessionCall: PalletSessionCall;
    PalletSessionError: PalletSessionError;
    PalletSessionEvent: PalletSessionEvent;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTicksCall: PalletTicksCall;
    PalletTicksError: PalletTicksError;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
    PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletTxPauseCall: PalletTxPauseCall;
    PalletTxPauseError: PalletTxPauseError;
    PalletTxPauseEvent: PalletTxPauseEvent;
    PalletUtilityCall: PalletUtilityCall;
    PalletUtilityError: PalletUtilityError;
    PalletUtilityEvent: PalletUtilityEvent;
    PalletVaultsCall: PalletVaultsCall;
    PalletVaultsError: PalletVaultsError;
    PalletVaultsEvent: PalletVaultsEvent;
    PalletVaultsHoldReason: PalletVaultsHoldReason;
    PalletVaultsVaultConfig: PalletVaultsVaultConfig;
    SpArithmeticArithmeticError: SpArithmeticArithmeticError;
    SpConsensusGrandpaAppPublic: SpConsensusGrandpaAppPublic;
    SpConsensusGrandpaAppSignature: SpConsensusGrandpaAppSignature;
    SpConsensusGrandpaEquivocation: SpConsensusGrandpaEquivocation;
    SpConsensusGrandpaEquivocationProof: SpConsensusGrandpaEquivocationProof;
    SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
    SpCoreVoid: SpCoreVoid;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpSessionMembershipProof: SpSessionMembershipProof;
    SpStakingOffenceOffenceDetails: SpStakingOffenceOffenceDetails;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
    SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
  } // InterfaceTypes
} // declare module
