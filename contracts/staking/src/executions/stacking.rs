use crate::state::{Deposit, DEPOSITED};
use crate::utils::{denom_stringify, send_balance};
use crate::ContractError;
use crate::{msg::ReceiveMsg, state::STATE};
use cosmwasm_std::{from_binary, DepsMut, MessageInfo, Response, StdResult, Uint128};
use cw20::{Cw20ReceiveMsg, Denom};

/// Deposit cw20 token
fn deposit_cw20(
    deps: DepsMut,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // check what cw20 we received and how much we recived
    let denom_received = Denom::Cw20(info.sender);
    let amount_received = wrapper.amount;
    let sender = deps.api.addr_validate(&wrapper.sender)?;

    // reject deposit of an unexpected type
    let mut state = STATE.load(deps.storage)?;
    if state.stakable_denom != denom_received {
        return Err(ContractError::UnsupportedDeposit {
            denom: denom_stringify(&denom_received),
        });
    }
    // register the deposit we received, 2 cases:
    // 1. New deposit from the address => create a new entry
    // 2. Already has deposited => increment the deposit record
    DEPOSITED.update(
        deps.storage,
        sender.to_string(),
        |deposit| -> StdResult<Deposit> {
            let mut entry = deposit.unwrap_or_else(|| Deposit::new(sender));
            entry.amount += amount_received;
            Ok(entry)
        },
    )?;

    // increase the total deposited in this contract
    state.total_stacked += amount_received;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "deposit"))
}
/// Deposit the asset being stacked
pub fn receive_cw20(
    deps: DepsMut,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // decode the inner instructions
    let received: ReceiveMsg = from_binary(&wrapper.msg)?;

    match received {
        ReceiveMsg::Deposit {} => deposit_cw20(deps, info, wrapper),
    }
}

/// withdraw stacked cw20 assets
pub fn withdraw_cw20(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    // Try to load the deposit from the sender
    let potential_deposit = DEPOSITED.may_load(deps.storage, info.sender.to_string())?;
    if let Some(mut deposit) = potential_deposit {
        // ensure we are not withdrawing more than available for this address
        if deposit.amount > amount {
            return Err(ContractError::InvalidWithdrawal {
                available: deposit.amount,
                requested: amount,
            });
        }
        // send the cw20 token
        let transfer_msg = send_balance(&deposit.owner, amount, state.stakable_denom.clone())?;

        // update balance
        deposit.amount -= amount;

        // Decrease the total stacked
        state.total_stacked -= amount;
        STATE.save(deps.storage, &state)?;

        if deposit.amount.is_zero() {
            // we don't need this anymore
            DEPOSITED.remove(deps.storage, info.sender.to_string());
        } else {
            DEPOSITED.save(deps.storage, info.sender.to_string(), &deposit)?;
        }

        // done desu !
        Ok(Response::new()
            .add_attribute("method", "deposit")
            .add_submessage(transfer_msg)
            .add_attribute("new balance", deposit.amount))
    } else {
        // we have nothing for this address, return an err
        Err(ContractError::NoWithdrawableBalance {})
    }
}
