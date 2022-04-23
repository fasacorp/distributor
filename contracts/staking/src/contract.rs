use crate::{executions, queries};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use crate::utils::denom_stringify;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        incensitive_denom: msg.incensitive_denom,
        stakable_denom: msg.stakable_denom,
        total_stacked: Uint128::zero(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute(
            "incensitive asset",
            denom_stringify(&state.incensitive_denom),
        )
        .add_attribute("stackable asset", denom_stringify(&state.stakable_denom)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => executions::receive_cw20(deps, info, msg),
        ExecuteMsg::Withdraw { amount } => executions::withdraw_cw20(deps, info, amount),
        ExecuteMsg::Reward {} => executions::deposit_rewards(deps, info),
        ExecuteMsg::Claim {} => executions::claim(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => {
            let validate_address = deps.api.addr_validate(&address)?;
            to_binary(&queries::balance(deps, validate_address)?)
        }
        QueryMsg::TotalDeposited {} => to_binary(&queries::total_stacked(deps)?),
        QueryMsg::Earned { address } => {
            let validate_address = deps.api.addr_validate(&address)?;
            to_binary(&queries::earned(deps, validate_address)?)
        }

    }
}
