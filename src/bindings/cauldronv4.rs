pub use cauldron_v4::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod cauldron_v4 {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "CauldronV4 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IBentoBoxV1\",\n        \"name\": \"bentoBox_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"magicInternetMoney_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"accruedAmount\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"LogAccrue\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogAddCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"part\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogBorrow\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"blacklisted\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"LogChangeBlacklistedCallee\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"newLimit\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"perAddressPart\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"LogChangeBorrowLimit\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"rate\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogExchangeRate\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newFeeTo\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LogFeeTo\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"oldInterestRate\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"newInterestRate\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"name\": \"LogInterestChange\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"collateralShare\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowPart\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogLiquidation\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogRemoveCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"part\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogRepay\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"previousElastic\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"newElastic\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"LogRepayForAll\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"feeTo\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"feesEarnedFraction\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogWithdrawFees\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"BORROW_OPENING_FEE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"COLLATERIZATION_RATE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"LIQUIDATION_MULTIPLIER\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"accrue\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"accrueInfo\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"lastAccrued\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"feesEarned\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"INTEREST_PER_SECOND\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"skim\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"addCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"bentoBox\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IBentoBoxV1\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"blacklistedCallees\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"borrow\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"part\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"borrowLimit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"total\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"borrowPartPerAddress\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"newBorrowLimit\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"perAddressPart\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"changeBorrowLimit\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"newInterestRate\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"name\": \"changeInterestRate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"claimOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"collateral\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint8[]\",\n        \"name\": \"actions\",\n        \"type\": \"uint8[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"values\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"bytes[]\",\n        \"name\": \"datas\",\n        \"type\": \"bytes[]\"\n      }\n    ],\n    \"name\": \"cook\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value1\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value2\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"exchangeRate\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeTo\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"init\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"users\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"maxBorrowParts\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract ISwapperV2\",\n        \"name\": \"swapper\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"swapperData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"liquidate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"magicInternetMoney\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"masterContract\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract CauldronV4\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"oracle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IOracle\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"oracleData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"pendingOwner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"reduceSupply\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"removeCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"skim\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"part\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"repay\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"skim\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"repayForAll\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callee\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"blacklisted\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setBlacklistedCallee\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newFeeTo\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setFeeTo\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalBorrow\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"elastic\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"base\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalCollateralShare\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"direct\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"renounce\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"updateExchangeRate\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"updated\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"rate\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"userBorrowPart\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"userCollateralShare\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"withdrawFees\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static CAULDRONV4_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct CauldronV4<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CauldronV4<M> {
        fn clone(&self) -> Self {
            CauldronV4(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CauldronV4<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for CauldronV4<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CauldronV4)).field(&self.address()).finish()
        }
    }
    impl<M: ethers::providers::Middleware> CauldronV4<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CAULDRONV4_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `BORROW_OPENING_FEE` (0xaba024f4) function"]
        pub fn borrow_opening_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([171, 160, 36, 244], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `COLLATERIZATION_RATE` (0xc7ee2a7b) function"]
        pub fn collaterization_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([199, 238, 42, 123], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `LIQUIDATION_MULTIPLIER` (0x6ec097fb) function"]
        pub fn liquidation_multiplier(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 192, 151, 251], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accrue` (0xf8ba4cff) function"]
        pub fn accrue(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 186, 76, 255], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accrueInfo` (0xb27c0e74) function"]
        pub fn accrue_info(&self) -> ethers::contract::builders::ContractCall<M, (u64, u128, u64)> {
            self.0
                .method_hash([178, 124, 14, 116], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addCollateral` (0x860ffea1) function"]
        pub fn add_collateral(
            &self,
            to: ethers::core::types::Address,
            skim: bool,
            share: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 15, 254, 161], (to, skim, share))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bentoBox` (0x6b2ace87) function"]
        pub fn bento_box(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([107, 42, 206, 135], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blacklistedCallees` (0xcb0dc548) function"]
        pub fn blacklisted_callees(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([203, 13, 197, 72], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrow` (0x4b8a3529) function"]
        pub fn borrow(
            &self,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([75, 138, 53, 41], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowLimit` (0xe551d11d) function"]
        pub fn borrow_limit(&self) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([229, 81, 209, 29], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeBorrowLimit` (0xf7dad434) function"]
        pub fn change_borrow_limit(
            &self,
            new_borrow_limit: u128,
            per_address_part: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 218, 212, 52], (new_borrow_limit, per_address_part))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeInterestRate` (0x1cd4c966) function"]
        pub fn change_interest_rate(
            &self,
            new_interest_rate: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 212, 201, 102], new_interest_rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimOwnership` (0x4e71e0c8) function"]
        pub fn claim_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 113, 224, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collateral` (0xd8dfeb45) function"]
        pub fn collateral(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([216, 223, 235, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cook` (0x656f3d64) function"]
        pub fn cook(
            &self,
            actions: ::std::vec::Vec<u8>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            datas: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([101, 111, 61, 100], (actions, values, datas))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchangeRate` (0x3ba0b9a9) function"]
        pub fn exchange_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([59, 160, 185, 169], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeTo` (0x017e7e58) function"]
        pub fn fee_to(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([1, 126, 126, 88], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `init` (0x4ddf47d4) function"]
        pub fn init(
            &self,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 223, 71, 212], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidate` (0xff6ff84b) function"]
        pub fn liquidate(
            &self,
            users: ::std::vec::Vec<ethers::core::types::Address>,
            max_borrow_parts: ::std::vec::Vec<ethers::core::types::U256>,
            to: ethers::core::types::Address,
            swapper: ethers::core::types::Address,
            swapper_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [255, 111, 248, 75],
                    (users, max_borrow_parts, to, swapper, swapper_data),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `magicInternetMoney` (0x9b352ae1) function"]
        pub fn magic_internet_money(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([155, 53, 42, 225], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `masterContract` (0xcd446e22) function"]
        pub fn master_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([205, 68, 110, 34], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracleData` (0x74645ff3) function"]
        pub fn oracle_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([116, 100, 95, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingOwner` (0xe30c3978) function"]
        pub fn pending_owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `reduceSupply` (0x80623444) function"]
        pub fn reduce_supply(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 98, 52, 68], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeCollateral` (0x876467f8) function"]
        pub fn remove_collateral(
            &self,
            to: ethers::core::types::Address,
            share: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 100, 103, 248], (to, share))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repay` (0x15294c40) function"]
        pub fn repay(
            &self,
            to: ethers::core::types::Address,
            skim: bool,
            part: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([21, 41, 76, 64], (to, skim, part))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayForAll` (0x7ef40455) function"]
        pub fn repay_for_all(
            &self,
            amount: u128,
            skim: bool,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([126, 244, 4, 85], (amount, skim))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBlacklistedCallee` (0xeeae797b) function"]
        pub fn set_blacklisted_callee(
            &self,
            callee: ethers::core::types::Address,
            blacklisted: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 174, 121, 123], (callee, blacklisted))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeTo` (0xf46901ed) function"]
        pub fn set_fee_to(
            &self,
            new_fee_to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 105, 1, 237], new_fee_to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalBorrow` (0x8285ef40) function"]
        pub fn total_borrow(&self) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([130, 133, 239, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalCollateralShare` (0x473e3ce7) function"]
        pub fn total_collateral_share(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([71, 62, 60, 231], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0x078dfbe7) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
            direct: bool,
            renounce: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 141, 251, 231], (new_owner, direct, renounce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateExchangeRate` (0x02ce728f) function"]
        pub fn update_exchange_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::U256)>
        {
            self.0
                .method_hash([2, 206, 114, 143], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `userBorrowPart` (0x48e4163e) function"]
        pub fn user_borrow_part(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([72, 228, 22, 62], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `userCollateralShare` (0x1c9e379b) function"]
        pub fn user_collateral_share(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 158, 55, 155], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFees` (0x476343ee) function"]
        pub fn withdraw_fees(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 99, 67, 238], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `LogAccrue` event"]
        pub fn log_accrue_filter(&self) -> ethers::contract::builders::Event<M, LogAccrueFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogAddCollateral` event"]
        pub fn log_add_collateral_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogAddCollateralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogBorrow` event"]
        pub fn log_borrow_filter(&self) -> ethers::contract::builders::Event<M, LogBorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogChangeBlacklistedCallee` event"]
        pub fn log_change_blacklisted_callee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogChangeBlacklistedCalleeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogChangeBorrowLimit` event"]
        pub fn log_change_borrow_limit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogChangeBorrowLimitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogExchangeRate` event"]
        pub fn log_exchange_rate_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogExchangeRateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogFeeTo` event"]
        pub fn log_fee_to_filter(&self) -> ethers::contract::builders::Event<M, LogFeeToFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogInterestChange` event"]
        pub fn log_interest_change_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogInterestChangeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogLiquidation` event"]
        pub fn log_liquidation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogLiquidationFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogRemoveCollateral` event"]
        pub fn log_remove_collateral_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogRemoveCollateralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogRepay` event"]
        pub fn log_repay_filter(&self) -> ethers::contract::builders::Event<M, LogRepayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogRepayForAll` event"]
        pub fn log_repay_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogRepayForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogWithdrawFees` event"]
        pub fn log_withdraw_fees_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogWithdrawFeesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CauldronV4Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CauldronV4<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogAccrue", abi = "LogAccrue(uint128)")]
    pub struct LogAccrueFilter {
        pub accrued_amount: u128,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogAddCollateral", abi = "LogAddCollateral(address,address,uint256)")]
    pub struct LogAddCollateralFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub share: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogBorrow", abi = "LogBorrow(address,address,uint256,uint256)")]
    pub struct LogBorrowFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub part: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "LogChangeBlacklistedCallee",
        abi = "LogChangeBlacklistedCallee(address,bool)"
    )]
    pub struct LogChangeBlacklistedCalleeFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub blacklisted: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogChangeBorrowLimit", abi = "LogChangeBorrowLimit(uint128,uint128)")]
    pub struct LogChangeBorrowLimitFilter {
        pub new_limit: u128,
        pub per_address_part: u128,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogExchangeRate", abi = "LogExchangeRate(uint256)")]
    pub struct LogExchangeRateFilter {
        pub rate: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogFeeTo", abi = "LogFeeTo(address)")]
    pub struct LogFeeToFilter {
        #[ethevent(indexed)]
        pub new_fee_to: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogInterestChange", abi = "LogInterestChange(uint64,uint64)")]
    pub struct LogInterestChangeFilter {
        pub old_interest_rate: u64,
        pub new_interest_rate: u64,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "LogLiquidation",
        abi = "LogLiquidation(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct LogLiquidationFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub collateral_share: ethers::core::types::U256,
        pub borrow_amount: ethers::core::types::U256,
        pub borrow_part: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogRemoveCollateral", abi = "LogRemoveCollateral(address,address,uint256)")]
    pub struct LogRemoveCollateralFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub share: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogRepay", abi = "LogRepay(address,address,uint256,uint256)")]
    pub struct LogRepayFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub part: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogRepayForAll", abi = "LogRepayForAll(uint256,uint128,uint128)")]
    pub struct LogRepayForAllFilter {
        pub amount: ethers::core::types::U256,
        pub previous_elastic: u128,
        pub new_elastic: u128,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogWithdrawFees", abi = "LogWithdrawFees(address,uint256)")]
    pub struct LogWithdrawFeesFilter {
        #[ethevent(indexed)]
        pub fee_to: ethers::core::types::Address,
        pub fees_earned_fraction: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "OwnershipTransferred", abi = "OwnershipTransferred(address,address)")]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CauldronV4Events {
        LogAccrueFilter(LogAccrueFilter),
        LogAddCollateralFilter(LogAddCollateralFilter),
        LogBorrowFilter(LogBorrowFilter),
        LogChangeBlacklistedCalleeFilter(LogChangeBlacklistedCalleeFilter),
        LogChangeBorrowLimitFilter(LogChangeBorrowLimitFilter),
        LogExchangeRateFilter(LogExchangeRateFilter),
        LogFeeToFilter(LogFeeToFilter),
        LogInterestChangeFilter(LogInterestChangeFilter),
        LogLiquidationFilter(LogLiquidationFilter),
        LogRemoveCollateralFilter(LogRemoveCollateralFilter),
        LogRepayFilter(LogRepayFilter),
        LogRepayForAllFilter(LogRepayForAllFilter),
        LogWithdrawFeesFilter(LogWithdrawFeesFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for CauldronV4Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogAccrueFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogAccrueFilter(decoded));
            }
            if let Ok(decoded) = LogAddCollateralFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogAddCollateralFilter(decoded));
            }
            if let Ok(decoded) = LogBorrowFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogBorrowFilter(decoded));
            }
            if let Ok(decoded) = LogChangeBlacklistedCalleeFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogChangeBlacklistedCalleeFilter(decoded));
            }
            if let Ok(decoded) = LogChangeBorrowLimitFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogChangeBorrowLimitFilter(decoded));
            }
            if let Ok(decoded) = LogExchangeRateFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogExchangeRateFilter(decoded));
            }
            if let Ok(decoded) = LogFeeToFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogFeeToFilter(decoded));
            }
            if let Ok(decoded) = LogInterestChangeFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogInterestChangeFilter(decoded));
            }
            if let Ok(decoded) = LogLiquidationFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogLiquidationFilter(decoded));
            }
            if let Ok(decoded) = LogRemoveCollateralFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogRemoveCollateralFilter(decoded));
            }
            if let Ok(decoded) = LogRepayFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogRepayFilter(decoded));
            }
            if let Ok(decoded) = LogRepayForAllFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogRepayForAllFilter(decoded));
            }
            if let Ok(decoded) = LogWithdrawFeesFilter::decode_log(log) {
                return Ok(CauldronV4Events::LogWithdrawFeesFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CauldronV4Events::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CauldronV4Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CauldronV4Events::LogAccrueFilter(element) => element.fmt(f),
                CauldronV4Events::LogAddCollateralFilter(element) => element.fmt(f),
                CauldronV4Events::LogBorrowFilter(element) => element.fmt(f),
                CauldronV4Events::LogChangeBlacklistedCalleeFilter(element) => element.fmt(f),
                CauldronV4Events::LogChangeBorrowLimitFilter(element) => element.fmt(f),
                CauldronV4Events::LogExchangeRateFilter(element) => element.fmt(f),
                CauldronV4Events::LogFeeToFilter(element) => element.fmt(f),
                CauldronV4Events::LogInterestChangeFilter(element) => element.fmt(f),
                CauldronV4Events::LogLiquidationFilter(element) => element.fmt(f),
                CauldronV4Events::LogRemoveCollateralFilter(element) => element.fmt(f),
                CauldronV4Events::LogRepayFilter(element) => element.fmt(f),
                CauldronV4Events::LogRepayForAllFilter(element) => element.fmt(f),
                CauldronV4Events::LogWithdrawFeesFilter(element) => element.fmt(f),
                CauldronV4Events::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `BORROW_OPENING_FEE` function with signature `BORROW_OPENING_FEE()` and selector `[171, 160, 36, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "BORROW_OPENING_FEE", abi = "BORROW_OPENING_FEE()")]
    pub struct BorrowOpeningFeeCall;
    #[doc = "Container type for all input parameters for the `COLLATERIZATION_RATE` function with signature `COLLATERIZATION_RATE()` and selector `[199, 238, 42, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "COLLATERIZATION_RATE", abi = "COLLATERIZATION_RATE()")]
    pub struct CollaterizationRateCall;
    #[doc = "Container type for all input parameters for the `LIQUIDATION_MULTIPLIER` function with signature `LIQUIDATION_MULTIPLIER()` and selector `[110, 192, 151, 251]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "LIQUIDATION_MULTIPLIER", abi = "LIQUIDATION_MULTIPLIER()")]
    pub struct LiquidationMultiplierCall;
    #[doc = "Container type for all input parameters for the `accrue` function with signature `accrue()` and selector `[248, 186, 76, 255]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "accrue", abi = "accrue()")]
    pub struct AccrueCall;
    #[doc = "Container type for all input parameters for the `accrueInfo` function with signature `accrueInfo()` and selector `[178, 124, 14, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "accrueInfo", abi = "accrueInfo()")]
    pub struct AccrueInfoCall;
    #[doc = "Container type for all input parameters for the `addCollateral` function with signature `addCollateral(address,bool,uint256)` and selector `[134, 15, 254, 161]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addCollateral", abi = "addCollateral(address,bool,uint256)")]
    pub struct AddCollateralCall {
        pub to: ethers::core::types::Address,
        pub skim: bool,
        pub share: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `bentoBox` function with signature `bentoBox()` and selector `[107, 42, 206, 135]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bentoBox", abi = "bentoBox()")]
    pub struct BentoBoxCall;
    #[doc = "Container type for all input parameters for the `blacklistedCallees` function with signature `blacklistedCallees(address)` and selector `[203, 13, 197, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "blacklistedCallees", abi = "blacklistedCallees(address)")]
    pub struct BlacklistedCalleesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `borrow` function with signature `borrow(address,uint256)` and selector `[75, 138, 53, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "borrow", abi = "borrow(address,uint256)")]
    pub struct BorrowCall {
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `borrowLimit` function with signature `borrowLimit()` and selector `[229, 81, 209, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "borrowLimit", abi = "borrowLimit()")]
    pub struct BorrowLimitCall;
    #[doc = "Container type for all input parameters for the `changeBorrowLimit` function with signature `changeBorrowLimit(uint128,uint128)` and selector `[247, 218, 212, 52]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "changeBorrowLimit", abi = "changeBorrowLimit(uint128,uint128)")]
    pub struct ChangeBorrowLimitCall {
        pub new_borrow_limit: u128,
        pub per_address_part: u128,
    }
    #[doc = "Container type for all input parameters for the `changeInterestRate` function with signature `changeInterestRate(uint64)` and selector `[28, 212, 201, 102]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "changeInterestRate", abi = "changeInterestRate(uint64)")]
    pub struct ChangeInterestRateCall {
        pub new_interest_rate: u64,
    }
    #[doc = "Container type for all input parameters for the `claimOwnership` function with signature `claimOwnership()` and selector `[78, 113, 224, 200]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "claimOwnership", abi = "claimOwnership()")]
    pub struct ClaimOwnershipCall;
    #[doc = "Container type for all input parameters for the `collateral` function with signature `collateral()` and selector `[216, 223, 235, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "collateral", abi = "collateral()")]
    pub struct CollateralCall;
    #[doc = "Container type for all input parameters for the `cook` function with signature `cook(uint8[],uint256[],bytes[])` and selector `[101, 111, 61, 100]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cook", abi = "cook(uint8[],uint256[],bytes[])")]
    pub struct CookCall {
        pub actions: ::std::vec::Vec<u8>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub datas: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `exchangeRate` function with signature `exchangeRate()` and selector `[59, 160, 185, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "exchangeRate", abi = "exchangeRate()")]
    pub struct ExchangeRateCall;
    #[doc = "Container type for all input parameters for the `feeTo` function with signature `feeTo()` and selector `[1, 126, 126, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeTo", abi = "feeTo()")]
    pub struct FeeToCall;
    #[doc = "Container type for all input parameters for the `init` function with signature `init(bytes)` and selector `[77, 223, 71, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "init", abi = "init(bytes)")]
    pub struct InitCall {
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `liquidate` function with signature `liquidate(address[],uint256[],address,address,bytes)` and selector `[255, 111, 248, 75]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "liquidate", abi = "liquidate(address[],uint256[],address,address,bytes)")]
    pub struct LiquidateCall {
        pub users: ::std::vec::Vec<ethers::core::types::Address>,
        pub max_borrow_parts: ::std::vec::Vec<ethers::core::types::U256>,
        pub to: ethers::core::types::Address,
        pub swapper: ethers::core::types::Address,
        pub swapper_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `magicInternetMoney` function with signature `magicInternetMoney()` and selector `[155, 53, 42, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "magicInternetMoney", abi = "magicInternetMoney()")]
    pub struct MagicInternetMoneyCall;
    #[doc = "Container type for all input parameters for the `masterContract` function with signature `masterContract()` and selector `[205, 68, 110, 34]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "masterContract", abi = "masterContract()")]
    pub struct MasterContractCall;
    #[doc = "Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    #[doc = "Container type for all input parameters for the `oracleData` function with signature `oracleData()` and selector `[116, 100, 95, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "oracleData", abi = "oracleData()")]
    pub struct OracleDataCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `[227, 12, 57, 120]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    #[doc = "Container type for all input parameters for the `reduceSupply` function with signature `reduceSupply(uint256)` and selector `[128, 98, 52, 68]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "reduceSupply", abi = "reduceSupply(uint256)")]
    pub struct ReduceSupplyCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeCollateral` function with signature `removeCollateral(address,uint256)` and selector `[135, 100, 103, 248]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeCollateral", abi = "removeCollateral(address,uint256)")]
    pub struct RemoveCollateralCall {
        pub to: ethers::core::types::Address,
        pub share: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `repay` function with signature `repay(address,bool,uint256)` and selector `[21, 41, 76, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "repay", abi = "repay(address,bool,uint256)")]
    pub struct RepayCall {
        pub to: ethers::core::types::Address,
        pub skim: bool,
        pub part: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `repayForAll` function with signature `repayForAll(uint128,bool)` and selector `[126, 244, 4, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "repayForAll", abi = "repayForAll(uint128,bool)")]
    pub struct RepayForAllCall {
        pub amount: u128,
        pub skim: bool,
    }
    #[doc = "Container type for all input parameters for the `setBlacklistedCallee` function with signature `setBlacklistedCallee(address,bool)` and selector `[238, 174, 121, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setBlacklistedCallee", abi = "setBlacklistedCallee(address,bool)")]
    pub struct SetBlacklistedCalleeCall {
        pub callee: ethers::core::types::Address,
        pub blacklisted: bool,
    }
    #[doc = "Container type for all input parameters for the `setFeeTo` function with signature `setFeeTo(address)` and selector `[244, 105, 1, 237]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeTo", abi = "setFeeTo(address)")]
    pub struct SetFeeToCall {
        pub new_fee_to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `totalBorrow` function with signature `totalBorrow()` and selector `[130, 133, 239, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalBorrow", abi = "totalBorrow()")]
    pub struct TotalBorrowCall;
    #[doc = "Container type for all input parameters for the `totalCollateralShare` function with signature `totalCollateralShare()` and selector `[71, 62, 60, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalCollateralShare", abi = "totalCollateralShare()")]
    pub struct TotalCollateralShareCall;
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address,bool,bool)` and selector `[7, 141, 251, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address,bool,bool)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
        pub direct: bool,
        pub renounce: bool,
    }
    #[doc = "Container type for all input parameters for the `updateExchangeRate` function with signature `updateExchangeRate()` and selector `[2, 206, 114, 143]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateExchangeRate", abi = "updateExchangeRate()")]
    pub struct UpdateExchangeRateCall;
    #[doc = "Container type for all input parameters for the `userBorrowPart` function with signature `userBorrowPart(address)` and selector `[72, 228, 22, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "userBorrowPart", abi = "userBorrowPart(address)")]
    pub struct UserBorrowPartCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `userCollateralShare` function with signature `userCollateralShare(address)` and selector `[28, 158, 55, 155]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "userCollateralShare", abi = "userCollateralShare(address)")]
    pub struct UserCollateralShareCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `withdrawFees` function with signature `withdrawFees()` and selector `[71, 99, 67, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdrawFees", abi = "withdrawFees()")]
    pub struct WithdrawFeesCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CauldronV4Calls {
        BorrowOpeningFee(BorrowOpeningFeeCall),
        CollaterizationRate(CollaterizationRateCall),
        LiquidationMultiplier(LiquidationMultiplierCall),
        Accrue(AccrueCall),
        AccrueInfo(AccrueInfoCall),
        AddCollateral(AddCollateralCall),
        BentoBox(BentoBoxCall),
        BlacklistedCallees(BlacklistedCalleesCall),
        Borrow(BorrowCall),
        BorrowLimit(BorrowLimitCall),
        ChangeBorrowLimit(ChangeBorrowLimitCall),
        ChangeInterestRate(ChangeInterestRateCall),
        ClaimOwnership(ClaimOwnershipCall),
        Collateral(CollateralCall),
        Cook(CookCall),
        ExchangeRate(ExchangeRateCall),
        FeeTo(FeeToCall),
        Init(InitCall),
        Liquidate(LiquidateCall),
        MagicInternetMoney(MagicInternetMoneyCall),
        MasterContract(MasterContractCall),
        Oracle(OracleCall),
        OracleData(OracleDataCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        ReduceSupply(ReduceSupplyCall),
        RemoveCollateral(RemoveCollateralCall),
        Repay(RepayCall),
        RepayForAll(RepayForAllCall),
        SetBlacklistedCallee(SetBlacklistedCalleeCall),
        SetFeeTo(SetFeeToCall),
        TotalBorrow(TotalBorrowCall),
        TotalCollateralShare(TotalCollateralShareCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateExchangeRate(UpdateExchangeRateCall),
        UserBorrowPart(UserBorrowPartCall),
        UserCollateralShare(UserCollateralShareCall),
        WithdrawFees(WithdrawFeesCall),
    }
    impl ethers::core::abi::AbiDecode for CauldronV4Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BorrowOpeningFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::BorrowOpeningFee(decoded));
            }
            if let Ok(decoded) =
                <CollaterizationRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::CollaterizationRate(decoded));
            }
            if let Ok(decoded) =
                <LiquidationMultiplierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::LiquidationMultiplier(decoded));
            }
            if let Ok(decoded) = <AccrueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::Accrue(decoded));
            }
            if let Ok(decoded) =
                <AccrueInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::AccrueInfo(decoded));
            }
            if let Ok(decoded) =
                <AddCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::AddCollateral(decoded));
            }
            if let Ok(decoded) =
                <BentoBoxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::BentoBox(decoded));
            }
            if let Ok(decoded) =
                <BlacklistedCalleesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::BlacklistedCallees(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <BorrowLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::BorrowLimit(decoded));
            }
            if let Ok(decoded) =
                <ChangeBorrowLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::ChangeBorrowLimit(decoded));
            }
            if let Ok(decoded) =
                <ChangeInterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::ChangeInterestRate(decoded));
            }
            if let Ok(decoded) =
                <ClaimOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::ClaimOwnership(decoded));
            }
            if let Ok(decoded) =
                <CollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::Collateral(decoded));
            }
            if let Ok(decoded) = <CookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CauldronV4Calls::Cook(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::ExchangeRate(decoded));
            }
            if let Ok(decoded) = <FeeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::FeeTo(decoded));
            }
            if let Ok(decoded) = <InitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CauldronV4Calls::Init(decoded));
            }
            if let Ok(decoded) =
                <LiquidateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::Liquidate(decoded));
            }
            if let Ok(decoded) =
                <MagicInternetMoneyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::MagicInternetMoney(decoded));
            }
            if let Ok(decoded) =
                <MasterContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::MasterContract(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <OracleDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::OracleData(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::PendingOwner(decoded));
            }
            if let Ok(decoded) =
                <ReduceSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::ReduceSupply(decoded));
            }
            if let Ok(decoded) =
                <RemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::RemoveCollateral(decoded));
            }
            if let Ok(decoded) = <RepayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::Repay(decoded));
            }
            if let Ok(decoded) =
                <RepayForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::RepayForAll(decoded));
            }
            if let Ok(decoded) =
                <SetBlacklistedCalleeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::SetBlacklistedCallee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::SetFeeTo(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::TotalBorrow(decoded));
            }
            if let Ok(decoded) =
                <TotalCollateralShareCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::TotalCollateralShare(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateExchangeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::UpdateExchangeRate(decoded));
            }
            if let Ok(decoded) =
                <UserBorrowPartCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::UserBorrowPart(decoded));
            }
            if let Ok(decoded) =
                <UserCollateralShareCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::UserCollateralShare(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV4Calls::WithdrawFees(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CauldronV4Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CauldronV4Calls::BorrowOpeningFee(element) => element.encode(),
                CauldronV4Calls::CollaterizationRate(element) => element.encode(),
                CauldronV4Calls::LiquidationMultiplier(element) => element.encode(),
                CauldronV4Calls::Accrue(element) => element.encode(),
                CauldronV4Calls::AccrueInfo(element) => element.encode(),
                CauldronV4Calls::AddCollateral(element) => element.encode(),
                CauldronV4Calls::BentoBox(element) => element.encode(),
                CauldronV4Calls::BlacklistedCallees(element) => element.encode(),
                CauldronV4Calls::Borrow(element) => element.encode(),
                CauldronV4Calls::BorrowLimit(element) => element.encode(),
                CauldronV4Calls::ChangeBorrowLimit(element) => element.encode(),
                CauldronV4Calls::ChangeInterestRate(element) => element.encode(),
                CauldronV4Calls::ClaimOwnership(element) => element.encode(),
                CauldronV4Calls::Collateral(element) => element.encode(),
                CauldronV4Calls::Cook(element) => element.encode(),
                CauldronV4Calls::ExchangeRate(element) => element.encode(),
                CauldronV4Calls::FeeTo(element) => element.encode(),
                CauldronV4Calls::Init(element) => element.encode(),
                CauldronV4Calls::Liquidate(element) => element.encode(),
                CauldronV4Calls::MagicInternetMoney(element) => element.encode(),
                CauldronV4Calls::MasterContract(element) => element.encode(),
                CauldronV4Calls::Oracle(element) => element.encode(),
                CauldronV4Calls::OracleData(element) => element.encode(),
                CauldronV4Calls::Owner(element) => element.encode(),
                CauldronV4Calls::PendingOwner(element) => element.encode(),
                CauldronV4Calls::ReduceSupply(element) => element.encode(),
                CauldronV4Calls::RemoveCollateral(element) => element.encode(),
                CauldronV4Calls::Repay(element) => element.encode(),
                CauldronV4Calls::RepayForAll(element) => element.encode(),
                CauldronV4Calls::SetBlacklistedCallee(element) => element.encode(),
                CauldronV4Calls::SetFeeTo(element) => element.encode(),
                CauldronV4Calls::TotalBorrow(element) => element.encode(),
                CauldronV4Calls::TotalCollateralShare(element) => element.encode(),
                CauldronV4Calls::TransferOwnership(element) => element.encode(),
                CauldronV4Calls::UpdateExchangeRate(element) => element.encode(),
                CauldronV4Calls::UserBorrowPart(element) => element.encode(),
                CauldronV4Calls::UserCollateralShare(element) => element.encode(),
                CauldronV4Calls::WithdrawFees(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CauldronV4Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CauldronV4Calls::BorrowOpeningFee(element) => element.fmt(f),
                CauldronV4Calls::CollaterizationRate(element) => element.fmt(f),
                CauldronV4Calls::LiquidationMultiplier(element) => element.fmt(f),
                CauldronV4Calls::Accrue(element) => element.fmt(f),
                CauldronV4Calls::AccrueInfo(element) => element.fmt(f),
                CauldronV4Calls::AddCollateral(element) => element.fmt(f),
                CauldronV4Calls::BentoBox(element) => element.fmt(f),
                CauldronV4Calls::BlacklistedCallees(element) => element.fmt(f),
                CauldronV4Calls::Borrow(element) => element.fmt(f),
                CauldronV4Calls::BorrowLimit(element) => element.fmt(f),
                CauldronV4Calls::ChangeBorrowLimit(element) => element.fmt(f),
                CauldronV4Calls::ChangeInterestRate(element) => element.fmt(f),
                CauldronV4Calls::ClaimOwnership(element) => element.fmt(f),
                CauldronV4Calls::Collateral(element) => element.fmt(f),
                CauldronV4Calls::Cook(element) => element.fmt(f),
                CauldronV4Calls::ExchangeRate(element) => element.fmt(f),
                CauldronV4Calls::FeeTo(element) => element.fmt(f),
                CauldronV4Calls::Init(element) => element.fmt(f),
                CauldronV4Calls::Liquidate(element) => element.fmt(f),
                CauldronV4Calls::MagicInternetMoney(element) => element.fmt(f),
                CauldronV4Calls::MasterContract(element) => element.fmt(f),
                CauldronV4Calls::Oracle(element) => element.fmt(f),
                CauldronV4Calls::OracleData(element) => element.fmt(f),
                CauldronV4Calls::Owner(element) => element.fmt(f),
                CauldronV4Calls::PendingOwner(element) => element.fmt(f),
                CauldronV4Calls::ReduceSupply(element) => element.fmt(f),
                CauldronV4Calls::RemoveCollateral(element) => element.fmt(f),
                CauldronV4Calls::Repay(element) => element.fmt(f),
                CauldronV4Calls::RepayForAll(element) => element.fmt(f),
                CauldronV4Calls::SetBlacklistedCallee(element) => element.fmt(f),
                CauldronV4Calls::SetFeeTo(element) => element.fmt(f),
                CauldronV4Calls::TotalBorrow(element) => element.fmt(f),
                CauldronV4Calls::TotalCollateralShare(element) => element.fmt(f),
                CauldronV4Calls::TransferOwnership(element) => element.fmt(f),
                CauldronV4Calls::UpdateExchangeRate(element) => element.fmt(f),
                CauldronV4Calls::UserBorrowPart(element) => element.fmt(f),
                CauldronV4Calls::UserCollateralShare(element) => element.fmt(f),
                CauldronV4Calls::WithdrawFees(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BorrowOpeningFeeCall> for CauldronV4Calls {
        fn from(var: BorrowOpeningFeeCall) -> Self {
            CauldronV4Calls::BorrowOpeningFee(var)
        }
    }
    impl ::std::convert::From<CollaterizationRateCall> for CauldronV4Calls {
        fn from(var: CollaterizationRateCall) -> Self {
            CauldronV4Calls::CollaterizationRate(var)
        }
    }
    impl ::std::convert::From<LiquidationMultiplierCall> for CauldronV4Calls {
        fn from(var: LiquidationMultiplierCall) -> Self {
            CauldronV4Calls::LiquidationMultiplier(var)
        }
    }
    impl ::std::convert::From<AccrueCall> for CauldronV4Calls {
        fn from(var: AccrueCall) -> Self {
            CauldronV4Calls::Accrue(var)
        }
    }
    impl ::std::convert::From<AccrueInfoCall> for CauldronV4Calls {
        fn from(var: AccrueInfoCall) -> Self {
            CauldronV4Calls::AccrueInfo(var)
        }
    }
    impl ::std::convert::From<AddCollateralCall> for CauldronV4Calls {
        fn from(var: AddCollateralCall) -> Self {
            CauldronV4Calls::AddCollateral(var)
        }
    }
    impl ::std::convert::From<BentoBoxCall> for CauldronV4Calls {
        fn from(var: BentoBoxCall) -> Self {
            CauldronV4Calls::BentoBox(var)
        }
    }
    impl ::std::convert::From<BlacklistedCalleesCall> for CauldronV4Calls {
        fn from(var: BlacklistedCalleesCall) -> Self {
            CauldronV4Calls::BlacklistedCallees(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for CauldronV4Calls {
        fn from(var: BorrowCall) -> Self {
            CauldronV4Calls::Borrow(var)
        }
    }
    impl ::std::convert::From<BorrowLimitCall> for CauldronV4Calls {
        fn from(var: BorrowLimitCall) -> Self {
            CauldronV4Calls::BorrowLimit(var)
        }
    }
    impl ::std::convert::From<ChangeBorrowLimitCall> for CauldronV4Calls {
        fn from(var: ChangeBorrowLimitCall) -> Self {
            CauldronV4Calls::ChangeBorrowLimit(var)
        }
    }
    impl ::std::convert::From<ChangeInterestRateCall> for CauldronV4Calls {
        fn from(var: ChangeInterestRateCall) -> Self {
            CauldronV4Calls::ChangeInterestRate(var)
        }
    }
    impl ::std::convert::From<ClaimOwnershipCall> for CauldronV4Calls {
        fn from(var: ClaimOwnershipCall) -> Self {
            CauldronV4Calls::ClaimOwnership(var)
        }
    }
    impl ::std::convert::From<CollateralCall> for CauldronV4Calls {
        fn from(var: CollateralCall) -> Self {
            CauldronV4Calls::Collateral(var)
        }
    }
    impl ::std::convert::From<CookCall> for CauldronV4Calls {
        fn from(var: CookCall) -> Self {
            CauldronV4Calls::Cook(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCall> for CauldronV4Calls {
        fn from(var: ExchangeRateCall) -> Self {
            CauldronV4Calls::ExchangeRate(var)
        }
    }
    impl ::std::convert::From<FeeToCall> for CauldronV4Calls {
        fn from(var: FeeToCall) -> Self {
            CauldronV4Calls::FeeTo(var)
        }
    }
    impl ::std::convert::From<InitCall> for CauldronV4Calls {
        fn from(var: InitCall) -> Self {
            CauldronV4Calls::Init(var)
        }
    }
    impl ::std::convert::From<LiquidateCall> for CauldronV4Calls {
        fn from(var: LiquidateCall) -> Self {
            CauldronV4Calls::Liquidate(var)
        }
    }
    impl ::std::convert::From<MagicInternetMoneyCall> for CauldronV4Calls {
        fn from(var: MagicInternetMoneyCall) -> Self {
            CauldronV4Calls::MagicInternetMoney(var)
        }
    }
    impl ::std::convert::From<MasterContractCall> for CauldronV4Calls {
        fn from(var: MasterContractCall) -> Self {
            CauldronV4Calls::MasterContract(var)
        }
    }
    impl ::std::convert::From<OracleCall> for CauldronV4Calls {
        fn from(var: OracleCall) -> Self {
            CauldronV4Calls::Oracle(var)
        }
    }
    impl ::std::convert::From<OracleDataCall> for CauldronV4Calls {
        fn from(var: OracleDataCall) -> Self {
            CauldronV4Calls::OracleData(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for CauldronV4Calls {
        fn from(var: OwnerCall) -> Self {
            CauldronV4Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingOwnerCall> for CauldronV4Calls {
        fn from(var: PendingOwnerCall) -> Self {
            CauldronV4Calls::PendingOwner(var)
        }
    }
    impl ::std::convert::From<ReduceSupplyCall> for CauldronV4Calls {
        fn from(var: ReduceSupplyCall) -> Self {
            CauldronV4Calls::ReduceSupply(var)
        }
    }
    impl ::std::convert::From<RemoveCollateralCall> for CauldronV4Calls {
        fn from(var: RemoveCollateralCall) -> Self {
            CauldronV4Calls::RemoveCollateral(var)
        }
    }
    impl ::std::convert::From<RepayCall> for CauldronV4Calls {
        fn from(var: RepayCall) -> Self {
            CauldronV4Calls::Repay(var)
        }
    }
    impl ::std::convert::From<RepayForAllCall> for CauldronV4Calls {
        fn from(var: RepayForAllCall) -> Self {
            CauldronV4Calls::RepayForAll(var)
        }
    }
    impl ::std::convert::From<SetBlacklistedCalleeCall> for CauldronV4Calls {
        fn from(var: SetBlacklistedCalleeCall) -> Self {
            CauldronV4Calls::SetBlacklistedCallee(var)
        }
    }
    impl ::std::convert::From<SetFeeToCall> for CauldronV4Calls {
        fn from(var: SetFeeToCall) -> Self {
            CauldronV4Calls::SetFeeTo(var)
        }
    }
    impl ::std::convert::From<TotalBorrowCall> for CauldronV4Calls {
        fn from(var: TotalBorrowCall) -> Self {
            CauldronV4Calls::TotalBorrow(var)
        }
    }
    impl ::std::convert::From<TotalCollateralShareCall> for CauldronV4Calls {
        fn from(var: TotalCollateralShareCall) -> Self {
            CauldronV4Calls::TotalCollateralShare(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for CauldronV4Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            CauldronV4Calls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpdateExchangeRateCall> for CauldronV4Calls {
        fn from(var: UpdateExchangeRateCall) -> Self {
            CauldronV4Calls::UpdateExchangeRate(var)
        }
    }
    impl ::std::convert::From<UserBorrowPartCall> for CauldronV4Calls {
        fn from(var: UserBorrowPartCall) -> Self {
            CauldronV4Calls::UserBorrowPart(var)
        }
    }
    impl ::std::convert::From<UserCollateralShareCall> for CauldronV4Calls {
        fn from(var: UserCollateralShareCall) -> Self {
            CauldronV4Calls::UserCollateralShare(var)
        }
    }
    impl ::std::convert::From<WithdrawFeesCall> for CauldronV4Calls {
        fn from(var: WithdrawFeesCall) -> Self {
            CauldronV4Calls::WithdrawFees(var)
        }
    }
    #[doc = "Container type for all return fields from the `BORROW_OPENING_FEE` function with signature `BORROW_OPENING_FEE()` and selector `[171, 160, 36, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BorrowOpeningFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `COLLATERIZATION_RATE` function with signature `COLLATERIZATION_RATE()` and selector `[199, 238, 42, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CollaterizationRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `LIQUIDATION_MULTIPLIER` function with signature `LIQUIDATION_MULTIPLIER()` and selector `[110, 192, 151, 251]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LiquidationMultiplierReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `accrueInfo` function with signature `accrueInfo()` and selector `[178, 124, 14, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AccrueInfoReturn {
        pub last_accrued: u64,
        pub fees_earned: u128,
        pub interest_per_second: u64,
    }
    #[doc = "Container type for all return fields from the `bentoBox` function with signature `bentoBox()` and selector `[107, 42, 206, 135]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BentoBoxReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `blacklistedCallees` function with signature `blacklistedCallees(address)` and selector `[203, 13, 197, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BlacklistedCalleesReturn(pub bool);
    #[doc = "Container type for all return fields from the `borrow` function with signature `borrow(address,uint256)` and selector `[75, 138, 53, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BorrowReturn {
        pub part: ethers::core::types::U256,
        pub share: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `borrowLimit` function with signature `borrowLimit()` and selector `[229, 81, 209, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BorrowLimitReturn {
        pub total: u128,
        pub borrow_part_per_address: u128,
    }
    #[doc = "Container type for all return fields from the `collateral` function with signature `collateral()` and selector `[216, 223, 235, 69]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CollateralReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `cook` function with signature `cook(uint8[],uint256[],bytes[])` and selector `[101, 111, 61, 100]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CookReturn {
        pub value_1: ethers::core::types::U256,
        pub value_2: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `exchangeRate` function with signature `exchangeRate()` and selector `[59, 160, 185, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExchangeRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `feeTo` function with signature `feeTo()` and selector `[1, 126, 126, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeToReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `magicInternetMoney` function with signature `magicInternetMoney()` and selector `[155, 53, 42, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MagicInternetMoneyReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `masterContract` function with signature `masterContract()` and selector `[205, 68, 110, 34]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MasterContractReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `oracleData` function with signature `oracleData()` and selector `[116, 100, 95, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OracleDataReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `[227, 12, 57, 120]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingOwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `repay` function with signature `repay(address,bool,uint256)` and selector `[21, 41, 76, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RepayReturn {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `repayForAll` function with signature `repayForAll(uint128,bool)` and selector `[126, 244, 4, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RepayForAllReturn(pub u128);
    #[doc = "Container type for all return fields from the `totalBorrow` function with signature `totalBorrow()` and selector `[130, 133, 239, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalBorrowReturn {
        pub elastic: u128,
        pub base: u128,
    }
    #[doc = "Container type for all return fields from the `totalCollateralShare` function with signature `totalCollateralShare()` and selector `[71, 62, 60, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalCollateralShareReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `updateExchangeRate` function with signature `updateExchangeRate()` and selector `[2, 206, 114, 143]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UpdateExchangeRateReturn {
        pub updated: bool,
        pub rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `userBorrowPart` function with signature `userBorrowPart(address)` and selector `[72, 228, 22, 62]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UserBorrowPartReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `userCollateralShare` function with signature `userCollateralShare(address)` and selector `[28, 158, 55, 155]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UserCollateralShareReturn(pub ethers::core::types::U256);
}
