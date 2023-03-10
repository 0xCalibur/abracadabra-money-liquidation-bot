use ethers::types::{Address, U256};
use rusty_money::{iso, Money};
use tokio::{sync::mpsc, time::sleep};

use anyhow::anyhow;

use crate::{
    bindings::cauldronliquidator::{Rebase, UserPosition},
    event_streams::Event,
    session::{ClientTypeWs, Parameters},
};
use std::{str::FromStr, time::Duration};

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

pub async fn check_liquidations(
    sender: mpsc::Sender<Event>,
    params: &Parameters<ClientTypeWs>,
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
    params: &Parameters<ClientTypeWs>,
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

        let tx_call = params.cauldron.liquidate(
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
