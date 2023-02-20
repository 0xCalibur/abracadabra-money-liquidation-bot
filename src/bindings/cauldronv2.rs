pub use cauldron_v2::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod cauldron_v2 {
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
    #[doc = "CauldronV2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IBentoBoxV1\",\n        \"name\": \"bentoBox_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"magicInternetMoney_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"accruedAmount\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"LogAccrue\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogAddCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"part\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogBorrow\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"rate\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogExchangeRate\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newFeeTo\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LogFeeTo\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogRemoveCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"part\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogRepay\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"feeTo\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"feesEarnedFraction\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogWithdrawFees\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"BORROW_OPENING_FEE\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"COLLATERIZATION_RATE\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"LIQUIDATION_MULTIPLIER\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"accrue\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"accrueInfo\",\n    \"outputs\": [\n      { \"internalType\": \"uint64\", \"name\": \"lastAccrued\", \"type\": \"uint64\" },\n      { \"internalType\": \"uint128\", \"name\": \"feesEarned\", \"type\": \"uint128\" },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"INTEREST_PER_SECOND\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      { \"internalType\": \"bool\", \"name\": \"skim\", \"type\": \"bool\" },\n      { \"internalType\": \"uint256\", \"name\": \"share\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"addCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"bentoBox\",\n    \"outputs\": [\n      { \"internalType\": \"contract IBentoBoxV1\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"borrow\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"part\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"share\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"claimOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"collateral\",\n    \"outputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint8[]\", \"name\": \"actions\", \"type\": \"uint8[]\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"values\", \"type\": \"uint256[]\" },\n      { \"internalType\": \"bytes[]\", \"name\": \"datas\", \"type\": \"bytes[]\" }\n    ],\n    \"name\": \"cook\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"value1\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"value2\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"exchangeRate\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeTo\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }],\n    \"name\": \"init\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address[]\", \"name\": \"users\", \"type\": \"address[]\" },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"maxBorrowParts\",\n        \"type\": \"uint256[]\"\n      },\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      {\n        \"internalType\": \"contract ISwapper\",\n        \"name\": \"swapper\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"liquidate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"magicInternetMoney\",\n    \"outputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"masterContract\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract CauldronV2Flat\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"oracle\",\n    \"outputs\": [\n      { \"internalType\": \"contract IOracle\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"oracleData\",\n    \"outputs\": [{ \"internalType\": \"bytes\", \"name\": \"\", \"type\": \"bytes\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"pendingOwner\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"reduceSupply\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"share\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"removeCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      { \"internalType\": \"bool\", \"name\": \"skim\", \"type\": \"bool\" },\n      { \"internalType\": \"uint256\", \"name\": \"part\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"repay\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"newFeeTo\", \"type\": \"address\" }\n    ],\n    \"name\": \"setFeeTo\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalBorrow\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"elastic\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"base\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalCollateralShare\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"newOwner\", \"type\": \"address\" },\n      { \"internalType\": \"bool\", \"name\": \"direct\", \"type\": \"bool\" },\n      { \"internalType\": \"bool\", \"name\": \"renounce\", \"type\": \"bool\" }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"updateExchangeRate\",\n    \"outputs\": [\n      { \"internalType\": \"bool\", \"name\": \"updated\", \"type\": \"bool\" },\n      { \"internalType\": \"uint256\", \"name\": \"rate\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"name\": \"userBorrowPart\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"name\": \"userCollateralShare\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"withdrawFees\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static CAULDRONV2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct CauldronV2<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CauldronV2<M> {
        fn clone(&self) -> Self {
            CauldronV2(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CauldronV2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for CauldronV2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CauldronV2)).field(&self.address()).finish()
        }
    }
    impl<M: ethers::providers::Middleware> CauldronV2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CAULDRONV2_ABI.clone(), client).into()
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
        #[doc = "Calls the contract's `liquidate` (0x912860c5) function"]
        pub fn liquidate(
            &self,
            users: ::std::vec::Vec<ethers::core::types::Address>,
            max_borrow_parts: ::std::vec::Vec<ethers::core::types::U256>,
            to: ethers::core::types::Address,
            swapper: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 40, 96, 197], (users, max_borrow_parts, to, swapper))
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, CauldronV2Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CauldronV2<M> {
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
    pub enum CauldronV2Events {
        LogAccrueFilter(LogAccrueFilter),
        LogAddCollateralFilter(LogAddCollateralFilter),
        LogBorrowFilter(LogBorrowFilter),
        LogExchangeRateFilter(LogExchangeRateFilter),
        LogFeeToFilter(LogFeeToFilter),
        LogRemoveCollateralFilter(LogRemoveCollateralFilter),
        LogRepayFilter(LogRepayFilter),
        LogWithdrawFeesFilter(LogWithdrawFeesFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for CauldronV2Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogAccrueFilter::decode_log(log) {
                return Ok(CauldronV2Events::LogAccrueFilter(decoded));
            }
            if let Ok(decoded) = LogAddCollateralFilter::decode_log(log) {
                return Ok(CauldronV2Events::LogAddCollateralFilter(decoded));
            }
            if let Ok(decoded) = LogBorrowFilter::decode_log(log) {
                return Ok(CauldronV2Events::LogBorrowFilter(decoded));
            }
            if let Ok(decoded) = LogExchangeRateFilter::decode_log(log) {
                return Ok(CauldronV2Events::LogExchangeRateFilter(decoded));
            }
            if let Ok(decoded) = LogFeeToFilter::decode_log(log) {
                return Ok(CauldronV2Events::LogFeeToFilter(decoded));
            }
            if let Ok(decoded) = LogRemoveCollateralFilter::decode_log(log) {
                return Ok(CauldronV2Events::LogRemoveCollateralFilter(decoded));
            }
            if let Ok(decoded) = LogRepayFilter::decode_log(log) {
                return Ok(CauldronV2Events::LogRepayFilter(decoded));
            }
            if let Ok(decoded) = LogWithdrawFeesFilter::decode_log(log) {
                return Ok(CauldronV2Events::LogWithdrawFeesFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CauldronV2Events::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CauldronV2Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CauldronV2Events::LogAccrueFilter(element) => element.fmt(f),
                CauldronV2Events::LogAddCollateralFilter(element) => element.fmt(f),
                CauldronV2Events::LogBorrowFilter(element) => element.fmt(f),
                CauldronV2Events::LogExchangeRateFilter(element) => element.fmt(f),
                CauldronV2Events::LogFeeToFilter(element) => element.fmt(f),
                CauldronV2Events::LogRemoveCollateralFilter(element) => element.fmt(f),
                CauldronV2Events::LogRepayFilter(element) => element.fmt(f),
                CauldronV2Events::LogWithdrawFeesFilter(element) => element.fmt(f),
                CauldronV2Events::OwnershipTransferredFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `liquidate` function with signature `liquidate(address[],uint256[],address,address)` and selector `[145, 40, 96, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "liquidate", abi = "liquidate(address[],uint256[],address,address)")]
    pub struct LiquidateCall {
        pub users: ::std::vec::Vec<ethers::core::types::Address>,
        pub max_borrow_parts: ::std::vec::Vec<ethers::core::types::U256>,
        pub to: ethers::core::types::Address,
        pub swapper: ethers::core::types::Address,
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
    pub enum CauldronV2Calls {
        BorrowOpeningFee(BorrowOpeningFeeCall),
        CollaterizationRate(CollaterizationRateCall),
        LiquidationMultiplier(LiquidationMultiplierCall),
        Accrue(AccrueCall),
        AccrueInfo(AccrueInfoCall),
        AddCollateral(AddCollateralCall),
        BentoBox(BentoBoxCall),
        Borrow(BorrowCall),
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
        SetFeeTo(SetFeeToCall),
        TotalBorrow(TotalBorrowCall),
        TotalCollateralShare(TotalCollateralShareCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateExchangeRate(UpdateExchangeRateCall),
        UserBorrowPart(UserBorrowPartCall),
        UserCollateralShare(UserCollateralShareCall),
        WithdrawFees(WithdrawFeesCall),
    }
    impl ethers::core::abi::AbiDecode for CauldronV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BorrowOpeningFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::BorrowOpeningFee(decoded));
            }
            if let Ok(decoded) =
                <CollaterizationRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::CollaterizationRate(decoded));
            }
            if let Ok(decoded) =
                <LiquidationMultiplierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::LiquidationMultiplier(decoded));
            }
            if let Ok(decoded) = <AccrueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::Accrue(decoded));
            }
            if let Ok(decoded) =
                <AccrueInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::AccrueInfo(decoded));
            }
            if let Ok(decoded) =
                <AddCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::AddCollateral(decoded));
            }
            if let Ok(decoded) =
                <BentoBoxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::BentoBox(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <ClaimOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::ClaimOwnership(decoded));
            }
            if let Ok(decoded) =
                <CollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::Collateral(decoded));
            }
            if let Ok(decoded) = <CookCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CauldronV2Calls::Cook(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::ExchangeRate(decoded));
            }
            if let Ok(decoded) = <FeeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::FeeTo(decoded));
            }
            if let Ok(decoded) = <InitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CauldronV2Calls::Init(decoded));
            }
            if let Ok(decoded) =
                <LiquidateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::Liquidate(decoded));
            }
            if let Ok(decoded) =
                <MagicInternetMoneyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::MagicInternetMoney(decoded));
            }
            if let Ok(decoded) =
                <MasterContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::MasterContract(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <OracleDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::OracleData(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::PendingOwner(decoded));
            }
            if let Ok(decoded) =
                <ReduceSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::ReduceSupply(decoded));
            }
            if let Ok(decoded) =
                <RemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::RemoveCollateral(decoded));
            }
            if let Ok(decoded) = <RepayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::Repay(decoded));
            }
            if let Ok(decoded) =
                <SetFeeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::SetFeeTo(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::TotalBorrow(decoded));
            }
            if let Ok(decoded) =
                <TotalCollateralShareCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::TotalCollateralShare(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateExchangeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::UpdateExchangeRate(decoded));
            }
            if let Ok(decoded) =
                <UserBorrowPartCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::UserBorrowPart(decoded));
            }
            if let Ok(decoded) =
                <UserCollateralShareCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::UserCollateralShare(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CauldronV2Calls::WithdrawFees(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CauldronV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CauldronV2Calls::BorrowOpeningFee(element) => element.encode(),
                CauldronV2Calls::CollaterizationRate(element) => element.encode(),
                CauldronV2Calls::LiquidationMultiplier(element) => element.encode(),
                CauldronV2Calls::Accrue(element) => element.encode(),
                CauldronV2Calls::AccrueInfo(element) => element.encode(),
                CauldronV2Calls::AddCollateral(element) => element.encode(),
                CauldronV2Calls::BentoBox(element) => element.encode(),
                CauldronV2Calls::Borrow(element) => element.encode(),
                CauldronV2Calls::ClaimOwnership(element) => element.encode(),
                CauldronV2Calls::Collateral(element) => element.encode(),
                CauldronV2Calls::Cook(element) => element.encode(),
                CauldronV2Calls::ExchangeRate(element) => element.encode(),
                CauldronV2Calls::FeeTo(element) => element.encode(),
                CauldronV2Calls::Init(element) => element.encode(),
                CauldronV2Calls::Liquidate(element) => element.encode(),
                CauldronV2Calls::MagicInternetMoney(element) => element.encode(),
                CauldronV2Calls::MasterContract(element) => element.encode(),
                CauldronV2Calls::Oracle(element) => element.encode(),
                CauldronV2Calls::OracleData(element) => element.encode(),
                CauldronV2Calls::Owner(element) => element.encode(),
                CauldronV2Calls::PendingOwner(element) => element.encode(),
                CauldronV2Calls::ReduceSupply(element) => element.encode(),
                CauldronV2Calls::RemoveCollateral(element) => element.encode(),
                CauldronV2Calls::Repay(element) => element.encode(),
                CauldronV2Calls::SetFeeTo(element) => element.encode(),
                CauldronV2Calls::TotalBorrow(element) => element.encode(),
                CauldronV2Calls::TotalCollateralShare(element) => element.encode(),
                CauldronV2Calls::TransferOwnership(element) => element.encode(),
                CauldronV2Calls::UpdateExchangeRate(element) => element.encode(),
                CauldronV2Calls::UserBorrowPart(element) => element.encode(),
                CauldronV2Calls::UserCollateralShare(element) => element.encode(),
                CauldronV2Calls::WithdrawFees(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CauldronV2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CauldronV2Calls::BorrowOpeningFee(element) => element.fmt(f),
                CauldronV2Calls::CollaterizationRate(element) => element.fmt(f),
                CauldronV2Calls::LiquidationMultiplier(element) => element.fmt(f),
                CauldronV2Calls::Accrue(element) => element.fmt(f),
                CauldronV2Calls::AccrueInfo(element) => element.fmt(f),
                CauldronV2Calls::AddCollateral(element) => element.fmt(f),
                CauldronV2Calls::BentoBox(element) => element.fmt(f),
                CauldronV2Calls::Borrow(element) => element.fmt(f),
                CauldronV2Calls::ClaimOwnership(element) => element.fmt(f),
                CauldronV2Calls::Collateral(element) => element.fmt(f),
                CauldronV2Calls::Cook(element) => element.fmt(f),
                CauldronV2Calls::ExchangeRate(element) => element.fmt(f),
                CauldronV2Calls::FeeTo(element) => element.fmt(f),
                CauldronV2Calls::Init(element) => element.fmt(f),
                CauldronV2Calls::Liquidate(element) => element.fmt(f),
                CauldronV2Calls::MagicInternetMoney(element) => element.fmt(f),
                CauldronV2Calls::MasterContract(element) => element.fmt(f),
                CauldronV2Calls::Oracle(element) => element.fmt(f),
                CauldronV2Calls::OracleData(element) => element.fmt(f),
                CauldronV2Calls::Owner(element) => element.fmt(f),
                CauldronV2Calls::PendingOwner(element) => element.fmt(f),
                CauldronV2Calls::ReduceSupply(element) => element.fmt(f),
                CauldronV2Calls::RemoveCollateral(element) => element.fmt(f),
                CauldronV2Calls::Repay(element) => element.fmt(f),
                CauldronV2Calls::SetFeeTo(element) => element.fmt(f),
                CauldronV2Calls::TotalBorrow(element) => element.fmt(f),
                CauldronV2Calls::TotalCollateralShare(element) => element.fmt(f),
                CauldronV2Calls::TransferOwnership(element) => element.fmt(f),
                CauldronV2Calls::UpdateExchangeRate(element) => element.fmt(f),
                CauldronV2Calls::UserBorrowPart(element) => element.fmt(f),
                CauldronV2Calls::UserCollateralShare(element) => element.fmt(f),
                CauldronV2Calls::WithdrawFees(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BorrowOpeningFeeCall> for CauldronV2Calls {
        fn from(var: BorrowOpeningFeeCall) -> Self {
            CauldronV2Calls::BorrowOpeningFee(var)
        }
    }
    impl ::std::convert::From<CollaterizationRateCall> for CauldronV2Calls {
        fn from(var: CollaterizationRateCall) -> Self {
            CauldronV2Calls::CollaterizationRate(var)
        }
    }
    impl ::std::convert::From<LiquidationMultiplierCall> for CauldronV2Calls {
        fn from(var: LiquidationMultiplierCall) -> Self {
            CauldronV2Calls::LiquidationMultiplier(var)
        }
    }
    impl ::std::convert::From<AccrueCall> for CauldronV2Calls {
        fn from(var: AccrueCall) -> Self {
            CauldronV2Calls::Accrue(var)
        }
    }
    impl ::std::convert::From<AccrueInfoCall> for CauldronV2Calls {
        fn from(var: AccrueInfoCall) -> Self {
            CauldronV2Calls::AccrueInfo(var)
        }
    }
    impl ::std::convert::From<AddCollateralCall> for CauldronV2Calls {
        fn from(var: AddCollateralCall) -> Self {
            CauldronV2Calls::AddCollateral(var)
        }
    }
    impl ::std::convert::From<BentoBoxCall> for CauldronV2Calls {
        fn from(var: BentoBoxCall) -> Self {
            CauldronV2Calls::BentoBox(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for CauldronV2Calls {
        fn from(var: BorrowCall) -> Self {
            CauldronV2Calls::Borrow(var)
        }
    }
    impl ::std::convert::From<ClaimOwnershipCall> for CauldronV2Calls {
        fn from(var: ClaimOwnershipCall) -> Self {
            CauldronV2Calls::ClaimOwnership(var)
        }
    }
    impl ::std::convert::From<CollateralCall> for CauldronV2Calls {
        fn from(var: CollateralCall) -> Self {
            CauldronV2Calls::Collateral(var)
        }
    }
    impl ::std::convert::From<CookCall> for CauldronV2Calls {
        fn from(var: CookCall) -> Self {
            CauldronV2Calls::Cook(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCall> for CauldronV2Calls {
        fn from(var: ExchangeRateCall) -> Self {
            CauldronV2Calls::ExchangeRate(var)
        }
    }
    impl ::std::convert::From<FeeToCall> for CauldronV2Calls {
        fn from(var: FeeToCall) -> Self {
            CauldronV2Calls::FeeTo(var)
        }
    }
    impl ::std::convert::From<InitCall> for CauldronV2Calls {
        fn from(var: InitCall) -> Self {
            CauldronV2Calls::Init(var)
        }
    }
    impl ::std::convert::From<LiquidateCall> for CauldronV2Calls {
        fn from(var: LiquidateCall) -> Self {
            CauldronV2Calls::Liquidate(var)
        }
    }
    impl ::std::convert::From<MagicInternetMoneyCall> for CauldronV2Calls {
        fn from(var: MagicInternetMoneyCall) -> Self {
            CauldronV2Calls::MagicInternetMoney(var)
        }
    }
    impl ::std::convert::From<MasterContractCall> for CauldronV2Calls {
        fn from(var: MasterContractCall) -> Self {
            CauldronV2Calls::MasterContract(var)
        }
    }
    impl ::std::convert::From<OracleCall> for CauldronV2Calls {
        fn from(var: OracleCall) -> Self {
            CauldronV2Calls::Oracle(var)
        }
    }
    impl ::std::convert::From<OracleDataCall> for CauldronV2Calls {
        fn from(var: OracleDataCall) -> Self {
            CauldronV2Calls::OracleData(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for CauldronV2Calls {
        fn from(var: OwnerCall) -> Self {
            CauldronV2Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingOwnerCall> for CauldronV2Calls {
        fn from(var: PendingOwnerCall) -> Self {
            CauldronV2Calls::PendingOwner(var)
        }
    }
    impl ::std::convert::From<ReduceSupplyCall> for CauldronV2Calls {
        fn from(var: ReduceSupplyCall) -> Self {
            CauldronV2Calls::ReduceSupply(var)
        }
    }
    impl ::std::convert::From<RemoveCollateralCall> for CauldronV2Calls {
        fn from(var: RemoveCollateralCall) -> Self {
            CauldronV2Calls::RemoveCollateral(var)
        }
    }
    impl ::std::convert::From<RepayCall> for CauldronV2Calls {
        fn from(var: RepayCall) -> Self {
            CauldronV2Calls::Repay(var)
        }
    }
    impl ::std::convert::From<SetFeeToCall> for CauldronV2Calls {
        fn from(var: SetFeeToCall) -> Self {
            CauldronV2Calls::SetFeeTo(var)
        }
    }
    impl ::std::convert::From<TotalBorrowCall> for CauldronV2Calls {
        fn from(var: TotalBorrowCall) -> Self {
            CauldronV2Calls::TotalBorrow(var)
        }
    }
    impl ::std::convert::From<TotalCollateralShareCall> for CauldronV2Calls {
        fn from(var: TotalCollateralShareCall) -> Self {
            CauldronV2Calls::TotalCollateralShare(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for CauldronV2Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            CauldronV2Calls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpdateExchangeRateCall> for CauldronV2Calls {
        fn from(var: UpdateExchangeRateCall) -> Self {
            CauldronV2Calls::UpdateExchangeRate(var)
        }
    }
    impl ::std::convert::From<UserBorrowPartCall> for CauldronV2Calls {
        fn from(var: UserBorrowPartCall) -> Self {
            CauldronV2Calls::UserBorrowPart(var)
        }
    }
    impl ::std::convert::From<UserCollateralShareCall> for CauldronV2Calls {
        fn from(var: UserCollateralShareCall) -> Self {
            CauldronV2Calls::UserCollateralShare(var)
        }
    }
    impl ::std::convert::From<WithdrawFeesCall> for CauldronV2Calls {
        fn from(var: WithdrawFeesCall) -> Self {
            CauldronV2Calls::WithdrawFees(var)
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
