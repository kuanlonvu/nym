// Copyright 2022 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::error::BackendError;
use crate::operations::simulate::FeeDetails;
use crate::state::WalletState;
use nym_types::currency::DecCoin;
use std::str::FromStr;
use validator_client::nymd::{AccountId, MsgSend};

#[tauri::command]
pub async fn simulate_send(
    address: &str,
    amount: DecCoin,
    state: tauri::State<'_, WalletState>,
) -> Result<FeeDetails, BackendError> {
    let guard = state.read().await;
    let amount_base = guard.attempt_convert_to_base_coin(amount.clone())?;

    let to_address = AccountId::from_str(address)?;
    let amount = vec![amount_base.into()];

    let client = guard.current_client()?;
    let from_address = client.nymd.address().clone();

    // TODO: I'm still not 100% convinced whether this should be exposed here or handled somewhere else in the client code
    let msg = MsgSend {
        from_address,
        to_address,
        amount,
    };

    let result = client.nymd.simulate(vec![msg]).await?;
    guard.create_detailed_fee(result)
}
