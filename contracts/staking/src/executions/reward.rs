use crate::state::{DEPOSITED, STATE};
use crate::utils::send_balance;
use crate::ContractError;
use cosmwasm_std::{Decimal, DepsMut, MessageInfo, Order, Response, SubMsg};
use cw20::Denom;

/// Deposit the rewards
pub fn deposit_rewards(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    // validate we received a reward
    if info.funds.is_empty() {
        return Err(ContractError::NoReward {});
    }

    // validate the incensitive received is what we expect
    let received = &info.funds[0];
    let received_denom = Denom::Native(received.denom.clone());
    if received_denom != state.incensitive_denom {
        return Err(ContractError::InvalidRewardDenomination {
            expected: state.incensitive_denom,
            received: received_denom,
        });
    }
    // Generate all the transfers
    // The amount a staker is entitled to is :
    // incensitive = total_received * (stacked / total stacked)

    let transfers: Vec<SubMsg> = DEPOSITED
        .range(deps.storage, None, None, Order::Ascending)
        .map(|entry| -> SubMsg {
            let (_, deposit) = entry.unwrap();
            let ratio = Decimal::from_ratio(deposit.amount, state.total_stacked);
            let incensitive_amount = received.amount * ratio;
            send_balance(&deposit.owner, incensitive_amount, received_denom.clone()).unwrap()
        })
        .collect();

    Ok(Response::new()
        .add_attribute("method", "deposit_rewards")
        .add_submessages(transfers))
}
