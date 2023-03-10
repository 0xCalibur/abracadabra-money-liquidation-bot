use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
use ethers::{abi::Address, providers::Middleware, types::{TransactionReceipt, U256}};

use crate::bindings::cauldronv2::CauldronV2;


pub struct Liquidation {
    pub position: Address,
    pub max_borrow_part: U256
}

#[async_trait]
pub trait LiquidationExecutor {
    async fn setup(&self) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&self, cauldron_address: Address, swapper_address: Address, liquidations: Vec<Liquidation>) -> anyhow::Result<TransactionReceipt>;
    async fn cleanup(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct CauldronV2LiquidationExecutor<M> {
    pub client: Arc<M>
}

impl<M> CauldronV2LiquidationExecutor<M>
where
    M: Middleware
{
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}
#[async_trait]
impl<M: Middleware + 'static> LiquidationExecutor for CauldronV2LiquidationExecutor<M> {
    async fn execute(&self, cauldron_address: Address, swapper_address: Address, liquidations: Vec<Liquidation>) -> anyhow::Result<TransactionReceipt> {
        let cauldron = CauldronV2::new(cauldron_address, self.client.clone());
        let (users, max_borrow_parts) = liquidations.iter().map(|x|(x.position, x.max_borrow_part)).unzip();
        let liquidation_call = cauldron.liquidate(users, max_borrow_parts, swapper_address, swapper_address);
        let estimated_gas = liquidation_call.estimate_gas().await?;
        let liquidation_call = liquidation_call.gas(estimated_gas);
        let pending_tx = liquidation_call.send().await?;
        let tx_receipt = pending_tx.await?;
        tx_receipt.ok_or_else(||anyhow::anyhow!("Liquidation tx was dropped"))
    }
}
