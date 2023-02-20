pub use cauldron_liquidator::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod cauldron_liquidator {
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
    #[doc = "CauldronLiquidator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"ErrSwapFailed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IBentoBoxV1\",\"name\":\"bentoBox\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"collateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICauldronV4\",\"name\":\"cauldron\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"oracleData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"accounts\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPositions\",\"outputs\":[{\"internalType\":\"struct Rebase\",\"name\":\"totalToken\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"elastic\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"base\",\"type\":\"uint128\",\"components\":[]}]},{\"internalType\":\"struct Rebase\",\"name\":\"totalBorrow\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"elastic\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"base\",\"type\":\"uint128\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"exchangeRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct CauldronLiquidator.UserPosition[]\",\"name\":\"positions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"borrowPart\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralShare\",\"type\":\"uint256\",\"components\":[]}]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static CAULDRONLIQUIDATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CAULDRONLIQUIDATOR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610987806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c8063575a806214610030575b600080fd5b61004361003e366004610595565b61005c565b604051610053949392919061066b565b60405180910390f35b6040805180820190915260008082526020820152604080518082019091526000808252602082015260006060886001600160a01b0316638285ef406040518163ffffffff1660e01b81526004016040805180830381865afa1580156100c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100e9919061073e565b92506000808a6001600160a01b031663b27c0e746040518163ffffffff1660e01b8152600401606060405180830381865afa15801561012c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061015091906107bf565b925050915060008267ffffffffffffffff164261016d9190610818565b9050801580159061018a575060208601516001600160801b031615155b156101e857670de0b6b3a7640000818367ffffffffffffffff1688600001516001600160801b03166101bc9190610831565b6101c69190610831565b6101d09190610850565b86516101dc9190610872565b6001600160801b031686525b8767ffffffffffffffff8111156102015761020161070c565b60405190808252806020026020018201604052801561024657816020015b604080518082019091526000808252602082015281526020019060019003908161021f5790505b50604051634ffe34db60e01b81526001600160a01b038f81166004830152919550908f1690634ffe34db906024016040805180830381865afa158015610290573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102b4919061073e565b96508b6001600160a01b0316637dc0d1d06040518163ffffffff1660e01b8152600401602060405180830381865afa1580156102f4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103189190610899565b6001600160a01b031663d39bbef08c8c6040518363ffffffff1660e01b81526004016103459291906108bd565b602060405180830381865afa158015610362573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061038691906108ec565b945060005b8881101561051f578c6001600160a01b03166348e4163e8b8b848181106103b4576103b4610905565b90506020020160208101906103c9919061091b565b6040516001600160e01b031960e084901b1681526001600160a01b039091166004820152602401602060405180830381865afa15801561040d573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061043191906108ec565b85828151811061044357610443610905565b6020908102919091010151526001600160a01b038d16631c9e379b8b8b8481811061047057610470610905565b9050602002016020810190610485919061091b565b6040516001600160e01b031960e084901b1681526001600160a01b039091166004820152602401602060405180830381865afa1580156104c9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104ed91906108ec565b8582815181106104ff576104ff610905565b60209081029190910181015101528061051781610938565b91505061038b565b50505050975097509750979350505050565b6001600160a01b038116811461054657600080fd5b50565b60008083601f84011261055b57600080fd5b50813567ffffffffffffffff81111561057357600080fd5b6020830191508360208260051b850101111561058e57600080fd5b9250929050565b600080600080600080600060a0888a0312156105b057600080fd5b87356105bb81610531565b965060208801356105cb81610531565b955060408801356105db81610531565b9450606088013567ffffffffffffffff808211156105f857600080fd5b818a0191508a601f83011261060c57600080fd5b81358181111561061b57600080fd5b8b602082850101111561062d57600080fd5b6020830196508095505060808a013591508082111561064b57600080fd5b506106588a828b01610549565b989b979a50959850939692959293505050565b600060c08201610691838880516001600160801b03908116835260209182015116910152565b60406106b58185018880516001600160801b03908116835260209182015116910152565b6080840186905260c060a085015284519182905260209160e085019083870160005b828110156106fc578151805185528601518685015292840192908501906001016106d7565b50919a9950505050505050505050565b634e487b7160e01b600052604160045260246000fd5b80516001600160801b038116811461073957600080fd5b919050565b60006040828403121561075057600080fd5b6040516040810181811067ffffffffffffffff8211171561078157634e487b7160e01b600052604160045260246000fd5b60405261078d83610722565b815261079b60208401610722565b60208201529392505050565b805167ffffffffffffffff8116811461073957600080fd5b6000806000606084860312156107d457600080fd5b6107dd846107a7565b92506107eb60208501610722565b91506107f9604085016107a7565b90509250925092565b634e487b7160e01b600052601160045260246000fd5b8181038181111561082b5761082b610802565b92915050565b600081600019048311821515161561084b5761084b610802565b500290565b60008261086d57634e487b7160e01b600052601260045260246000fd5b500490565b6001600160801b0381811683821601908082111561089257610892610802565b5092915050565b6000602082840312156108ab57600080fd5b81516108b681610531565b9392505050565b60208152816020820152818360408301376000818301604090810191909152601f909201601f19160101919050565b6000602082840312156108fe57600080fd5b5051919050565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561092d57600080fd5b81356108b681610531565b60006001820161094a5761094a610802565b506001019056fea26469706673582212203e1deb53d1d4775434f7197b8956708a7d9ccbe20b40dd7ea63fa06e8a79d4b964736f6c63430008100033" . parse () . expect ("invalid bytecode")
        });
    pub struct CauldronLiquidator<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CauldronLiquidator<M> {
        fn clone(&self) -> Self {
            CauldronLiquidator(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CauldronLiquidator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for CauldronLiquidator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CauldronLiquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ethers::providers::Middleware> CauldronLiquidator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CAULDRONLIQUIDATOR_ABI.clone(), client)
                .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                CAULDRONLIQUIDATOR_ABI.clone(),
                CAULDRONLIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getPositions` (0x575a8062) function"]
        pub fn get_positions(
            &self,
            bento_box: ethers::core::types::Address,
            collateral: ethers::core::types::Address,
            cauldron: ethers::core::types::Address,
            oracle_data: ethers::core::types::Bytes,
            accounts: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (Rebase, Rebase, ethers::core::types::U256, ::std::vec::Vec<UserPosition>),
        > {
            self.0
                .method_hash(
                    [87, 90, 128, 98],
                    (bento_box, collateral, cauldron, oracle_data, accounts),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CauldronLiquidator<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `ErrSwapFailed` with signature `ErrSwapFailed()` and selector `[252, 49, 22, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ErrSwapFailed", abi = "ErrSwapFailed()")]
    pub struct ErrSwapFailed;
    #[doc = "Container type for all input parameters for the `getPositions` function with signature `getPositions(address,address,address,bytes,address[])` and selector `[87, 90, 128, 98]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPositions", abi = "getPositions(address,address,address,bytes,address[])")]
    pub struct GetPositionsCall {
        pub bento_box: ethers::core::types::Address,
        pub collateral: ethers::core::types::Address,
        pub cauldron: ethers::core::types::Address,
        pub oracle_data: ethers::core::types::Bytes,
        pub accounts: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all return fields from the `getPositions` function with signature `getPositions(address,address,address,bytes,address[])` and selector `[87, 90, 128, 98]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPositionsReturn {
        pub total_token: Rebase,
        pub total_borrow: Rebase,
        pub exchange_rate: ethers::core::types::U256,
        pub positions: ::std::vec::Vec<UserPosition>,
    }
    #[doc = "`UserPosition(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UserPosition {
        pub borrow_part: ethers::core::types::U256,
        pub collateral_share: ethers::core::types::U256,
    }
    #[doc = "`Rebase(uint128,uint128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Rebase {
        pub elastic: u128,
        pub base: u128,
    }
}
