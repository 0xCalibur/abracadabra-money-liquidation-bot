use std::sync::Arc;

use ethers::{
    abi::Address,
    providers::{Middleware, StreamExt, PubsubClient},
    types::{BlockNumber, U64},
};
use tokio::sync::mpsc;

use crate::bindings::cauldronv2::CauldronV2;

#[derive(Debug)]
pub enum Event {
    AddBorrower { address: Address },
    RemoveBorrowers { addresses: Vec<Address> },
    NewBlock { block_no: U64 },
}

pub async fn stream_blocks<M>(
    sender: mpsc::Sender<Event>,
    client: Arc<M>,
) -> anyhow::Result<()>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static
{
    let mut stream = client.subscribe_blocks().await?;

    while let Some(block) = stream.next().await {
        sender.send(Event::NewBlock { block_no: block.number.unwrap() }).await.unwrap();
    }

    Ok(())
}

pub async fn stream_borrows<M>(
    sender: mpsc::Sender<Event>,
    cauldron: CauldronV2<M>
) -> anyhow::Result<()>
where
    M: Middleware + 'static,
    M::Provider: PubsubClient,
{
    let last_block = cauldron.client().get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();

    let borrow_filter = cauldron.log_borrow_filter().from_block(last_block);

    let mut log_borrow_stream = borrow_filter.stream().await?;

    while let Some(Ok(borrow)) = log_borrow_stream.next().await {
        sender.send(Event::AddBorrower { address: borrow.from }).await.unwrap();
    }

    Ok(())
}
