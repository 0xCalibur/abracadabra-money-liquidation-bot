extern crate loggy;
use ethers::{
    abi::Address,
    prelude::{
        gas_escalator::{Frequency, GasEscalatorMiddleware, GeometricGasPrice},
        k256::ecdsa::SigningKey,
        SignerMiddleware, *,
    },
    providers::{Middleware, Provider, StreamExt, Ws},
    signers::LocalWallet,
    types::{BlockNumber, Bytes, Filter, ValueOrArray, H256, U256, U64},
    utils::{format_bytes32_string, keccak256, parse_units},
};

use loggy::Loggy;
use rusty_money::{iso, Money};

use std::{env, path::Path, str::FromStr, sync::Arc};
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
pub mod bindings;
pub mod rebase;
use anyhow::{anyhow, Result};

use crate::bindings::cauldronliquidator::{cauldron_liquidator::UserPosition, Rebase};
use bindings::{
    bentoboxv1::bento_box_v1::BentoBoxV1,
    cauldronliquidator::cauldron_liquidator::CauldronLiquidator,
    cauldronv2::cauldron_v2::CauldronV2, erc20::erc20::ERC20,
};

type ClientTypeWs = NonceManagerMiddleware<
    SignerMiddleware<GasEscalatorMiddleware<Provider<Ws>, GeometricGasPrice>, Wallet<SigningKey>>,
>;
type ClientTypeHttp = NonceManagerMiddleware<
    SignerMiddleware<GasEscalatorMiddleware<Provider<Http>, GeometricGasPrice>, Wallet<SigningKey>>,
>;

#[derive(Debug)]
enum Event {
    AddBorrower { address: Address },
    RemoveBorrowers { addresses: Vec<Address> },
    NewBlock { block_no: U64 },
}

#[derive(Debug, Clone)]
struct Parameters<M, N> {
    collaterization_rate: U256,
    collateral_decimals: U256,
    liquidation_multiplier: U256,
    oracle_data: Bytes,
    users: Vec<Address>,
    collateral: ERC20<M>,
    cauldron: CauldronV2<M>,
    cauldron_send: CauldronV2<N>,
    bentobox: BentoBoxV1<M>,
    cauldron_liquidator: CauldronLiquidator<M>,
    cauldron_address: Address,
    swapper_address: Address,
    ratio_increment_in_bips: u32,
}

async fn stream_blocks(
    sender: mpsc::Sender<Event>,
    client: Arc<ClientTypeWs>,
) -> anyhow::Result<()> {
    let mut stream = client.subscribe_blocks().await?;

    while let Some(block) = stream.next().await {
        sender.send(Event::NewBlock { block_no: block.number.unwrap() }).await.unwrap();
    }

    Ok(())
}

async fn stream_borrows(
    sender: mpsc::Sender<Event>,
    client: Arc<ClientTypeWs>,
    caudron_address: Address,
) -> anyhow::Result<()> {
    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    let borrow_filter = Filter::new().from_block(last_block).address(caudron_address).topic0(
        ValueOrArray::Value(H256::from(keccak256("LogBorrow(address,address,uint256,uint256)"))),
    );

    let mut stream = client.subscribe_logs(&borrow_filter).await?;

    while let Some(log) = stream.next().await {
        let address = Address::from(log.topics[1]);
        sender.send(Event::AddBorrower { address }).await.unwrap();
    }

    Ok(())
}

async fn create_client(
    pk: &String,
    rpc: &String,
) -> anyhow::Result<Arc<ClientTypeWs>, anyhow::Error> {
    let provider = Provider::new(Ws::connect(rpc).await?);
    let chain_id = provider.get_chainid().await?;

    loggy::info!("Chain Id is {}", chain_id);

    // Sign transactions with a private key
    let signer = pk.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());
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

async fn create_client_send(
    pk: &String,
    rpc: &String,
) -> anyhow::Result<Arc<ClientTypeHttp>, anyhow::Error> {
    let provider = Provider::<Http>::try_from(rpc)?;

    let chain_id = provider.get_chainid().await?;

    // Sign transactions with a private key
    let signer = pk.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64());
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

