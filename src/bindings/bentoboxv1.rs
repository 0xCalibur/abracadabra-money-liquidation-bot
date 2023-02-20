pub use bento_box_v1::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod bento_box_v1 {
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
    #[doc = "BentoBoxV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"wethToken_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"masterContract\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"cloneAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LogDeploy\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogDeposit\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"feeAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"receiver\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LogFlashLoan\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"protocol\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LogRegisterProtocol\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"masterContract\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"approved\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"LogSetMasterContractApproval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogStrategyDivest\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogStrategyInvest\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogStrategyLoss\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogStrategyProfit\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IStrategy\",\n        \"name\": \"strategy\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LogStrategyQueued\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IStrategy\",\n        \"name\": \"strategy\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LogStrategySet\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"targetPercentage\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogStrategyTargetPercentage\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogTransfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"masterContract\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"approved\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"LogWhiteListMasterContract\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"share\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LogWithdraw\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"DOMAIN_SEPARATOR\",\n    \"outputs\": [{ \"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"name\": \"balanceOf\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"bytes[]\", \"name\": \"calls\", \"type\": \"bytes[]\" },\n      { \"internalType\": \"bool\", \"name\": \"revertOnFail\", \"type\": \"bool\" }\n    ],\n    \"name\": \"batch\",\n    \"outputs\": [\n      { \"internalType\": \"bool[]\", \"name\": \"successes\", \"type\": \"bool[]\" },\n      { \"internalType\": \"bytes[]\", \"name\": \"results\", \"type\": \"bytes[]\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IBatchFlashBorrower\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"address[]\", \"name\": \"receivers\", \"type\": \"address[]\" },\n      {\n        \"internalType\": \"contract IERC20[]\",\n        \"name\": \"tokens\",\n        \"type\": \"address[]\"\n      },\n      { \"internalType\": \"uint256[]\", \"name\": \"amounts\", \"type\": \"uint256[]\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"batchFlashLoan\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"claimOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"masterContract\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" },\n      { \"internalType\": \"bool\", \"name\": \"useCreate2\", \"type\": \"bool\" }\n    ],\n    \"name\": \"deploy\",\n    \"outputs\": [\n      { \"internalType\": \"address\", \"name\": \"cloneAddress\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token_\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"address\", \"name\": \"from\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"share\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"deposit\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amountOut\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"shareOut\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IFlashBorrower\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"address\", \"name\": \"receiver\", \"type\": \"address\" },\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"flashLoan\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"bool\", \"name\": \"balance\", \"type\": \"bool\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maxChangeAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"harvest\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"name\": \"masterContractApproved\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"name\": \"masterContractOf\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"name\": \"nonces\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"pendingOwner\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"name\": \"pendingStrategy\",\n    \"outputs\": [\n      { \"internalType\": \"contract IStrategy\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"from\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"deadline\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint8\", \"name\": \"v\", \"type\": \"uint8\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"s\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"permitToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"registerProtocol\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"user\", \"type\": \"address\" },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"masterContract\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"bool\", \"name\": \"approved\", \"type\": \"bool\" },\n      { \"internalType\": \"uint8\", \"name\": \"v\", \"type\": \"uint8\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"s\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"setMasterContractApproval\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      {\n        \"internalType\": \"contract IStrategy\",\n        \"name\": \"newStrategy\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setStrategy\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"targetPercentage_\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"name\": \"setStrategyTargetPercentage\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"name\": \"strategy\",\n    \"outputs\": [\n      { \"internalType\": \"contract IStrategy\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"name\": \"strategyData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"strategyStartDate\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"targetPercentage\",\n        \"type\": \"uint64\"\n      },\n      { \"internalType\": \"uint128\", \"name\": \"balance\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"share\", \"type\": \"uint256\" },\n      { \"internalType\": \"bool\", \"name\": \"roundUp\", \"type\": \"bool\" }\n    ],\n    \"name\": \"toAmount\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"bool\", \"name\": \"roundUp\", \"type\": \"bool\" }\n    ],\n    \"name\": \"toShare\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"share\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"\", \"type\": \"address\" }\n    ],\n    \"name\": \"totals\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"elastic\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"base\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"from\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"share\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"transfer\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"from\", \"type\": \"address\" },\n      { \"internalType\": \"address[]\", \"name\": \"tos\", \"type\": \"address[]\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"shares\", \"type\": \"uint256[]\" }\n    ],\n    \"name\": \"transferMultiple\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"newOwner\", \"type\": \"address\" },\n      { \"internalType\": \"bool\", \"name\": \"direct\", \"type\": \"bool\" },\n      { \"internalType\": \"bool\", \"name\": \"renounce\", \"type\": \"bool\" }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"masterContract\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"bool\", \"name\": \"approved\", \"type\": \"bool\" }\n    ],\n    \"name\": \"whitelistMasterContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"name\": \"whitelistedMasterContracts\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"token_\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"address\", \"name\": \"from\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"share\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"withdraw\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amountOut\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"shareOut\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  { \"stateMutability\": \"payable\", \"type\": \"receive\" }\n]\n" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static BENTOBOXV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct BentoBoxV1<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BentoBoxV1<M> {
        fn clone(&self) -> Self {
            BentoBoxV1(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for BentoBoxV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for BentoBoxV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BentoBoxV1)).field(&self.address()).finish()
        }
    }
    impl<M: ethers::providers::Middleware> BentoBoxV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BENTOBOXV1_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0xf7888aec) function"]
        pub fn balance_of(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([247, 136, 138, 236], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `batch` (0xd2423b51) function"]
        pub fn batch(
            &self,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
            revert_on_fail: bool,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<bool>, ::std::vec::Vec<ethers::core::types::Bytes>),
        > {
            self.0
                .method_hash([210, 66, 59, 81], (calls, revert_on_fail))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `batchFlashLoan` (0xf483b3da) function"]
        pub fn batch_flash_loan(
            &self,
            borrower: ethers::core::types::Address,
            receivers: ::std::vec::Vec<ethers::core::types::Address>,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 131, 179, 218], (borrower, receivers, tokens, amounts, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimOwnership` (0x4e71e0c8) function"]
        pub fn claim_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 113, 224, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deploy` (0x1f54245b) function"]
        pub fn deploy(
            &self,
            master_contract: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
            use_create_2: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([31, 84, 36, 91], (master_contract, data, use_create_2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x02b9446c) function"]
        pub fn deposit(
            &self,
            token: ethers::core::types::Address,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            share: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([2, 185, 68, 108], (token, from, to, amount, share))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flashLoan` (0xf1676d37) function"]
        pub fn flash_loan(
            &self,
            borrower: ethers::core::types::Address,
            receiver: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 103, 109, 55], (borrower, receiver, token, amount, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `harvest` (0x66c6bb0b) function"]
        pub fn harvest(
            &self,
            token: ethers::core::types::Address,
            balance: bool,
            max_change_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 198, 187, 11], (token, balance, max_change_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `masterContractApproved` (0x91e0eab5) function"]
        pub fn master_contract_approved(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 224, 234, 181], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `masterContractOf` (0xbafe4f14) function"]
        pub fn master_contract_of(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([186, 254, 79, 20], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
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
        #[doc = "Calls the contract's `pendingStrategy` (0x5108a558) function"]
        pub fn pending_strategy(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([81, 8, 165, 88], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permitToken` (0x7c516e94) function"]
        pub fn permit_token(
            &self,
            token: ethers::core::types::Address,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 81, 110, 148], (token, from, to, amount, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerProtocol` (0xaee4d1b2) function"]
        pub fn register_protocol(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 228, 209, 178], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMasterContractApproval` (0xc0a47c93) function"]
        pub fn set_master_contract_approval(
            &self,
            user: ethers::core::types::Address,
            master_contract: ethers::core::types::Address,
            approved: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 164, 124, 147], (user, master_contract, approved, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStrategy` (0x72cb5d97) function"]
        pub fn set_strategy(
            &self,
            token: ethers::core::types::Address,
            new_strategy: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 203, 93, 151], (token, new_strategy))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStrategyTargetPercentage` (0x3e2a9d4e) function"]
        pub fn set_strategy_target_percentage(
            &self,
            token: ethers::core::types::Address,
            target_percentage: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 42, 157, 78], (token, target_percentage))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `strategy` (0x228bfd9f) function"]
        pub fn strategy(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([34, 139, 253, 159], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `strategyData` (0xdf23b45b) function"]
        pub fn strategy_data(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (u64, u64, u128)> {
            self.0
                .method_hash([223, 35, 180, 91], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `toAmount` (0x56623118) function"]
        pub fn to_amount(
            &self,
            token: ethers::core::types::Address,
            share: ethers::core::types::U256,
            round_up: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([86, 98, 49, 24], (token, share, round_up))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `toShare` (0xda5139ca) function"]
        pub fn to_share(
            &self,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            round_up: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([218, 81, 57, 202], (token, amount, round_up))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totals` (0x4ffe34db) function"]
        pub fn totals(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([79, 254, 52, 219], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xf18d03cc) function"]
        pub fn transfer(
            &self,
            token: ethers::core::types::Address,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            share: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 141, 3, 204], (token, from, to, share))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferMultiple` (0x0fca8843) function"]
        pub fn transfer_multiple(
            &self,
            token: ethers::core::types::Address,
            from: ethers::core::types::Address,
            tos: ::std::vec::Vec<ethers::core::types::Address>,
            shares: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 202, 136, 67], (token, from, tos, shares))
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
        #[doc = "Calls the contract's `whitelistMasterContract` (0x733a9d7c) function"]
        pub fn whitelist_master_contract(
            &self,
            master_contract: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 58, 157, 124], (master_contract, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `whitelistedMasterContracts` (0x12a90c8a) function"]
        pub fn whitelisted_master_contracts(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([18, 169, 12, 138], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x97da6d30) function"]
        pub fn withdraw(
            &self,
            token: ethers::core::types::Address,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            share: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([151, 218, 109, 48], (token, from, to, amount, share))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `LogDeploy` event"]
        pub fn log_deploy_filter(&self) -> ethers::contract::builders::Event<M, LogDeployFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogDeposit` event"]
        pub fn log_deposit_filter(&self) -> ethers::contract::builders::Event<M, LogDepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogFlashLoan` event"]
        pub fn log_flash_loan_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogFlashLoanFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogRegisterProtocol` event"]
        pub fn log_register_protocol_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogRegisterProtocolFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogSetMasterContractApproval` event"]
        pub fn log_set_master_contract_approval_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogSetMasterContractApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogStrategyDivest` event"]
        pub fn log_strategy_divest_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogStrategyDivestFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogStrategyInvest` event"]
        pub fn log_strategy_invest_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogStrategyInvestFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogStrategyLoss` event"]
        pub fn log_strategy_loss_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogStrategyLossFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogStrategyProfit` event"]
        pub fn log_strategy_profit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogStrategyProfitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogStrategyQueued` event"]
        pub fn log_strategy_queued_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogStrategyQueuedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogStrategySet` event"]
        pub fn log_strategy_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogStrategySetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogStrategyTargetPercentage` event"]
        pub fn log_strategy_target_percentage_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogStrategyTargetPercentageFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogTransfer` event"]
        pub fn log_transfer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogTransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogWhiteListMasterContract` event"]
        pub fn log_white_list_master_contract_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogWhiteListMasterContractFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogWithdraw` event"]
        pub fn log_withdraw_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogWithdrawFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, BentoBoxV1Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for BentoBoxV1<M> {
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
    #[ethevent(name = "LogDeploy", abi = "LogDeploy(address,bytes,address)")]
    pub struct LogDeployFilter {
        #[ethevent(indexed)]
        pub master_contract: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
        #[ethevent(indexed)]
        pub clone_address: ethers::core::types::Address,
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
    #[ethevent(name = "LogDeposit", abi = "LogDeposit(address,address,address,uint256,uint256)")]
    pub struct LogDepositFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(
        name = "LogFlashLoan",
        abi = "LogFlashLoan(address,address,uint256,uint256,address)"
    )]
    pub struct LogFlashLoanFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub fee_amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub receiver: ethers::core::types::Address,
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
    #[ethevent(name = "LogRegisterProtocol", abi = "LogRegisterProtocol(address)")]
    pub struct LogRegisterProtocolFilter {
        #[ethevent(indexed)]
        pub protocol: ethers::core::types::Address,
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
        name = "LogSetMasterContractApproval",
        abi = "LogSetMasterContractApproval(address,address,bool)"
    )]
    pub struct LogSetMasterContractApprovalFilter {
        #[ethevent(indexed)]
        pub master_contract: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "LogStrategyDivest", abi = "LogStrategyDivest(address,uint256)")]
    pub struct LogStrategyDivestFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "LogStrategyInvest", abi = "LogStrategyInvest(address,uint256)")]
    pub struct LogStrategyInvestFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "LogStrategyLoss", abi = "LogStrategyLoss(address,uint256)")]
    pub struct LogStrategyLossFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "LogStrategyProfit", abi = "LogStrategyProfit(address,uint256)")]
    pub struct LogStrategyProfitFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "LogStrategyQueued", abi = "LogStrategyQueued(address,address)")]
    pub struct LogStrategyQueuedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub strategy: ethers::core::types::Address,
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
    #[ethevent(name = "LogStrategySet", abi = "LogStrategySet(address,address)")]
    pub struct LogStrategySetFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub strategy: ethers::core::types::Address,
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
        name = "LogStrategyTargetPercentage",
        abi = "LogStrategyTargetPercentage(address,uint256)"
    )]
    pub struct LogStrategyTargetPercentageFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub target_percentage: ethers::core::types::U256,
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
    #[ethevent(name = "LogTransfer", abi = "LogTransfer(address,address,address,uint256)")]
    pub struct LogTransferFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
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
    #[ethevent(
        name = "LogWhiteListMasterContract",
        abi = "LogWhiteListMasterContract(address,bool)"
    )]
    pub struct LogWhiteListMasterContractFilter {
        #[ethevent(indexed)]
        pub master_contract: ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "LogWithdraw", abi = "LogWithdraw(address,address,address,uint256,uint256)")]
    pub struct LogWithdrawFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "OwnershipTransferred", abi = "OwnershipTransferred(address,address)")]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BentoBoxV1Events {
        LogDeployFilter(LogDeployFilter),
        LogDepositFilter(LogDepositFilter),
        LogFlashLoanFilter(LogFlashLoanFilter),
        LogRegisterProtocolFilter(LogRegisterProtocolFilter),
        LogSetMasterContractApprovalFilter(LogSetMasterContractApprovalFilter),
        LogStrategyDivestFilter(LogStrategyDivestFilter),
        LogStrategyInvestFilter(LogStrategyInvestFilter),
        LogStrategyLossFilter(LogStrategyLossFilter),
        LogStrategyProfitFilter(LogStrategyProfitFilter),
        LogStrategyQueuedFilter(LogStrategyQueuedFilter),
        LogStrategySetFilter(LogStrategySetFilter),
        LogStrategyTargetPercentageFilter(LogStrategyTargetPercentageFilter),
        LogTransferFilter(LogTransferFilter),
        LogWhiteListMasterContractFilter(LogWhiteListMasterContractFilter),
        LogWithdrawFilter(LogWithdrawFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for BentoBoxV1Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogDeployFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogDeployFilter(decoded));
            }
            if let Ok(decoded) = LogDepositFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogDepositFilter(decoded));
            }
            if let Ok(decoded) = LogFlashLoanFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogFlashLoanFilter(decoded));
            }
            if let Ok(decoded) = LogRegisterProtocolFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogRegisterProtocolFilter(decoded));
            }
            if let Ok(decoded) = LogSetMasterContractApprovalFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogSetMasterContractApprovalFilter(decoded));
            }
            if let Ok(decoded) = LogStrategyDivestFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogStrategyDivestFilter(decoded));
            }
            if let Ok(decoded) = LogStrategyInvestFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogStrategyInvestFilter(decoded));
            }
            if let Ok(decoded) = LogStrategyLossFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogStrategyLossFilter(decoded));
            }
            if let Ok(decoded) = LogStrategyProfitFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogStrategyProfitFilter(decoded));
            }
            if let Ok(decoded) = LogStrategyQueuedFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogStrategyQueuedFilter(decoded));
            }
            if let Ok(decoded) = LogStrategySetFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogStrategySetFilter(decoded));
            }
            if let Ok(decoded) = LogStrategyTargetPercentageFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogStrategyTargetPercentageFilter(decoded));
            }
            if let Ok(decoded) = LogTransferFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogTransferFilter(decoded));
            }
            if let Ok(decoded) = LogWhiteListMasterContractFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogWhiteListMasterContractFilter(decoded));
            }
            if let Ok(decoded) = LogWithdrawFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::LogWithdrawFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(BentoBoxV1Events::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BentoBoxV1Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BentoBoxV1Events::LogDeployFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogDepositFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogFlashLoanFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogRegisterProtocolFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogSetMasterContractApprovalFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogStrategyDivestFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogStrategyInvestFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogStrategyLossFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogStrategyProfitFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogStrategyQueuedFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogStrategySetFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogStrategyTargetPercentageFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogTransferFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogWhiteListMasterContractFilter(element) => element.fmt(f),
                BentoBoxV1Events::LogWithdrawFilter(element) => element.fmt(f),
                BentoBoxV1Events::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,address)` and selector `[247, 136, 138, 236]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,address)")]
    pub struct BalanceOfCall(pub ethers::core::types::Address, pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `batch` function with signature `batch(bytes[],bool)` and selector `[210, 66, 59, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "batch", abi = "batch(bytes[],bool)")]
    pub struct BatchCall {
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub revert_on_fail: bool,
    }
    #[doc = "Container type for all input parameters for the `batchFlashLoan` function with signature `batchFlashLoan(address,address[],address[],uint256[],bytes)` and selector `[244, 131, 179, 218]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "batchFlashLoan",
        abi = "batchFlashLoan(address,address[],address[],uint256[],bytes)"
    )]
    pub struct BatchFlashLoanCall {
        pub borrower: ethers::core::types::Address,
        pub receivers: ::std::vec::Vec<ethers::core::types::Address>,
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub data: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `deploy` function with signature `deploy(address,bytes,bool)` and selector `[31, 84, 36, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deploy", abi = "deploy(address,bytes,bool)")]
    pub struct DeployCall {
        pub master_contract: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
        pub use_create_2: bool,
    }
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit(address,address,address,uint256,uint256)` and selector `[2, 185, 68, 108]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deposit", abi = "deposit(address,address,address,uint256,uint256)")]
    pub struct DepositCall {
        pub token: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub share: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `flashLoan` function with signature `flashLoan(address,address,address,uint256,bytes)` and selector `[241, 103, 109, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "flashLoan", abi = "flashLoan(address,address,address,uint256,bytes)")]
    pub struct FlashLoanCall {
        pub borrower: ethers::core::types::Address,
        pub receiver: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `harvest` function with signature `harvest(address,bool,uint256)` and selector `[102, 198, 187, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "harvest", abi = "harvest(address,bool,uint256)")]
    pub struct HarvestCall {
        pub token: ethers::core::types::Address,
        pub balance: bool,
        pub max_change_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `masterContractApproved` function with signature `masterContractApproved(address,address)` and selector `[145, 224, 234, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "masterContractApproved", abi = "masterContractApproved(address,address)")]
    pub struct MasterContractApprovedCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `masterContractOf` function with signature `masterContractOf(address)` and selector `[186, 254, 79, 20]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "masterContractOf", abi = "masterContractOf(address)")]
    pub struct MasterContractOfCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `pendingStrategy` function with signature `pendingStrategy(address)` and selector `[81, 8, 165, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pendingStrategy", abi = "pendingStrategy(address)")]
    pub struct PendingStrategyCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `permitToken` function with signature `permitToken(address,address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[124, 81, 110, 148]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "permitToken",
        abi = "permitToken(address,address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitTokenCall {
        pub token: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `registerProtocol` function with signature `registerProtocol()` and selector `[174, 228, 209, 178]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "registerProtocol", abi = "registerProtocol()")]
    pub struct RegisterProtocolCall;
    #[doc = "Container type for all input parameters for the `setMasterContractApproval` function with signature `setMasterContractApproval(address,address,bool,uint8,bytes32,bytes32)` and selector `[192, 164, 124, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setMasterContractApproval",
        abi = "setMasterContractApproval(address,address,bool,uint8,bytes32,bytes32)"
    )]
    pub struct SetMasterContractApprovalCall {
        pub user: ethers::core::types::Address,
        pub master_contract: ethers::core::types::Address,
        pub approved: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setStrategy` function with signature `setStrategy(address,address)` and selector `[114, 203, 93, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setStrategy", abi = "setStrategy(address,address)")]
    pub struct SetStrategyCall {
        pub token: ethers::core::types::Address,
        pub new_strategy: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setStrategyTargetPercentage` function with signature `setStrategyTargetPercentage(address,uint64)` and selector `[62, 42, 157, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setStrategyTargetPercentage",
        abi = "setStrategyTargetPercentage(address,uint64)"
    )]
    pub struct SetStrategyTargetPercentageCall {
        pub token: ethers::core::types::Address,
        pub target_percentage: u64,
    }
    #[doc = "Container type for all input parameters for the `strategy` function with signature `strategy(address)` and selector `[34, 139, 253, 159]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "strategy", abi = "strategy(address)")]
    pub struct StrategyCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `strategyData` function with signature `strategyData(address)` and selector `[223, 35, 180, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "strategyData", abi = "strategyData(address)")]
    pub struct StrategyDataCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `toAmount` function with signature `toAmount(address,uint256,bool)` and selector `[86, 98, 49, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "toAmount", abi = "toAmount(address,uint256,bool)")]
    pub struct ToAmountCall {
        pub token: ethers::core::types::Address,
        pub share: ethers::core::types::U256,
        pub round_up: bool,
    }
    #[doc = "Container type for all input parameters for the `toShare` function with signature `toShare(address,uint256,bool)` and selector `[218, 81, 57, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "toShare", abi = "toShare(address,uint256,bool)")]
    pub struct ToShareCall {
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub round_up: bool,
    }
    #[doc = "Container type for all input parameters for the `totals` function with signature `totals(address)` and selector `[79, 254, 52, 219]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totals", abi = "totals(address)")]
    pub struct TotalsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,address,address,uint256)` and selector `[241, 141, 3, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,address,address,uint256)")]
    pub struct TransferCall {
        pub token: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub share: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferMultiple` function with signature `transferMultiple(address,address,address[],uint256[])` and selector `[15, 202, 136, 67]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "transferMultiple",
        abi = "transferMultiple(address,address,address[],uint256[])"
    )]
    pub struct TransferMultipleCall {
        pub token: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub tos: ::std::vec::Vec<ethers::core::types::Address>,
        pub shares: ::std::vec::Vec<ethers::core::types::U256>,
    }
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
    #[doc = "Container type for all input parameters for the `whitelistMasterContract` function with signature `whitelistMasterContract(address,bool)` and selector `[115, 58, 157, 124]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "whitelistMasterContract", abi = "whitelistMasterContract(address,bool)")]
    pub struct WhitelistMasterContractCall {
        pub master_contract: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `whitelistedMasterContracts` function with signature `whitelistedMasterContracts(address)` and selector `[18, 169, 12, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "whitelistedMasterContracts", abi = "whitelistedMasterContracts(address)")]
    pub struct WhitelistedMasterContractsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(address,address,address,uint256,uint256)` and selector `[151, 218, 109, 48]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,address,address,uint256,uint256)")]
    pub struct WithdrawCall {
        pub token: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub share: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BentoBoxV1Calls {
        DomainSeparator(DomainSeparatorCall),
        BalanceOf(BalanceOfCall),
        Batch(BatchCall),
        BatchFlashLoan(BatchFlashLoanCall),
        ClaimOwnership(ClaimOwnershipCall),
        Deploy(DeployCall),
        Deposit(DepositCall),
        FlashLoan(FlashLoanCall),
        Harvest(HarvestCall),
        MasterContractApproved(MasterContractApprovedCall),
        MasterContractOf(MasterContractOfCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        PendingStrategy(PendingStrategyCall),
        PermitToken(PermitTokenCall),
        RegisterProtocol(RegisterProtocolCall),
        SetMasterContractApproval(SetMasterContractApprovalCall),
        SetStrategy(SetStrategyCall),
        SetStrategyTargetPercentage(SetStrategyTargetPercentageCall),
        Strategy(StrategyCall),
        StrategyData(StrategyDataCall),
        ToAmount(ToAmountCall),
        ToShare(ToShareCall),
        Totals(TotalsCall),
        Transfer(TransferCall),
        TransferMultiple(TransferMultipleCall),
        TransferOwnership(TransferOwnershipCall),
        WhitelistMasterContract(WhitelistMasterContractCall),
        WhitelistedMasterContracts(WhitelistedMasterContractsCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for BentoBoxV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Batch(decoded));
            }
            if let Ok(decoded) =
                <BatchFlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::BatchFlashLoan(decoded));
            }
            if let Ok(decoded) =
                <ClaimOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::ClaimOwnership(decoded));
            }
            if let Ok(decoded) = <DeployCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Deploy(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <HarvestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Harvest(decoded));
            }
            if let Ok(decoded) =
                <MasterContractApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::MasterContractApproved(decoded));
            }
            if let Ok(decoded) =
                <MasterContractOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::MasterContractOf(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::PendingOwner(decoded));
            }
            if let Ok(decoded) =
                <PendingStrategyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::PendingStrategy(decoded));
            }
            if let Ok(decoded) =
                <PermitTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::PermitToken(decoded));
            }
            if let Ok(decoded) =
                <RegisterProtocolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::RegisterProtocol(decoded));
            }
            if let Ok(decoded) =
                <SetMasterContractApprovalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BentoBoxV1Calls::SetMasterContractApproval(decoded));
            }
            if let Ok(decoded) =
                <SetStrategyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::SetStrategy(decoded));
            }
            if let Ok(decoded) =
                <SetStrategyTargetPercentageCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BentoBoxV1Calls::SetStrategyTargetPercentage(decoded));
            }
            if let Ok(decoded) =
                <StrategyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Strategy(decoded));
            }
            if let Ok(decoded) =
                <StrategyDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::StrategyData(decoded));
            }
            if let Ok(decoded) =
                <ToAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::ToAmount(decoded));
            }
            if let Ok(decoded) =
                <ToShareCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::ToShare(decoded));
            }
            if let Ok(decoded) = <TotalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Totals(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferMultipleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::TransferMultiple(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <WhitelistMasterContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::WhitelistMasterContract(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedMasterContractsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BentoBoxV1Calls::WhitelistedMasterContracts(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BentoBoxV1Calls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BentoBoxV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                BentoBoxV1Calls::DomainSeparator(element) => element.encode(),
                BentoBoxV1Calls::BalanceOf(element) => element.encode(),
                BentoBoxV1Calls::Batch(element) => element.encode(),
                BentoBoxV1Calls::BatchFlashLoan(element) => element.encode(),
                BentoBoxV1Calls::ClaimOwnership(element) => element.encode(),
                BentoBoxV1Calls::Deploy(element) => element.encode(),
                BentoBoxV1Calls::Deposit(element) => element.encode(),
                BentoBoxV1Calls::FlashLoan(element) => element.encode(),
                BentoBoxV1Calls::Harvest(element) => element.encode(),
                BentoBoxV1Calls::MasterContractApproved(element) => element.encode(),
                BentoBoxV1Calls::MasterContractOf(element) => element.encode(),
                BentoBoxV1Calls::Nonces(element) => element.encode(),
                BentoBoxV1Calls::Owner(element) => element.encode(),
                BentoBoxV1Calls::PendingOwner(element) => element.encode(),
                BentoBoxV1Calls::PendingStrategy(element) => element.encode(),
                BentoBoxV1Calls::PermitToken(element) => element.encode(),
                BentoBoxV1Calls::RegisterProtocol(element) => element.encode(),
                BentoBoxV1Calls::SetMasterContractApproval(element) => element.encode(),
                BentoBoxV1Calls::SetStrategy(element) => element.encode(),
                BentoBoxV1Calls::SetStrategyTargetPercentage(element) => element.encode(),
                BentoBoxV1Calls::Strategy(element) => element.encode(),
                BentoBoxV1Calls::StrategyData(element) => element.encode(),
                BentoBoxV1Calls::ToAmount(element) => element.encode(),
                BentoBoxV1Calls::ToShare(element) => element.encode(),
                BentoBoxV1Calls::Totals(element) => element.encode(),
                BentoBoxV1Calls::Transfer(element) => element.encode(),
                BentoBoxV1Calls::TransferMultiple(element) => element.encode(),
                BentoBoxV1Calls::TransferOwnership(element) => element.encode(),
                BentoBoxV1Calls::WhitelistMasterContract(element) => element.encode(),
                BentoBoxV1Calls::WhitelistedMasterContracts(element) => element.encode(),
                BentoBoxV1Calls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BentoBoxV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BentoBoxV1Calls::DomainSeparator(element) => element.fmt(f),
                BentoBoxV1Calls::BalanceOf(element) => element.fmt(f),
                BentoBoxV1Calls::Batch(element) => element.fmt(f),
                BentoBoxV1Calls::BatchFlashLoan(element) => element.fmt(f),
                BentoBoxV1Calls::ClaimOwnership(element) => element.fmt(f),
                BentoBoxV1Calls::Deploy(element) => element.fmt(f),
                BentoBoxV1Calls::Deposit(element) => element.fmt(f),
                BentoBoxV1Calls::FlashLoan(element) => element.fmt(f),
                BentoBoxV1Calls::Harvest(element) => element.fmt(f),
                BentoBoxV1Calls::MasterContractApproved(element) => element.fmt(f),
                BentoBoxV1Calls::MasterContractOf(element) => element.fmt(f),
                BentoBoxV1Calls::Nonces(element) => element.fmt(f),
                BentoBoxV1Calls::Owner(element) => element.fmt(f),
                BentoBoxV1Calls::PendingOwner(element) => element.fmt(f),
                BentoBoxV1Calls::PendingStrategy(element) => element.fmt(f),
                BentoBoxV1Calls::PermitToken(element) => element.fmt(f),
                BentoBoxV1Calls::RegisterProtocol(element) => element.fmt(f),
                BentoBoxV1Calls::SetMasterContractApproval(element) => element.fmt(f),
                BentoBoxV1Calls::SetStrategy(element) => element.fmt(f),
                BentoBoxV1Calls::SetStrategyTargetPercentage(element) => element.fmt(f),
                BentoBoxV1Calls::Strategy(element) => element.fmt(f),
                BentoBoxV1Calls::StrategyData(element) => element.fmt(f),
                BentoBoxV1Calls::ToAmount(element) => element.fmt(f),
                BentoBoxV1Calls::ToShare(element) => element.fmt(f),
                BentoBoxV1Calls::Totals(element) => element.fmt(f),
                BentoBoxV1Calls::Transfer(element) => element.fmt(f),
                BentoBoxV1Calls::TransferMultiple(element) => element.fmt(f),
                BentoBoxV1Calls::TransferOwnership(element) => element.fmt(f),
                BentoBoxV1Calls::WhitelistMasterContract(element) => element.fmt(f),
                BentoBoxV1Calls::WhitelistedMasterContracts(element) => element.fmt(f),
                BentoBoxV1Calls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for BentoBoxV1Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            BentoBoxV1Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for BentoBoxV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            BentoBoxV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BatchCall> for BentoBoxV1Calls {
        fn from(var: BatchCall) -> Self {
            BentoBoxV1Calls::Batch(var)
        }
    }
    impl ::std::convert::From<BatchFlashLoanCall> for BentoBoxV1Calls {
        fn from(var: BatchFlashLoanCall) -> Self {
            BentoBoxV1Calls::BatchFlashLoan(var)
        }
    }
    impl ::std::convert::From<ClaimOwnershipCall> for BentoBoxV1Calls {
        fn from(var: ClaimOwnershipCall) -> Self {
            BentoBoxV1Calls::ClaimOwnership(var)
        }
    }
    impl ::std::convert::From<DeployCall> for BentoBoxV1Calls {
        fn from(var: DeployCall) -> Self {
            BentoBoxV1Calls::Deploy(var)
        }
    }
    impl ::std::convert::From<DepositCall> for BentoBoxV1Calls {
        fn from(var: DepositCall) -> Self {
            BentoBoxV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<FlashLoanCall> for BentoBoxV1Calls {
        fn from(var: FlashLoanCall) -> Self {
            BentoBoxV1Calls::FlashLoan(var)
        }
    }
    impl ::std::convert::From<HarvestCall> for BentoBoxV1Calls {
        fn from(var: HarvestCall) -> Self {
            BentoBoxV1Calls::Harvest(var)
        }
    }
    impl ::std::convert::From<MasterContractApprovedCall> for BentoBoxV1Calls {
        fn from(var: MasterContractApprovedCall) -> Self {
            BentoBoxV1Calls::MasterContractApproved(var)
        }
    }
    impl ::std::convert::From<MasterContractOfCall> for BentoBoxV1Calls {
        fn from(var: MasterContractOfCall) -> Self {
            BentoBoxV1Calls::MasterContractOf(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for BentoBoxV1Calls {
        fn from(var: NoncesCall) -> Self {
            BentoBoxV1Calls::Nonces(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for BentoBoxV1Calls {
        fn from(var: OwnerCall) -> Self {
            BentoBoxV1Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingOwnerCall> for BentoBoxV1Calls {
        fn from(var: PendingOwnerCall) -> Self {
            BentoBoxV1Calls::PendingOwner(var)
        }
    }
    impl ::std::convert::From<PendingStrategyCall> for BentoBoxV1Calls {
        fn from(var: PendingStrategyCall) -> Self {
            BentoBoxV1Calls::PendingStrategy(var)
        }
    }
    impl ::std::convert::From<PermitTokenCall> for BentoBoxV1Calls {
        fn from(var: PermitTokenCall) -> Self {
            BentoBoxV1Calls::PermitToken(var)
        }
    }
    impl ::std::convert::From<RegisterProtocolCall> for BentoBoxV1Calls {
        fn from(var: RegisterProtocolCall) -> Self {
            BentoBoxV1Calls::RegisterProtocol(var)
        }
    }
    impl ::std::convert::From<SetMasterContractApprovalCall> for BentoBoxV1Calls {
        fn from(var: SetMasterContractApprovalCall) -> Self {
            BentoBoxV1Calls::SetMasterContractApproval(var)
        }
    }
    impl ::std::convert::From<SetStrategyCall> for BentoBoxV1Calls {
        fn from(var: SetStrategyCall) -> Self {
            BentoBoxV1Calls::SetStrategy(var)
        }
    }
    impl ::std::convert::From<SetStrategyTargetPercentageCall> for BentoBoxV1Calls {
        fn from(var: SetStrategyTargetPercentageCall) -> Self {
            BentoBoxV1Calls::SetStrategyTargetPercentage(var)
        }
    }
    impl ::std::convert::From<StrategyCall> for BentoBoxV1Calls {
        fn from(var: StrategyCall) -> Self {
            BentoBoxV1Calls::Strategy(var)
        }
    }
    impl ::std::convert::From<StrategyDataCall> for BentoBoxV1Calls {
        fn from(var: StrategyDataCall) -> Self {
            BentoBoxV1Calls::StrategyData(var)
        }
    }
    impl ::std::convert::From<ToAmountCall> for BentoBoxV1Calls {
        fn from(var: ToAmountCall) -> Self {
            BentoBoxV1Calls::ToAmount(var)
        }
    }
    impl ::std::convert::From<ToShareCall> for BentoBoxV1Calls {
        fn from(var: ToShareCall) -> Self {
            BentoBoxV1Calls::ToShare(var)
        }
    }
    impl ::std::convert::From<TotalsCall> for BentoBoxV1Calls {
        fn from(var: TotalsCall) -> Self {
            BentoBoxV1Calls::Totals(var)
        }
    }
    impl ::std::convert::From<TransferCall> for BentoBoxV1Calls {
        fn from(var: TransferCall) -> Self {
            BentoBoxV1Calls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferMultipleCall> for BentoBoxV1Calls {
        fn from(var: TransferMultipleCall) -> Self {
            BentoBoxV1Calls::TransferMultiple(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for BentoBoxV1Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            BentoBoxV1Calls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<WhitelistMasterContractCall> for BentoBoxV1Calls {
        fn from(var: WhitelistMasterContractCall) -> Self {
            BentoBoxV1Calls::WhitelistMasterContract(var)
        }
    }
    impl ::std::convert::From<WhitelistedMasterContractsCall> for BentoBoxV1Calls {
        fn from(var: WhitelistedMasterContractsCall) -> Self {
            BentoBoxV1Calls::WhitelistedMasterContracts(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for BentoBoxV1Calls {
        fn from(var: WithdrawCall) -> Self {
            BentoBoxV1Calls::Withdraw(var)
        }
    }
    #[doc = "Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,address)` and selector `[247, 136, 138, 236]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `batch` function with signature `batch(bytes[],bool)` and selector `[210, 66, 59, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BatchReturn {
        pub successes: ::std::vec::Vec<bool>,
        pub results: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all return fields from the `deploy` function with signature `deploy(address,bytes,bool)` and selector `[31, 84, 36, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DeployReturn {
        pub clone_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `deposit` function with signature `deposit(address,address,address,uint256,uint256)` and selector `[2, 185, 68, 108]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DepositReturn {
        pub amount_out: ethers::core::types::U256,
        pub share_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `masterContractApproved` function with signature `masterContractApproved(address,address)` and selector `[145, 224, 234, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MasterContractApprovedReturn(pub bool);
    #[doc = "Container type for all return fields from the `masterContractOf` function with signature `masterContractOf(address)` and selector `[186, 254, 79, 20]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MasterContractOfReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NoncesReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `pendingStrategy` function with signature `pendingStrategy(address)` and selector `[81, 8, 165, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingStrategyReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `strategy` function with signature `strategy(address)` and selector `[34, 139, 253, 159]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StrategyReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `strategyData` function with signature `strategyData(address)` and selector `[223, 35, 180, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StrategyDataReturn {
        pub strategy_start_date: u64,
        pub target_percentage: u64,
        pub balance: u128,
    }
    #[doc = "Container type for all return fields from the `toAmount` function with signature `toAmount(address,uint256,bool)` and selector `[86, 98, 49, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ToAmountReturn {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `toShare` function with signature `toShare(address,uint256,bool)` and selector `[218, 81, 57, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ToShareReturn {
        pub share: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `totals` function with signature `totals(address)` and selector `[79, 254, 52, 219]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalsReturn {
        pub elastic: u128,
        pub base: u128,
    }
    #[doc = "Container type for all return fields from the `whitelistedMasterContracts` function with signature `whitelistedMasterContracts(address)` and selector `[18, 169, 12, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WhitelistedMasterContractsReturn(pub bool);
    #[doc = "Container type for all return fields from the `withdraw` function with signature `withdraw(address,address,address,uint256,uint256)` and selector `[151, 218, 109, 48]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WithdrawReturn {
        pub amount_out: ethers::core::types::U256,
        pub share_out: ethers::core::types::U256,
    }
}
