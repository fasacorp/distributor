use cosmwasm_std::{Addr, Deps, StdResult, Uint128};

use crate::msg::{BalanceResponse, TotalStackedResponse};
use crate::state::{DEPOSITED, STATE};

/// Check the balance for the given address
pub fn balance(deps: Deps, address: Addr) -> StdResult<BalanceResponse> {
    let state = STATE.load(deps.storage)?;
    let potential_deposit = DEPOSITED.may_load(deps.storage, address.to_string())?;
    if let Some(deposit) = potential_deposit {
        return Ok(BalanceResponse {
            amount: deposit.amount,
            denom: state.stakable_denom,
        });
    }
    Ok(BalanceResponse {
        amount: Uint128::zero(),
        denom: state.stakable_denom,
    })
}

/// The total amount stacked in this contract
pub fn total_stacked(deps: Deps) -> StdResult<TotalStackedResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(TotalStackedResponse {
        amount: state.total_stacked,
        denom: state.stakable_denom,
    })
}