async fn create_clients() -> anyhow::Result<(Arc<ClientTypeWs>, Arc<ClientTypeHttp>), anyhow::Error>
{
    let pk: String;
    let use_anvil = env::var("USE_ANVIL").unwrap_or_default();
    let rpc: String;
    let rpc_send: String;

    if use_anvil == "1" {
        loggy::info!(">> Using Anvil");
        rpc = "ws://127.0.0.1:8545".to_string();
        rpc_send = rpc.clone();
        pk = env::var("LOCAL_PRIVATE_KEY").expect("LOCAL_PRIVATE_KEY not found");
    } else {
        rpc = env::var("RPC_URL").expect("RPC_URL not found");
        rpc_send = env::var("RPC_URL_SEND").expect("RPC_URL_SEND not found");
        pk = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found");
    }

    loggy::info!("Connecting to {rpc}...");

    let client_read = create_client(&pk, &rpc).await.unwrap();
    let client_send = create_client_send(&pk, &rpc_send).await.unwrap();

    return Ok((client_read, client_send))
}

async fn initialize(
    client: &Arc<ClientTypeWs>,
    client_send: &Arc<ClientTypeHttp>,
) -> anyhow::Result<Parameters<ClientTypeWs, ClientTypeHttp>, anyhow::Error> {
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
    let cauldron_send = CauldronV2::new(cauldron_address, client_send.clone());
    let bentobox = BentoBoxV1::new(cauldron.bento_box().call().await.unwrap(), client.clone());
    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();

    loggy::info!("Last Block: {}", last_block);
    let borrow_filter = Filter::new()
        .from_block(cauldron_deploy_block)
        .to_block(last_block)
        .address(cauldron_address)
        .topic0(ValueOrArray::Value(H256::from(keccak256(
            "LogBorrow(address,address,uint256,uint256)",
        ))));

    loggy::info!("Fetching Past LogBorrow...");
    let logs = client.get_logs(&borrow_filter).await?;
    loggy::info!("Found {} events", logs.len());

    let mut users = Vec::new();
    for log in logs {
        let user = Address::from(log.topics[1]);
        if !users.contains(&user) {
            users.push(user);
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
        cauldron_send,
        bentobox,
        collateral,
        cauldron_address,
        swapper_address,
        ratio_increment_in_bips,
    };

    return Ok(parameters)
}

fn get_solvency_state(
    total_token: &Rebase,
    total_borrow: &Rebase,
    exchange_rate: U256,
    collateralization_rate: U256,
    position: &UserPosition,
) -> (bool, U256, U256) {
    let share = position.collateral_share
        * U256::from(10000000000000_u64) // 1e13
        * collateralization_rate;

    let collateral_value = total_token.to_elastic(share, false);
    let borrow_value = (position.borrow_part * U256::from(total_borrow.elastic) * exchange_rate) /
        total_borrow.base;

    return (collateral_value >= borrow_value, collateral_value, borrow_value)
}

fn adjust_max_borrow(
    account: &Address,
    max_borrow: &U256,
    total_borrow: &Rebase,
    total_token: &Rebase,
    liquidation_multiplier: U256,
    exchange_rate: U256,
    user_collateral_share: &U256,
    percent_in_bips: U256,
) -> Result<U256, anyhow::Error> {
    let one_e23 = U256::from(100000000000000000000000_u128);

    let mut adjusted_borrow = user_collateral_share.clone();
    adjusted_borrow = total_token.to_elastic(adjusted_borrow, false);
    adjusted_borrow = (adjusted_borrow * one_e23) / (liquidation_multiplier * exchange_rate);
    adjusted_borrow = total_borrow.to_base(adjusted_borrow, false);

    if adjusted_borrow.gt(max_borrow) {
        adjusted_borrow = max_borrow.clone();
    }

    if percent_in_bips > U256::from(0) {
        adjusted_borrow = (adjusted_borrow * percent_in_bips) / 10_000;
    }

    loggy::debug!(
        "{:?} -> Adjusted borrow part {} -> {}, collateral share: {}",
        account,
        max_borrow,
        adjusted_borrow,
        user_collateral_share
    );

    Ok(adjusted_borrow)
}

async fn check_liquidations(
    sender: mpsc::Sender<Event>,
    params: &Parameters<ClientTypeWs, ClientTypeHttp>,
    min_borrow_part: U256,
) -> anyhow::Result<()> {
    let one_e_13 = U256::from(10000000000000_u64);
    let one_e_18 = U256::from(1000000000000000000_u64);
    let min_borrow_part_in_usd = min_borrow_part / one_e_18;

    loggy::debug!("Checking Positions...");
    let (total_token, total_borrow, exchange_rate, positions) = params
        .cauldron_liquidator
        .get_positions(
            params.bentobox.address(),
            params.collateral.address(),
            params.cauldron.address(),
            params.oracle_data.clone(),
            params.users.clone(),
        )
        .call()
        .await?
        as (Rebase, Rebase, U256, Vec<UserPosition>);

    let mut addresses_to_remove = Vec::new();
    let mut addresses_to_liquidate = Vec::new();
    let mut borrow_part_to_liquidate = Vec::new();
    let mut collateral_share_to_liquidate = Vec::new();

    for (i, position) in positions.iter().enumerate() {
        loggy::debug!(
            "borrower: {:?}, borrow_part: {}, collateral_share: {}",
            params.users[i],
            position.borrow_part,
            position.collateral_share
        );
        if position.borrow_part.is_zero() || position.collateral_share.is_zero() {
            addresses_to_remove.push(params.users[i]);
            continue
        }

        let (solvent, collateral_value, borrow_value) = get_solvency_state(
            &total_token,
            &total_borrow,
            exchange_rate,
            params.collaterization_rate,
            &position,
        );

        let collateral_value_in_usd = (collateral_value / one_e_13 / params.collaterization_rate) /
            params.collateral_decimals;

        if collateral_value_in_usd >= min_borrow_part_in_usd {
            let ltv = (f32::from_str(&collateral_value.to_string()).unwrap() /
                f32::from_str(&borrow_value.to_string()).unwrap()) *
                100_f32;

            if ltv - 100_f32 <= 0.1_f32 {
                loggy::info!(
                    "ALERT: {:?}, collateral value: {}, ltv: {:.5} %",
                    params.users[i],
                    Money::from_str(&collateral_value_in_usd.to_string(), iso::USD).unwrap(),
                    ltv
                );
            }
        }

        if !solvent {
            addresses_to_liquidate.push(params.users[i]);
            borrow_part_to_liquidate.push(position.borrow_part);
            collateral_share_to_liquidate.push(position.collateral_share);
        }
    }

    if addresses_to_remove.len() > 0 {
        sender.send(Event::RemoveBorrowers { addresses: addresses_to_remove }).await.unwrap();
    }

    if addresses_to_liquidate.len() > 0 {
        liquidate(
            addresses_to_liquidate,
            borrow_part_to_liquidate,
            params,
            total_borrow,
            total_token,
            exchange_rate,
            collateral_share_to_liquidate,
            min_borrow_part,
        )
        .await?;
    }

    Ok(())
}

async fn liquidate(
    accounts: Vec<Address>,
    borrow_parts: Vec<U256>,
    params: &Parameters<ClientTypeWs, ClientTypeHttp>,
    total_borrow: Rebase,
    total_token: Rebase,
    exchange_rate: U256,
    collateral_shares: Vec<U256>,
    min_borrow_part: U256,
) -> anyhow::Result<()> {
    loggy::debug!("liquidation candidates found: {:?} ", accounts);

    loggy::debug!(
        "exchange rate: {}, total borrow elastic: {}",
        exchange_rate,
        total_borrow.elastic
    );

    let mut ratio = 0;

    loop {
        let percent_in_bips = U256::from(10_000 - ratio);
        loggy::debug!("Trying with bips: {}", percent_in_bips);

        let adjusted_borrow_parts = borrow_parts
            .iter()
            .enumerate()
            .map(|(index, max_borrow)| {
                adjust_max_borrow(
                    &accounts[index],
                    max_borrow,
                    &total_borrow,
                    &total_token,
                    params.liquidation_multiplier,
                    exchange_rate,
                    &collateral_shares[index],
                    percent_in_bips,
                )
                .unwrap()
            })
            .collect::<Vec<U256>>();

        let mut selected_accounts: Vec<Address> = Vec::new();
        let mut selected_adjusted_borrow_parts: Vec<U256> = Vec::new();

        for (index, adjusted_borrow_amount) in adjusted_borrow_parts.iter().enumerate() {
            if adjusted_borrow_amount >= &min_borrow_part {
                selected_accounts.push(accounts[index]);
                selected_adjusted_borrow_parts.push(adjusted_borrow_amount.clone());
            }
        }

        if selected_accounts.len() == 0 {
            break
        }

        loggy::info!(
            "selected for liquidation: {:?}, adjusted_borrow_parts: {:?}",
            selected_accounts,
            selected_adjusted_borrow_parts
        );

        let tx_call = params.cauldron_send.liquidate(
            selected_accounts,
            selected_adjusted_borrow_parts,
            params.swapper_address,
            params.swapper_address,
        );

        let calldata = tx_call.calldata().unwrap();

        let gas = match tx_call.estimate_gas().await {
            Ok(gas) => gas,
            Err(err) => {
                loggy::debug!("Error while estimating: {}", err);

                ratio += params.ratio_increment_in_bips;

                if ratio >= 500 {
                    return Err(anyhow!("Cannot liquidate, ratio limit reached, 5%"))
                }

                sleep(Duration::from_millis(1000)).await;
                continue
            }
        };

        loggy::info!("calldata: {}", calldata);
        tx_call.gas(gas).send().await?.await?.and_then(|r| {
            loggy::info!("https://etherscan.io/tx/{:?}", r.transaction_hash);
            Some(r)
        });

        break
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args().nth(1).expect("no config given");
    let config_path = Path::new(&config_path);
    dotenv::from_path(config_path).ok();

    let prefix = config_path.file_stem().unwrap().to_str().unwrap().to_string();
    let prefix: &'static str = Box::leak(prefix.into_boxed_str());

    let logger = loggy::Loggy { prefix: &prefix, show_time: false, show_thread: false };
    let logger: &'static Loggy = Box::leak(logger.into());

    log::set_logger(logger).unwrap();

    let log_level_str = env::var("LOG_LEVEL").unwrap_or(String::from("Info"));
    log::set_max_level(log::LevelFilter::from_str(&log_level_str).expect("invalid log level"));

    let min_borrow_part =
        parse_units(env::var("MIN_BORROW_PART").expect("MIN_BORROW_PART not found"), "ether")
            .unwrap();

    let clients = create_clients().await.unwrap();
    let client = clients.0;
    let client_send = clients.1;

    let mut params = initialize(&client, &client_send).await.unwrap();

    let client1 = client.clone();
    let client2 = client.clone();
    let (sender1, mut rx) = mpsc::channel(32);
    let sender2 = sender1.clone();
    let sender3 = sender1.clone();

    let sender = params.bentobox.client().default_sender().unwrap();
    let master_contract = params.cauldron.master_contract().call().await?;

    // TODO: Don't approve it and use set approval signature inside cook() for newer cauldronv4 contracts.
    if !params.bentobox.master_contract_approved(master_contract, sender).call().await? {
        loggy::info!("Approving Master Contract...");
        params
            .bentobox
            .set_master_contract_approval(
                sender,
                master_contract,
                true,
                0,
                format_bytes32_string("").unwrap(),
                format_bytes32_string("").unwrap(),
            )
            .send()
            .await?
            .await?;
    }

    check_liquidations(sender1.clone(), &params, min_borrow_part).await?;

    let t1 = tokio::spawn(async move {
        stream_borrows(sender1, client1, params.cauldron_address).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        stream_blocks(sender2, client2).await.unwrap();
    });

    let event_handler = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Event::AddBorrower { address } => {
                    if !params.users.contains(&address) {
                        params.users.push(address);
                        loggy::info!("Registering Borrower {:?}", address);
                    } else {
                        loggy::info!("Borrower {:?} already registered", address);
                    }
                }
                Event::RemoveBorrowers { addresses } => {
                    params.users.retain(|k| !addresses.contains(k));
                    loggy::info!("Pruning Borrowers, New Size: {}", params.users.len());
                    loggy::debug!("Removed Addresses: {:?}", addresses);
                }
                Event::NewBlock { block_no } => {
                    loggy::debug!(">> blockNo: {}", block_no);
                    check_liquidations(sender3.clone(), &params, min_borrow_part)
                        .await
                        .map_err(|e| loggy::info!("check_liquidations error: {}", e.to_string()))
                        .ok();
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    event_handler.await.unwrap();

    Ok(())
}
