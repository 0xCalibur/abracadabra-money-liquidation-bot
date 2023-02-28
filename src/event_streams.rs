use std::sync::Arc;

use ethers::{
    abi::Address,
    providers::{Middleware, StreamExt},
    types::{BlockNumber, U64},
};
use tokio::sync::mpsc;

use crate::{bindings::cauldronv2::CauldronV2, session::ClientTypeWs};

#[derive(Debug)]
pub enum Event {
    AddBorrower { address: Address },
    RemoveBorrowers { addresses: Vec<Address> },
    NewBlock { block_no: U64 },
}

pub async fn stream_blocks(
    sender: mpsc::Sender<Event>,
    client: Arc<ClientTypeWs>,
) -> anyhow::Result<()> {
    let mut stream = client.subscribe_blocks().await?;

    while let Some(block) = stream.next().await {
        sender.send(Event::NewBlock { block_no: block.number.unwrap() }).await.unwrap();
    }

    Ok(())
}

pub async fn stream_borrows(
    sender: mpsc::Sender<Event>,
    client: Arc<ClientTypeWs>,
    caudron_address: Address,
) -> anyhow::Result<()> {
    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();

    let cauldron = CauldronV2::new(caudron_address, client);
    let borrow_filter = cauldron.log_borrow_filter().from_block(last_block);

    let mut log_borrow_stream = borrow_filter.stream().await?;

    while let Some(Ok(borrow)) = log_borrow_stream.next().await {
        sender.send(Event::AddBorrower { address: borrow.from }).await.unwrap();
    }

    Ok(())
}
