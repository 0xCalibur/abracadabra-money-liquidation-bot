use std::{env, str::FromStr, sync::Arc};

use anyhow::anyhow;
use ethers::{
    abi::Address,
    prelude::{
        gas_escalator::{Frequency, GasEscalatorMiddleware, GeometricGasPrice},
        k256::ecdsa::SigningKey,
        NonceManagerMiddleware, SignerMiddleware,
    },
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer, Wallet},
    types::{BlockNumber, Bytes, U256},
};

use ethers::providers::StreamExt;

use crate::bindings::{
    bentoboxv1::BentoBoxV1, cauldronliquidator::CauldronLiquidator, cauldronv2::CauldronV2,
    erc20::ERC20,
};

pub type ClientTypeWs = NonceManagerMiddleware<
    SignerMiddleware<GasEscalatorMiddleware<Provider<Ws>, GeometricGasPrice>, Wallet<SigningKey>>,
>;

#[derive(Debug, Clone)]
pub struct Parameters<M> {
    pub collaterization_rate: U256,
    pub collateral_decimals: U256,
    pub liquidation_multiplier: U256,
    pub oracle_data: Bytes,
    pub users: Vec<Address>,
    pub collateral: ERC20<M>,
    pub cauldron: CauldronV2<M>,
    pub bentobox: BentoBoxV1<M>,
    pub cauldron_liquidator: CauldronLiquidator<M>,
    pub cauldron_address: Address,
    pub swapper_address: Address,
    pub ratio_increment_in_bips: u32,
}

pub struct ClientConfig {
    rpc: String,
    private_key: String,
}

pub fn get_client_config() -> anyhow::Result<ClientConfig> {
    let use_anvil = env::var("USE_ANVIL").unwrap_or_default();

    let client_config = if use_anvil == "1" {
        loggy::info!(">> Using Anvil");
        ClientConfig {
            rpc: "ws://127.0.0.1:8545".to_string(),
            private_key: env::var("LOCAL_PRIVATE_KEY")
                .or(Err(anyhow!("LOCAL_PRIVATE_KEY not found")))?,
        }
    } else {
        ClientConfig {
            rpc: env::var("RPC_URL").or(Err(anyhow!("RPC_URL not found")))?,
            private_key: env::var("PRIVATE_KEY").or(Err(anyhow!("PRIVATE_KEY not found")))?,
        }
    };

    return Ok(client_config)
}

pub async fn create_client(
    client_config: &ClientConfig,
) -> anyhow::Result<Arc<ClientTypeWs>, anyhow::Error> {
    let ClientConfig { rpc, private_key } = client_config;
    let provider = Provider::new(Ws::connect(rpc).await?);
    let chain_id = provider.get_chainid().await?;

    loggy::info!("Connecting to {rpc}...");
    loggy::info!("Chain Id is {}", chain_id);

    // Sign transactions with a private key
    let signer = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());
    let address = signer.address();

    // Escalate gas prices
    let escalator = GeometricGasPrice::new(1.125, 60u64, None::<u64>);
    let provider = GasEscalatorMiddleware::new(provider, escalator, Frequency::PerBlock);

    let provider = SignerMiddleware::new(provider, signer);

    // Manage nonces locally
    let provider = NonceManagerMiddleware::new(provider, address);
    let client = Arc::new(provider);

    return Ok(client)
}

pub async fn initialize(
    client: &Arc<ClientTypeWs>,
) -> anyhow::Result<Parameters<ClientTypeWs>, anyhow::Error> {
    let cauldron_liquidation_address = env::var("CAULDRON_LIQUIDATOR")
        .expect("CAULDRON_LIQUIDATOR not found")
        .parse::<Address>()
        .expect("invalid cauldron liquidation lens address");
    let cauldron_address = env::var("CAULDRON")
        .expect("CAULDRON not found")
        .parse::<Address>()
        .expect("invalid cauldron address");
    let swapper_address = env::var("SWAPPER")
        .expect("SWAPPER not found")
        .parse::<Address>()
        .expect("invalid swapper address");
    let cauldron_deploy_block =
        u32::from_str(&env::var("CAULDRON_DEPLOY_BLOCK").expect("CAULDRON_DEPLOY_BLOCK not found"))
            .expect("Invalid BlockNumber");

    let ratio_increment_in_bips =
        u32::from_str(&env::var("RATIO_INCREMENT_IN_BIPS").unwrap_or(String::from("10")))
            .expect("Invalid BlockNumber");

    loggy::info!("Ratio Increment Bips: {}", ratio_increment_in_bips);

    let cauldron = CauldronV2::new(cauldron_address, client.clone());
    let bentobox = BentoBoxV1::new(cauldron.bento_box().call().await.unwrap(), client.clone());
    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();

    let mut users = Vec::new();

    loggy::info!("Last Block: {}", last_block);
    {
        let log_borrow_filter =
            cauldron.log_borrow_filter().from_block(cauldron_deploy_block).to_block(last_block);

        loggy::info!("Fetching Past LogBorrow...");
        let mut log_borrow_stream = log_borrow_filter.stream().await?;

        while let Some(Ok(borrow)) = log_borrow_stream.next().await {
            let user = Address::from(borrow.from);
            if !users.contains(&user) {
                users.push(user);
            }
        }
    }

    loggy::info!("Found {} borrower addresses", users.len());

    let collateral = ERC20::new(cauldron.collateral().call().await.unwrap(), client.clone());
    let mut decimals = collateral.decimals().call().await.unwrap();
    if decimals == 0 {
        decimals = 18;
    }
    let collateral_decimals = U256::exp10(usize::from(decimals));

    let parameters = Parameters {
        collaterization_rate: cauldron.collaterization_rate().call().await.unwrap(),
        collateral_decimals,
        liquidation_multiplier: cauldron.liquidation_multiplier().call().await.unwrap(),
        cauldron_liquidator: CauldronLiquidator::new(cauldron_liquidation_address, client.clone()),
        oracle_data: cauldron.oracle_data().call().await.unwrap(),
        users,
        cauldron,
        bentobox,
        collateral,
        cauldron_address,
        swapper_address,
        ratio_increment_in_bips,
    };

    return Ok(parameters)
}
