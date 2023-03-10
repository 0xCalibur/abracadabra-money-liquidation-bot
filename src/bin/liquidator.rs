use std::{env, path::Path, str::FromStr};

use ethers::{
    providers::Middleware,
    utils::{format_bytes32_string, parse_units},
};
use loggy::Loggy;

use abra_liquidator::{
    event_streams::{stream_blocks, stream_borrows, Event},
    liquidation::check_liquidations,
    session::{create_client, get_client_config, initialize},
};
use tokio::sync::mpsc;

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

    let client_config = get_client_config().unwrap();
    let client = create_client(&client_config).await.unwrap();

    let mut params = initialize(&client).await.unwrap();

    let (sender1, mut rx) = mpsc::channel(32);
    let sender2 = sender1.clone();
    let sender3 = sender1.clone();

    let sender = params.bentobox.client().default_sender().unwrap();
    let master_contract = params.cauldron.master_contract().call().await?;

    // TODO: Don't approve it and use set approval signature inside cook() for newer cauldronv4
    // contracts.
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

    let cauldron = params.cauldron.clone();
    let t1 = tokio::spawn(async move {
        stream_borrows(sender1, cauldron).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        stream_blocks(sender2, client).await.unwrap();
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
