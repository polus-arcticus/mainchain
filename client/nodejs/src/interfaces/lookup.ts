// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /**
   * Lookup3: frame_system::AccountInfo<Nonce, pallet_balances::types::AccountData<Balance>>
   **/
  FrameSystemAccountInfo: {
    nonce: 'u32',
    consumers: 'u32',
    providers: 'u32',
    sufficients: 'u32',
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::types::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    frozen: 'u128',
    flags: 'u128'
  },
  /**
   * Lookup8: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
   **/
  FrameSupportDispatchPerDispatchClassWeight: {
    normal: 'SpWeightsWeightV2Weight',
    operational: 'SpWeightsWeightV2Weight',
    mandatory: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup9: sp_weights::weight_v2::Weight
   **/
  SpWeightsWeightV2Weight: {
    refTime: 'Compact<u64>',
    proofSize: 'Compact<u64>'
  },
  /**
   * Lookup14: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup16: sp_runtime::generic::digest::DigestItem
   **/
  SpRuntimeDigestDigestItem: {
    _enum: {
      Other: 'Bytes',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      Consensus: '([u8;4],Bytes)',
      Seal: '([u8;4],Bytes)',
      PreRuntime: '([u8;4],Bytes)',
      __Unused7: 'Null',
      RuntimeEnvironmentUpdated: 'Null'
    }
  },
  /**
   * Lookup19: frame_system::EventRecord<ulx_node_runtime::RuntimeEvent, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup21: frame_system::pallet::Event<T>
   **/
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: 'FrameSupportDispatchDispatchInfo',
      },
      ExtrinsicFailed: {
        dispatchError: 'SpRuntimeDispatchError',
        dispatchInfo: 'FrameSupportDispatchDispatchInfo',
      },
      CodeUpdated: 'Null',
      NewAccount: {
        account: 'AccountId32',
      },
      KilledAccount: {
        account: 'AccountId32',
      },
      Remarked: {
        _alias: {
          hash_: 'hash',
        },
        sender: 'AccountId32',
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup22: frame_support::dispatch::DispatchInfo
   **/
  FrameSupportDispatchDispatchInfo: {
    weight: 'SpWeightsWeightV2Weight',
    class: 'FrameSupportDispatchDispatchClass',
    paysFee: 'FrameSupportDispatchPays'
  },
  /**
   * Lookup23: frame_support::dispatch::DispatchClass
   **/
  FrameSupportDispatchDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup24: frame_support::dispatch::Pays
   **/
  FrameSupportDispatchPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup25: sp_runtime::DispatchError
   **/
  SpRuntimeDispatchError: {
    _enum: {
      Other: 'Null',
      CannotLookup: 'Null',
      BadOrigin: 'Null',
      Module: 'SpRuntimeModuleError',
      ConsumerRemaining: 'Null',
      NoProviders: 'Null',
      TooManyConsumers: 'Null',
      Token: 'SpRuntimeTokenError',
      Arithmetic: 'SpArithmeticArithmeticError',
      Transactional: 'SpRuntimeTransactionalError',
      Exhausted: 'Null',
      Corruption: 'Null',
      Unavailable: 'Null',
      RootNotAllowed: 'Null'
    }
  },
  /**
   * Lookup26: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup27: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['FundsUnavailable', 'OnlyProvider', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported', 'CannotCreateHold', 'NotExpendable', 'Blocked']
  },
  /**
   * Lookup28: sp_arithmetic::ArithmeticError
   **/
  SpArithmeticArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup29: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup30: pallet_mining_slot::pallet::Event<T>
   **/
  PalletMiningSlotEvent: {
    _enum: {
      NewMiners: {
        startIndex: 'u32',
        newMiners: 'Vec<UlxPrimitivesBlockSealMiningRegistration>',
      },
      SlotBidderAdded: {
        accountId: 'AccountId32',
        bidAmount: 'u128',
        index: 'u32',
      },
      SlotBidderReplaced: {
        accountId: 'AccountId32',
        bondId: 'Option<u64>',
        keptOwnershipBond: 'bool',
      },
      UnbondedMiner: {
        accountId: 'AccountId32',
        bondId: 'Option<u64>',
        keptOwnershipBond: 'bool'
      }
    }
  },
  /**
   * Lookup32: ulx_primitives::block_seal::MiningRegistration<sp_core::crypto::AccountId32, BondId, Balance>
   **/
  UlxPrimitivesBlockSealMiningRegistration: {
    accountId: 'AccountId32',
    rewardDestination: 'UlxPrimitivesBlockSealRewardDestination',
    bondId: 'Option<u64>',
    bondAmount: 'Compact<u128>',
    ownershipTokens: 'Compact<u128>'
  },
  /**
   * Lookup33: ulx_primitives::block_seal::RewardDestination<sp_core::crypto::AccountId32>
   **/
  UlxPrimitivesBlockSealRewardDestination: {
    _enum: {
      Owner: 'Null',
      Account: 'AccountId32'
    }
  },
  /**
   * Lookup38: pallet_bond::pallet::Event<T>
   **/
  PalletBondEvent: {
    _enum: {
      BondFundOffered: {
        bondFundId: 'u32',
        amountOffered: 'u128',
        expirationBlock: 'u32',
        offerAccountId: 'AccountId32',
      },
      BondFundExtended: {
        bondFundId: 'u32',
        amountOffered: 'u128',
        expirationBlock: 'u32',
      },
      BondFundEnded: {
        bondFundId: 'u32',
        amountStillBonded: 'u128',
      },
      BondFundExpired: {
        bondFundId: 'u32',
        offerAccountId: 'AccountId32',
      },
      BondedSelf: {
        bondId: 'u64',
        bondedAccountId: 'AccountId32',
        amount: 'u128',
        completionBlock: 'u32',
      },
      BondLeased: {
        bondFundId: 'u32',
        bondId: 'u64',
        bondedAccountId: 'AccountId32',
        amount: 'u128',
        totalFee: 'u128',
        annualPercentRate: 'u32',
        completionBlock: 'u32',
      },
      BondExtended: {
        bondFundId: 'Option<u32>',
        bondId: 'u64',
        amount: 'u128',
        completionBlock: 'u32',
        feeChange: 'u128',
        annualPercentRate: 'u32',
      },
      BondCompleted: {
        bondFundId: 'Option<u32>',
        bondId: 'u64',
      },
      BondFeeRefund: {
        bondFundId: 'u32',
        bondId: 'u64',
        bondedAccountId: 'AccountId32',
        bondFundReductionForPayment: 'u128',
        finalFee: 'u128',
        refundAmount: 'u128',
      },
      BondLocked: {
        bondId: 'u64',
        bondedAccountId: 'AccountId32',
      },
      BondUnlocked: {
        bondId: 'u64',
        bondedAccountId: 'AccountId32'
      }
    }
  },
  /**
   * Lookup40: pallet_notaries::pallet::Event<T>
   **/
  PalletNotariesEvent: {
    _enum: {
      NotaryProposed: {
        operatorAccount: 'AccountId32',
        meta: 'UlxPrimitivesNotaryNotaryMeta',
        expires: 'u32',
      },
      NotaryActivated: {
        notary: 'UlxPrimitivesNotaryNotaryRecord',
      },
      NotaryMetaUpdateQueued: {
        notaryId: 'u32',
        meta: 'UlxPrimitivesNotaryNotaryMeta',
        effectiveBlock: 'u32',
      },
      NotaryMetaUpdated: {
        notaryId: 'u32',
        meta: 'UlxPrimitivesNotaryNotaryMeta'
      }
    }
  },
  /**
   * Lookup41: ulx_primitives::notary::NotaryMeta<MaxHosts>
   **/
  UlxPrimitivesNotaryNotaryMeta: {
    public: 'SpCoreEd25519Public',
    hosts: 'Vec<UlxPrimitivesHost>'
  },
  /**
   * Lookup42: sp_core::ed25519::Public
   **/
  SpCoreEd25519Public: '[u8;32]',
  /**
   * Lookup44: ulx_primitives::host::Host
   **/
  UlxPrimitivesHost: {
    ip: 'Compact<u32>',
    port: 'Compact<u16>',
    isSecure: 'bool'
  },
  /**
   * Lookup49: ulx_primitives::notary::NotaryRecord<sp_core::crypto::AccountId32, BlockNumber, MaxHosts>
   **/
  UlxPrimitivesNotaryNotaryRecord: {
    notaryId: 'Compact<u32>',
    operatorAccountId: 'AccountId32',
    activatedBlock: 'Compact<u32>',
    metaUpdatedBlock: 'Compact<u32>',
    meta: 'UlxPrimitivesNotaryNotaryMeta'
  },
  /**
   * Lookup50: pallet_notebook::pallet::Event<T>
   **/
  PalletNotebookEvent: {
    _enum: {
      NotebookSubmitted: {
        notaryId: 'u32',
        notebookNumber: 'u32',
      },
      NotebookAuditFailure: {
        notaryId: 'u32',
        notebookNumber: 'u32',
        firstFailureReason: 'UlxNotaryAuditErrorVerifyError'
      }
    }
  },
  /**
   * Lookup51: ulx_notary_audit::error::VerifyError
   **/
  UlxNotaryAuditErrorVerifyError: {
    _enum: {
      MissingAccountOrigin: {
        accountId: 'AccountId32',
        accountType: 'UlxPrimitivesNoteAccountType',
      },
      HistoryLookupError: {
        source: 'UlxNotaryAuditAccountHistoryLookupError',
      },
      InvalidAccountChangelist: 'Null',
      InvalidChainTransfersList: 'Null',
      InvalidBalanceChangeRoot: 'Null',
      InvalidHeaderTaxRecorded: 'Null',
      InvalidPreviousNonce: 'Null',
      InvalidPreviousBalance: 'Null',
      InvalidPreviousAccountOrigin: 'Null',
      InvalidPreviousBalanceChangeNotebook: 'Null',
      InvalidBalanceChange: 'Null',
      InvalidBalanceChangeSignature: {
        changeIndex: 'u16',
      },
      InvalidNoteRecipients: 'Null',
      BalanceChangeError: {
        changeIndex: 'u16',
        noteIndex: 'u16',
        message: 'Text',
      },
      InvalidNetBalanceChangeset: 'Null',
      InsufficientBalance: {
        balance: 'u128',
        amount: 'u128',
        noteIndex: 'u16',
        changeIndex: 'u16',
      },
      ExceededMaxBalance: {
        balance: 'u128',
        amount: 'u128',
        noteIndex: 'u16',
        changeIndex: 'u16',
      },
      BalanceChangeMismatch: {
        changeIndex: 'u16',
        providedBalance: 'u128',
        calculatedBalance: 'i128',
      },
      BalanceChangeNotNetZero: {
        sent: 'u128',
        claimed: 'u128',
      },
      InvalidDomainLeaseAllocation: 'Null',
      InvalidDomainName: 'Null',
      TaxBalanceChangeNotNetZero: {
        sent: 'u128',
        claimed: 'u128',
      },
      MissingBalanceProof: 'Null',
      InvalidPreviousBalanceProof: 'Null',
      InvalidNotebookHash: 'Null',
      InvalidNotebookHeaderHash: 'Null',
      DuplicateChainTransfer: 'Null',
      DuplicatedAccountOriginUid: 'Null',
      InvalidNotarySignature: 'Null',
      NotebookTooOld: 'Null',
      DecodeError: 'Null',
      AccountChannelHoldDoesntExist: 'Null',
      AccountAlreadyHasChannelHold: 'Null',
      ChannelHoldNotReadyForClaim: 'Null',
      AccountLocked: 'Null',
      MissingChannelHoldNote: 'Null',
      InvalidChannelHoldNote: 'Null',
      InvalidChannelClaimers: 'Null',
      ChannelNoteBelowMinimum: 'Null',
      InvalidTaxNoteAccount: 'Null',
      InvalidTaxOperation: 'Null',
      InsufficientTaxIncluded: {
        taxSent: 'u128',
        taxOwed: 'u128',
        accountId: 'AccountId32',
      },
      InsufficientBlockVoteTax: 'Null',
      InvalidBlockVoteAllocation: 'Null',
      InvalidBlockVoteRoot: 'Null',
      InvalidBlockVotesCount: 'Null',
      InvalidBlockVotingPower: 'Null',
      InvalidBlockVoteList: 'Null',
      InvalidComputeProof: 'Null',
      InvalidBlockVoteSource: 'Null',
      BlockVoteInvalidSignature: 'Null',
      InsufficientBlockVoteMinimum: 'Null',
      BlockVoteDataDomainMismatch: 'Null',
      BlockVoteChannelReused: 'Null'
    }
  },
  /**
   * Lookup52: ulx_primitives::note::AccountType
   **/
  UlxPrimitivesNoteAccountType: {
    _enum: ['Tax', 'Deposit']
  },
  /**
   * Lookup53: ulx_notary_audit::AccountHistoryLookupError
   **/
  UlxNotaryAuditAccountHistoryLookupError: {
    _enum: ['RootNotFound', 'LastChangeNotFound', 'InvalidTransferToLocalchain', 'BlockSpecificationNotFound']
  },
  /**
   * Lookup56: pallet_chain_transfer::pallet::Event<T>
   **/
  PalletChainTransferEvent: {
    _enum: {
      TransferToLocalchain: {
        accountId: 'AccountId32',
        amount: 'u128',
        accountNonce: 'u32',
        notaryId: 'u32',
        expirationBlock: 'u32',
      },
      TransferToLocalchainExpired: {
        accountId: 'AccountId32',
        accountNonce: 'u32',
        notaryId: 'u32',
      },
      TransferIn: {
        accountId: 'AccountId32',
        amount: 'u128',
        notaryId: 'u32'
      }
    }
  },
  /**
   * Lookup57: pallet_block_seal_spec::pallet::Event<T>
   **/
  PalletBlockSealSpecEvent: {
    _enum: {
      VoteMinimumAdjusted: {
        expectedBlockVotes: 'u128',
        actualBlockVotes: 'u128',
        startVoteMinimum: 'u128',
        newVoteMinimum: 'u128',
      },
      ComputeDifficultyAdjusted: {
        expectedBlockTime: 'u64',
        actualBlockTime: 'u64',
        startDifficulty: 'u128',
        newDifficulty: 'u128'
      }
    }
  },
  /**
   * Lookup58: pallet_data_domain::pallet::Event<T>
   **/
  PalletDataDomainEvent: {
    _enum: {
      ZoneRecordUpdated: {
        domain: 'UlxPrimitivesDataDomain',
        zoneRecord: 'UlxPrimitivesDataDomainZoneRecord',
      },
      DataDomainRegistered: {
        domain: 'UlxPrimitivesDataDomain',
        registration: 'PalletDataDomainDataDomainRegistration',
      },
      DataDomainRenewed: {
        domain: 'UlxPrimitivesDataDomain',
      },
      DataDomainExpired: {
        domain: 'UlxPrimitivesDataDomain',
      },
      DataDomainRegistrationCanceled: {
        domain: 'UlxPrimitivesDataDomain',
        registration: 'PalletDataDomainDataDomainRegistration'
      }
    }
  },
  /**
   * Lookup59: ulx_primitives::data_domain::DataDomain
   **/
  UlxPrimitivesDataDomain: {
    domainName: 'Bytes',
    topLevelDomain: 'UlxPrimitivesDataTLD'
  },
  /**
   * Lookup61: ulx_primitives::data_tld::DataTLD
   **/
  UlxPrimitivesDataTLD: {
    _enum: ['Analytics', 'Automotive', 'Bikes', 'Business', 'Cars', 'Communication', 'Entertainment', 'Finance', 'Flights', 'Health', 'Hotels', 'Jobs', 'News', 'RealEstate', 'Restaurants', 'Shopping', 'Sports', 'Transportation', 'Travel', 'Weather']
  },
  /**
   * Lookup62: ulx_primitives::data_domain::ZoneRecord<sp_core::crypto::AccountId32>
   **/
  UlxPrimitivesDataDomainZoneRecord: {
    paymentAccount: 'AccountId32',
    versions: 'BTreeMap<UlxPrimitivesDataDomainSemver, UlxPrimitivesDataDomainVersionHost>'
  },
  /**
   * Lookup64: ulx_primitives::data_domain::Semver
   **/
  UlxPrimitivesDataDomainSemver: {
    major: 'u32',
    minor: 'u32',
    patch: 'u32'
  },
  /**
   * Lookup65: ulx_primitives::data_domain::VersionHost
   **/
  UlxPrimitivesDataDomainVersionHost: {
    datastoreId: 'Bytes',
    host: 'UlxPrimitivesHost'
  },
  /**
   * Lookup68: pallet_data_domain::DataDomainRegistration<sp_core::crypto::AccountId32>
   **/
  PalletDataDomainDataDomainRegistration: {
    accountId: 'AccountId32',
    registeredAtTick: 'u32'
  },
  /**
   * Lookup69: pallet_session::pallet::Event
   **/
  PalletSessionEvent: {
    _enum: {
      NewSession: {
        sessionIndex: 'u32'
      }
    }
  },
  /**
   * Lookup70: pallet_block_rewards::pallet::Event<T>
   **/
  PalletBlockRewardsEvent: {
    _enum: {
      RewardCreated: {
        maturationBlock: 'u32',
        rewards: 'Vec<PalletBlockRewardsBlockPayout>',
      },
      RewardUnlocked: {
        rewards: 'Vec<PalletBlockRewardsBlockPayout>'
      }
    }
  },
  /**
   * Lookup72: pallet_block_rewards::pallet::BlockPayout<sp_core::crypto::AccountId32, Balance>
   **/
  PalletBlockRewardsBlockPayout: {
    accountId: 'AccountId32',
    ulixees: 'u128',
    argons: 'u128'
  },
  /**
   * Lookup73: pallet_grandpa::pallet::Event
   **/
  PalletGrandpaEvent: {
    _enum: {
      NewAuthorities: {
        authoritySet: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
      },
      Paused: 'Null',
      Resumed: 'Null'
    }
  },
  /**
   * Lookup76: sp_consensus_grandpa::app::Public
   **/
  SpConsensusGrandpaAppPublic: 'SpCoreEd25519Public',
  /**
   * Lookup77: pallet_offences::pallet::Event
   **/
  PalletOffencesEvent: {
    _enum: {
      Offence: {
        kind: '[u8;16]',
        timeslot: 'Bytes'
      }
    }
  },
  /**
   * Lookup79: pallet_balances::pallet::Event<T, I>
   **/
  PalletBalancesEvent: {
    _enum: {
      Endowed: {
        account: 'AccountId32',
        freeBalance: 'u128',
      },
      DustLost: {
        account: 'AccountId32',
        amount: 'u128',
      },
      Transfer: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
      },
      BalanceSet: {
        who: 'AccountId32',
        free: 'u128',
      },
      Reserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unreserved: {
        who: 'AccountId32',
        amount: 'u128',
      },
      ReserveRepatriated: {
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
        destinationStatus: 'FrameSupportTokensMiscBalanceStatus',
      },
      Deposit: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Withdraw: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Slashed: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Minted: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Burned: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Suspended: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Restored: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Upgraded: {
        who: 'AccountId32',
      },
      Issued: {
        amount: 'u128',
      },
      Rescinded: {
        amount: 'u128',
      },
      Locked: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unlocked: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Frozen: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Thawed: {
        who: 'AccountId32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup80: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup81: pallet_mint::pallet::Event<T>
   **/
  PalletMintEvent: 'Null',
  /**
   * Lookup83: pallet_tx_pause::pallet::Event<T>
   **/
  PalletTxPauseEvent: {
    _enum: {
      CallPaused: {
        fullName: '(Bytes,Bytes)',
      },
      CallUnpaused: {
        fullName: '(Bytes,Bytes)'
      }
    }
  },
  /**
   * Lookup86: pallet_transaction_payment::pallet::Event<T>
   **/
  PalletTransactionPaymentEvent: {
    _enum: {
      TransactionFeePaid: {
        who: 'AccountId32',
        actualFee: 'u128',
        tip: 'u128'
      }
    }
  },
  /**
   * Lookup87: pallet_sudo::pallet::Event<T>
   **/
  PalletSudoEvent: {
    _enum: {
      Sudid: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>',
      },
      KeyChanged: {
        _alias: {
          new_: 'new',
        },
        old: 'Option<AccountId32>',
        new_: 'AccountId32',
      },
      KeyRemoved: 'Null',
      SudoAsDone: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup91: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup95: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup96: frame_system::pallet::Call<T>
   **/
  FrameSystemCall: {
    _enum: {
      remark: {
        remark: 'Bytes',
      },
      set_heap_pages: {
        pages: 'u64',
      },
      set_code: {
        code: 'Bytes',
      },
      set_code_without_checks: {
        code: 'Bytes',
      },
      set_storage: {
        items: 'Vec<(Bytes,Bytes)>',
      },
      kill_storage: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'Vec<Bytes>',
      },
      kill_prefix: {
        prefix: 'Bytes',
        subkeys: 'u32',
      },
      remark_with_event: {
        remark: 'Bytes'
      }
    }
  },
  /**
   * Lookup100: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'SpWeightsWeightV2Weight',
    maxBlock: 'SpWeightsWeightV2Weight',
    perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup101: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportDispatchPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup102: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'SpWeightsWeightV2Weight',
    maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
    maxTotal: 'Option<SpWeightsWeightV2Weight>',
    reserved: 'Option<SpWeightsWeightV2Weight>'
  },
  /**
   * Lookup104: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportDispatchPerDispatchClassU32'
  },
  /**
   * Lookup105: frame_support::dispatch::PerDispatchClass<T>
   **/
  FrameSupportDispatchPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup106: sp_weights::RuntimeDbWeight
   **/
  SpWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup107: sp_version::RuntimeVersion
   **/
  SpVersionRuntimeVersion: {
    specName: 'Text',
    implName: 'Text',
    authoringVersion: 'u32',
    specVersion: 'u32',
    implVersion: 'u32',
    apis: 'Vec<([u8;8],u32)>',
    transactionVersion: 'u32',
    stateVersion: 'u8'
  },
  /**
   * Lookup112: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup113: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup115: pallet_ticks::pallet::Call<T>
   **/
  PalletTicksCall: 'Null',
  /**
   * Lookup116: pallet_ticks::pallet::Error<T>
   **/
  PalletTicksError: 'Null',
  /**
   * Lookup119: ulx_primitives::block_seal::app::Public
   **/
  UlxPrimitivesBlockSealAppPublic: 'SpCoreEd25519Public',
  /**
   * Lookup125: pallet_mining_slot::pallet::Call<T>
   **/
  PalletMiningSlotCall: {
    _enum: {
      bid: {
        bondId: 'Option<u64>',
        rewardDestination: 'UlxPrimitivesBlockSealRewardDestination'
      }
    }
  },
  /**
   * Lookup126: pallet_mining_slot::pallet::Error<T>
   **/
  PalletMiningSlotError: {
    _enum: ['SlotNotTakingBids', 'TooManyBlockRegistrants', 'UnableToRotateAuthority', 'InsufficientOwnershipTokens', 'InsufficientBalanceForBid', 'BidTooLow', 'BadInternalState', 'RpcHostsAreRequired', 'BidBondDurationTooShort', 'CannotRegisteredOverlappingSessions', 'BadState', 'BondNotFound', 'NoMoreBondIds', 'BondFundClosed', 'MinimumBondAmountNotMet', 'LeaseUntilBlockTooSoon', 'LeaseUntilPastFundExpiration', 'ExpirationAtBlockOverflow', 'InsufficientFunds', 'InsufficientBondFunds', 'ExpirationTooSoon', 'NoPermissions', 'NoBondFundFound', 'HoldUnexpectedlyModified', 'BondFundMaximumBondsExceeded', 'UnrecoverableHold', 'BondFundNotFound', 'BondAlreadyClosed', 'BondAlreadyLocked', 'BondLockedCannotModify', 'FeeExceedsBondAmount', 'AccountWouldBeBelowMinimum']
  },
  /**
   * Lookup127: ulx_primitives::bond::BondFund<sp_core::crypto::AccountId32, Balance, BlockNumber>
   **/
  UlxPrimitivesBondBondFund: {
    leaseAnnualPercentRate: 'Compact<u32>',
    leaseBaseFee: 'Compact<u128>',
    offerAccountId: 'AccountId32',
    amountReserved: 'Compact<u128>',
    offerExpirationBlock: 'u32',
    amountBonded: 'Compact<u128>',
    isEnded: 'bool'
  },
  /**
   * Lookup130: ulx_primitives::bond::Bond<sp_core::crypto::AccountId32, Balance, BlockNumber, BondFundId>
   **/
  UlxPrimitivesBond: {
    bondFundId: 'Option<u32>',
    bondedAccountId: 'AccountId32',
    annualPercentRate: 'Compact<u32>',
    baseFee: 'Compact<u128>',
    fee: 'Compact<u128>',
    amount: 'Compact<u128>',
    startBlock: 'Compact<u32>',
    completionBlock: 'Compact<u32>',
    isLocked: 'bool'
  },
  /**
   * Lookup133: pallet_bond::pallet::Call<T>
   **/
  PalletBondCall: {
    _enum: {
      offer_fund: {
        leaseAnnualPercentRate: 'Compact<u32>',
        leaseBaseFee: 'Compact<u128>',
        amountOffered: 'Compact<u128>',
        expirationBlock: 'u32',
      },
      end_fund: {
        bondFundId: 'u32',
      },
      extend_fund: {
        bondFundId: 'u32',
        totalAmountOffered: 'u128',
        expirationBlock: 'u32',
      },
      bond_self: {
        amount: 'u128',
        bondUntilBlock: 'u32',
      },
      lease: {
        bondFundId: 'u32',
        amount: 'u128',
        leaseUntilBlock: 'u32',
      },
      return_bond: {
        bondId: 'u64',
      },
      extend_bond: {
        bondId: 'u64',
        totalAmount: 'u128',
        bondUntilBlock: 'u32'
      }
    }
  },
  /**
   * Lookup134: pallet_bond::pallet::Error<T>
   **/
  PalletBondError: {
    _enum: ['BadState', 'BondNotFound', 'NoMoreBondFundIds', 'NoMoreBondIds', 'MinimumBondAmountNotMet', 'ExpirationAtBlockOverflow', 'InsufficientFunds', 'InsufficientBondFunds', 'TransactionWouldTakeAccountBelowMinimumBalance', 'BondFundClosed', 'BondFundReductionExceedsAllocatedFunds', 'ExpirationTooSoon', 'LeaseUntilBlockTooSoon', 'LeaseUntilPastFundExpiration', 'NoPermissions', 'NoBondFundFound', 'FundExtensionMustBeLater', 'HoldUnexpectedlyModified', 'BondFundMaximumBondsExceeded', 'UnrecoverableHold', 'BondFundNotFound', 'BondAlreadyLocked', 'BondLockedCannotModify', 'FeeExceedsBondAmount']
  },
  /**
   * Lookup147: pallet_notaries::pallet::Call<T>
   **/
  PalletNotariesCall: {
    _enum: {
      propose: {
        meta: 'UlxPrimitivesNotaryNotaryMeta',
      },
      activate: {
        operatorAccount: 'AccountId32',
      },
      update: {
        notaryId: 'Compact<u32>',
        meta: 'UlxPrimitivesNotaryNotaryMeta'
      }
    }
  },
  /**
   * Lookup148: pallet_notaries::pallet::Error<T>
   **/
  PalletNotariesError: {
    _enum: ['ProposalNotFound', 'MaxNotariesExceeded', 'MaxProposalsPerBlockExceeded', 'NotAnActiveNotary', 'InvalidNotaryOperator', 'NoMoreNotaryIds']
  },
  /**
   * Lookup150: ulx_primitives::balance_change::AccountOrigin
   **/
  UlxPrimitivesBalanceChangeAccountOrigin: {
    notebookNumber: 'Compact<u32>',
    accountUid: 'Compact<u32>'
  },
  /**
   * Lookup153: ulx_primitives::notary::NotaryNotebookKeyDetails
   **/
  UlxPrimitivesNotaryNotaryNotebookKeyDetails: {
    notebookNumber: 'Compact<u32>',
    tick: 'Compact<u32>',
    blockVotesRoot: 'H256',
    secretHash: 'H256',
    parentSecret: 'Option<H256>'
  },
  /**
   * Lookup156: ulx_primitives::digests::NotebookDigest<ulx_notary_audit::error::VerifyError>
   **/
  UlxPrimitivesDigestsNotebookDigest: {
    notebooks: 'Vec<UlxPrimitivesDigestsNotebookDigestRecord>'
  },
  /**
   * Lookup158: ulx_primitives::digests::NotebookDigestRecord<ulx_notary_audit::error::VerifyError>
   **/
  UlxPrimitivesDigestsNotebookDigestRecord: {
    notaryId: 'Compact<u32>',
    notebookNumber: 'Compact<u32>',
    tick: 'Compact<u32>',
    auditFirstFailure: 'Option<UlxNotaryAuditErrorVerifyError>'
  },
  /**
   * Lookup161: pallet_notebook::pallet::Call<T>
   **/
  PalletNotebookCall: {
    _enum: {
      submit: {
        notebooks: 'Vec<UlxPrimitivesNotebookSignedNotebookHeader>'
      }
    }
  },
  /**
   * Lookup163: ulx_primitives::notebook::SignedNotebookHeader
   **/
  UlxPrimitivesNotebookSignedNotebookHeader: {
    header: 'UlxPrimitivesNotebookNotebookHeader',
    signature: 'SpCoreEd25519Signature'
  },
  /**
   * Lookup164: ulx_primitives::notebook::NotebookHeader
   **/
  UlxPrimitivesNotebookNotebookHeader: {
    version: 'Compact<u16>',
    notebookNumber: 'Compact<u32>',
    tick: 'Compact<u32>',
    finalizedBlockNumber: 'Compact<u32>',
    tax: 'Compact<u128>',
    notaryId: 'Compact<u32>',
    chainTransfers: 'Vec<UlxPrimitivesNotebookChainTransfer>',
    changedAccountsRoot: 'H256',
    changedAccountOrigins: 'Vec<UlxPrimitivesBalanceChangeAccountOrigin>',
    blockVotesRoot: 'H256',
    blockVotesCount: 'Compact<u32>',
    blocksWithVotes: 'Vec<H256>',
    blockVotingPower: 'Compact<u128>',
    secretHash: 'H256',
    parentSecret: 'Option<H256>',
    dataDomains: 'Vec<(UlxPrimitivesDataDomain,AccountId32)>'
  },
  /**
   * Lookup166: ulx_primitives::notebook::ChainTransfer
   **/
  UlxPrimitivesNotebookChainTransfer: {
    _enum: {
      ToMainchain: {
        accountId: 'AccountId32',
        amount: 'Compact<u128>',
      },
      ToLocalchain: {
        accountId: 'AccountId32',
        accountNonce: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup174: sp_core::ed25519::Signature
   **/
  SpCoreEd25519Signature: '[u8;64]',
  /**
   * Lookup176: pallet_notebook::pallet::Error<T>
   **/
  PalletNotebookError: {
    _enum: ['DuplicateNotebookNumber', 'MissingNotebookNumber', 'NotebookTickAlreadyUsed', 'InvalidNotebookSignature', 'InvalidSecretProvided', 'CouldNotDecodeNotebook', 'DuplicateNotebookDigest', 'MissingNotebookDigest', 'InvalidNotebookDigest', 'MultipleNotebookInherentsProvided', 'InternalError']
  },
  /**
   * Lookup178: pallet_chain_transfer::QueuedTransferOut<Balance, BlockNumber>
   **/
  PalletChainTransferQueuedTransferOut: {
    amount: 'u128',
    expirationBlock: 'u32',
    notaryId: 'u32'
  },
  /**
   * Lookup181: pallet_chain_transfer::pallet::Call<T>
   **/
  PalletChainTransferCall: {
    _enum: {
      send_to_localchain: {
        amount: 'Compact<u128>',
        notaryId: 'u32'
      }
    }
  },
  /**
   * Lookup182: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup183: pallet_chain_transfer::pallet::Error<T>
   **/
  PalletChainTransferError: {
    _enum: ['MaxBlockTransfersExceeded', 'InsufficientFunds', 'InvalidAccountNonce', 'InsufficientNotarizedFunds', 'InvalidOrDuplicatedLocalchainTransfer', 'NotebookIncludesExpiredLocalchainTransfer', 'InvalidNotaryUsedForTransfer']
  },
  /**
   * Lookup188: ulx_primitives::notary::NotaryNotebookVoteDigestDetails
   **/
  UlxPrimitivesNotaryNotaryNotebookVoteDigestDetails: {
    notaryId: 'Compact<u32>',
    notebookNumber: 'Compact<u32>',
    tick: 'Compact<u32>',
    blockVotesCount: 'Compact<u32>',
    blockVotingPower: 'Compact<u128>'
  },
  /**
   * Lookup190: ulx_primitives::digests::BlockVoteDigest
   **/
  UlxPrimitivesDigestsBlockVoteDigest: {
    votingPower: 'Compact<u128>',
    votesCount: 'Compact<u32>'
  },
  /**
   * Lookup194: pallet_block_seal_spec::pallet::Call<T>
   **/
  PalletBlockSealSpecCall: {
    _enum: {
      configure: {
        voteMinimum: 'Option<u128>',
        computeDifficulty: 'Option<u128>'
      }
    }
  },
  /**
   * Lookup196: pallet_block_seal_spec::pallet::Error<T>
   **/
  PalletBlockSealSpecError: {
    _enum: ['MaxNotebooksAtTickExceeded']
  },
  /**
   * Lookup200: pallet_data_domain::pallet::Call<T>
   **/
  PalletDataDomainCall: {
    _enum: {
      set_zone_record: {
        domain: 'UlxPrimitivesDataDomain',
        zoneRecord: 'UlxPrimitivesDataDomainZoneRecord'
      }
    }
  },
  /**
   * Lookup201: pallet_data_domain::pallet::Error<T>
   **/
  PalletDataDomainError: {
    _enum: ['DomainNotRegistered', 'NotDomainOwner']
  },
  /**
   * Lookup205: ulx_node_runtime::opaque::SessionKeys
   **/
  UlxNodeRuntimeOpaqueSessionKeys: {
    grandpa: 'SpConsensusGrandpaAppPublic',
    blockSealAuthority: 'UlxPrimitivesBlockSealAppPublic'
  },
  /**
   * Lookup207: sp_core::crypto::KeyTypeId
   **/
  SpCoreCryptoKeyTypeId: '[u8;4]',
  /**
   * Lookup208: pallet_session::pallet::Call<T>
   **/
  PalletSessionCall: {
    _enum: {
      set_keys: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'UlxNodeRuntimeOpaqueSessionKeys',
        proof: 'Bytes',
      },
      purge_keys: 'Null'
    }
  },
  /**
   * Lookup209: pallet_session::pallet::Error<T>
   **/
  PalletSessionError: {
    _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount']
  },
  /**
   * Lookup210: ulx_primitives::providers::BlockSealerInfo<sp_core::crypto::AccountId32>
   **/
  UlxPrimitivesProvidersBlockSealerInfo: {
    minerRewardsAccount: 'AccountId32',
    blockVoteRewardsAccount: 'AccountId32'
  },
  /**
   * Lookup211: ulx_primitives::inherents::BlockSealInherent
   **/
  UlxPrimitivesInherentsBlockSealInherent: {
    _enum: {
      Vote: {
        sealStrength: 'U256',
        notaryId: 'Compact<u32>',
        sourceNotebookNumber: 'Compact<u32>',
        sourceNotebookProof: 'UlxPrimitivesBalanceChangeMerkleProof',
        blockVote: 'UlxPrimitivesBlockVoteBlockVoteT',
        minerSignature: 'UlxPrimitivesBlockSealAppSignature',
      },
      Compute: 'Null'
    }
  },
  /**
   * Lookup212: ulx_primitives::balance_change::MerkleProof
   **/
  UlxPrimitivesBalanceChangeMerkleProof: {
    proof: 'Vec<H256>',
    numberOfLeaves: 'Compact<u32>',
    leafIndex: 'Compact<u32>'
  },
  /**
   * Lookup214: ulx_primitives::block_vote::BlockVoteT<primitive_types::H256>
   **/
  UlxPrimitivesBlockVoteBlockVoteT: {
    accountId: 'AccountId32',
    blockHash: 'H256',
    index: 'Compact<u32>',
    power: 'Compact<u128>',
    dataDomain: 'UlxPrimitivesDataDomain',
    dataDomainAccount: 'AccountId32',
    signature: 'SpRuntimeMultiSignature'
  },
  /**
   * Lookup215: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: 'SpCoreEd25519Signature',
      Sr25519: 'SpCoreSr25519Signature',
      Ecdsa: 'SpCoreEcdsaSignature'
    }
  },
  /**
   * Lookup216: sp_core::sr25519::Signature
   **/
  SpCoreSr25519Signature: '[u8;64]',
  /**
   * Lookup217: sp_core::ecdsa::Signature
   **/
  SpCoreEcdsaSignature: '[u8;65]',
  /**
   * Lookup219: ulx_primitives::block_seal::app::Signature
   **/
  UlxPrimitivesBlockSealAppSignature: 'SpCoreEd25519Signature',
  /**
   * Lookup220: ulx_primitives::digests::ParentVotingKeyDigest
   **/
  UlxPrimitivesDigestsParentVotingKeyDigest: {
    parentVotingKey: 'Option<H256>'
  },
  /**
   * Lookup221: pallet_block_seal::pallet::Call<T>
   **/
  PalletBlockSealCall: {
    _enum: {
      apply: {
        seal: 'UlxPrimitivesInherentsBlockSealInherent'
      }
    }
  },
  /**
   * Lookup222: pallet_block_seal::pallet::Error<T>
   **/
  PalletBlockSealError: {
    _enum: ['InvalidVoteSealStrength', 'InvalidSubmitter', 'UnableToDecodeVoteAccount', 'UnregisteredBlockAuthor', 'InvalidBlockVoteProof', 'NoGrandparentVoteMinimum', 'DuplicateBlockSealProvided', 'InsufficientVotingPower', 'ParentVotingKeyNotFound', 'InvalidVoteGrandparentHash', 'IneligibleNotebookUsed', 'NoEligibleVotingRoot', 'UnregisteredDataDomain', 'InvalidDataDomainAccount', 'InvalidAuthoritySignature', 'CouldNotDecodeVote', 'MaxNotebooksAtTickExceeded', 'NoClosestMinerFoundForVote', 'BlockVoteInvalidSignature']
  },
  /**
   * Lookup224: pallet_block_rewards::pallet::Call<T>
   **/
  PalletBlockRewardsCall: 'Null',
  /**
   * Lookup225: pallet_block_rewards::pallet::Error<T>
   **/
  PalletBlockRewardsError: 'Null',
  /**
   * Lookup226: pallet_grandpa::StoredState<N>
   **/
  PalletGrandpaStoredState: {
    _enum: {
      Live: 'Null',
      PendingPause: {
        scheduledAt: 'u32',
        delay: 'u32',
      },
      Paused: 'Null',
      PendingResume: {
        scheduledAt: 'u32',
        delay: 'u32'
      }
    }
  },
  /**
   * Lookup227: pallet_grandpa::StoredPendingChange<N, Limit>
   **/
  PalletGrandpaStoredPendingChange: {
    scheduledAt: 'u32',
    delay: 'u32',
    nextAuthorities: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
    forced: 'Option<u32>'
  },
  /**
   * Lookup229: pallet_grandpa::pallet::Call<T>
   **/
  PalletGrandpaCall: {
    _enum: {
      report_equivocation: {
        equivocationProof: 'SpConsensusGrandpaEquivocationProof',
        keyOwnerProof: 'SpSessionMembershipProof',
      },
      report_equivocation_unsigned: {
        equivocationProof: 'SpConsensusGrandpaEquivocationProof',
        keyOwnerProof: 'SpSessionMembershipProof',
      },
      note_stalled: {
        delay: 'u32',
        bestFinalizedBlockNumber: 'u32'
      }
    }
  },
  /**
   * Lookup230: sp_consensus_grandpa::EquivocationProof<primitive_types::H256, N>
   **/
  SpConsensusGrandpaEquivocationProof: {
    setId: 'u64',
    equivocation: 'SpConsensusGrandpaEquivocation'
  },
  /**
   * Lookup231: sp_consensus_grandpa::Equivocation<primitive_types::H256, N>
   **/
  SpConsensusGrandpaEquivocation: {
    _enum: {
      Prevote: 'FinalityGrandpaEquivocationPrevote',
      Precommit: 'FinalityGrandpaEquivocationPrecommit'
    }
  },
  /**
   * Lookup232: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: 'u64',
    identity: 'SpConsensusGrandpaAppPublic',
    first: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)',
    second: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)'
  },
  /**
   * Lookup233: finality_grandpa::Prevote<primitive_types::H256, N>
   **/
  FinalityGrandpaPrevote: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup234: sp_consensus_grandpa::app::Signature
   **/
  SpConsensusGrandpaAppSignature: 'SpCoreEd25519Signature',
  /**
   * Lookup236: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: 'u64',
    identity: 'SpConsensusGrandpaAppPublic',
    first: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)',
    second: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)'
  },
  /**
   * Lookup237: finality_grandpa::Precommit<primitive_types::H256, N>
   **/
  FinalityGrandpaPrecommit: {
    targetHash: 'H256',
    targetNumber: 'u32'
  },
  /**
   * Lookup239: sp_session::MembershipProof
   **/
  SpSessionMembershipProof: {
    session: 'u32',
    trieNodes: 'Vec<Bytes>',
    validatorCount: 'u32'
  },
  /**
   * Lookup240: pallet_grandpa::pallet::Error<T>
   **/
  PalletGrandpaError: {
    _enum: ['PauseFailed', 'ResumeFailed', 'ChangePending', 'TooSoon', 'InvalidKeyOwnershipProof', 'InvalidEquivocationProof', 'DuplicateOffenceReport']
  },
  /**
   * Lookup241: sp_staking::offence::OffenceDetails<sp_core::crypto::AccountId32, Offender>
   **/
  SpStakingOffenceOffenceDetails: {
    offender: '(AccountId32,PalletMiningSlotMinerHistory)',
    reporters: 'Vec<AccountId32>'
  },
  /**
   * Lookup243: pallet_mining_slot::MinerHistory
   **/
  PalletMiningSlotMinerHistory: {
    authorityIndex: 'u32'
  },
  /**
   * Lookup246: pallet_balances::types::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup247: pallet_balances::types::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup250: pallet_balances::types::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup253: pallet_balances::types::IdAmount<ulx_node_runtime::RuntimeHoldReason, Balance>
   **/
  PalletBalancesIdAmountRuntimeHoldReason: {
    id: 'UlxNodeRuntimeRuntimeHoldReason',
    amount: 'u128'
  },
  /**
   * Lookup254: ulx_node_runtime::RuntimeHoldReason
   **/
  UlxNodeRuntimeRuntimeHoldReason: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      MiningSlot: 'PalletMiningSlotHoldReason',
      Bond: 'PalletBondHoldReason',
      __Unused5: 'Null',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      BlockRewards: 'PalletBlockRewardsHoldReason',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      Mint: 'PalletMintHoldReason'
    }
  },
  /**
   * Lookup255: pallet_mining_slot::pallet::HoldReason
   **/
  PalletMiningSlotHoldReason: {
    _enum: ['RegisterAsMiner']
  },
  /**
   * Lookup256: pallet_bond::pallet::HoldReason
   **/
  PalletBondHoldReason: {
    _enum: ['EnterBondFund']
  },
  /**
   * Lookup257: pallet_block_rewards::pallet::HoldReason
   **/
  PalletBlockRewardsHoldReason: {
    _enum: ['MaturationPeriod']
  },
  /**
   * Lookup258: pallet_mint::pallet::HoldReason
   **/
  PalletMintHoldReason: 'Null',
  /**
   * Lookup261: pallet_balances::types::IdAmount<ulx_node_runtime::RuntimeFreezeReason, Balance>
   **/
  PalletBalancesIdAmountRuntimeFreezeReason: {
    id: 'UlxNodeRuntimeRuntimeFreezeReason',
    amount: 'u128'
  },
  /**
   * Lookup262: ulx_node_runtime::RuntimeFreezeReason
   **/
  UlxNodeRuntimeRuntimeFreezeReason: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      __Unused4: 'Null',
      __Unused5: 'Null',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      BlockRewards: 'PalletBlockRewardsFreezeReason'
    }
  },
  /**
   * Lookup263: pallet_block_rewards::pallet::FreezeReason
   **/
  PalletBlockRewardsFreezeReason: {
    _enum: ['MaturationPeriod']
  },
  /**
   * Lookup265: pallet_balances::pallet::Call<T, I>
   **/
  PalletBalancesCall: {
    _enum: {
      transfer_allow_death: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      __Unused1: 'Null',
      force_transfer: {
        source: 'MultiAddress',
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_keep_alive: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      transfer_all: {
        dest: 'MultiAddress',
        keepAlive: 'bool',
      },
      force_unreserve: {
        who: 'MultiAddress',
        amount: 'u128',
      },
      upgrade_accounts: {
        who: 'Vec<AccountId32>',
      },
      __Unused7: 'Null',
      force_set_balance: {
        who: 'MultiAddress',
        newFree: 'Compact<u128>'
      }
    }
  },
  /**
   * Lookup269: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'Expendability', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves', 'TooManyHolds', 'TooManyFreezes']
  },
  /**
   * Lookup270: pallet_mint::pallet::Call<T>
   **/
  PalletMintCall: 'Null',
  /**
   * Lookup271: pallet_mint::pallet::Error<T>
   **/
  PalletMintError: 'Null',
  /**
   * Lookup275: pallet_tx_pause::pallet::Call<T>
   **/
  PalletTxPauseCall: {
    _enum: {
      pause: {
        fullName: '(Bytes,Bytes)',
      },
      unpause: {
        ident: '(Bytes,Bytes)'
      }
    }
  },
  /**
   * Lookup276: pallet_tx_pause::pallet::Error<T>
   **/
  PalletTxPauseError: {
    _enum: ['IsPaused', 'IsUnpaused', 'Unpausable', 'NotFound']
  },
  /**
   * Lookup278: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup279: pallet_sudo::pallet::Call<T>
   **/
  PalletSudoCall: {
    _enum: {
      sudo: {
        call: 'Call',
      },
      sudo_unchecked_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight',
      },
      set_key: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
      },
      sudo_as: {
        who: 'MultiAddress',
        call: 'Call',
      },
      remove_key: 'Null'
    }
  },
  /**
   * Lookup281: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup284: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup285: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup286: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup287: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup290: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup291: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup292: pallet_transaction_payment::ChargeTransactionPayment<T>
   **/
  PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
  /**
   * Lookup293: ulx_node_runtime::Runtime
   **/
  UlxNodeRuntimeRuntime: 'Null'
};
