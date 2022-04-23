use crate::state::{Balance, DEPOSITED, EARNED, STATE};
use crate::utils::send_balance;
use crate::ContractError;
use cosmwasm_std::{Addr, Decimal, DepsMut, MessageInfo, Order, Response, StdResult, Uint128};
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
    // Generate the earnings per address
    // The amount a staker is entitled to is :
    // incensitive = total_received * (stacked / total stacked)

    // NOTE: necessary to disbale due to deps storage borrow.
    #[allow(clippy::needless_collect)]
    let earnings: Vec<(Addr, Uint128)> = DEPOSITED
        .range(deps.storage, None, None, Order::Ascending)
        .map(|entry| -> (Addr, Uint128) {
            let (_, deposit) = entry.unwrap();
            let ratio = Decimal::from_ratio(deposit.amount, state.total_stacked);
            let incensitive_amount = received.amount * ratio;
            (deposit.owner, incensitive_amount)
        })
        .collect();

    // Update earnings
    earnings.into_iter().for_each(|(owner, amount)| {
        EARNED
            .update(
                deps.storage,
                owner.to_string(),
                |maybe_balance| -> StdResult<Balance> {
                    let mut balance = match maybe_balance {
                        Some(balance) => balance,
                        None => Balance::new(owner),
                    };
                    balance.amount += amount;
                    Ok(balance)
                },
            )
            .unwrap();
    });

    Ok(Response::new().add_attribute("method", "deposit_rewards"))
}

/// claim any accrued rewards
pub fn claim(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let earnings = EARNED.may_load(deps.storage, info.sender.to_string())?;
    // we got nothing to send
    if earnings.is_none() {
        return Ok(Response::new());
    }
    let earnings = earnings.unwrap();
    let state = STATE.load(deps.storage)?;
    // generate transfer message
    let transfer_msg = send_balance(&earnings.owner, earnings.amount, state.incensitive_denom)?;
    // remove claimed earnings
    EARNED.remove(deps.storage, info.sender.to_string());
    Ok(Response::new().add_submessage(transfer_msg))
}
