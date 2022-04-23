use cosmwasm_std::{coins, Addr, BankMsg, StdError, StdResult, SubMsg, Uint128, WasmMsg};
use cw20::{Cw20ExecuteMsg, Denom};

/// Stringify a denomination
pub fn denom_stringify(asset: &Denom) -> String {
    match asset {
        Denom::Cw20(token) => token.to_string(),
        Denom::Native(coin) => coin.clone(),
    }
}

/// send the given balance to the given recipient
pub fn send_balance(to: &Addr, amount: Uint128, asset: Denom) -> StdResult<SubMsg> {
    let res = match asset {
        Denom::Native(denom) => {
            if amount == Uint128::zero() {
                return Err(StdError::GenericErr {
                    msg: "Cannot send 0 amount".into(),
                });
            }
            SubMsg::new(BankMsg::Send {
                to_address: to.into(),
                amount: coins(amount.u128(), denom),
            })
        }
        Denom::Cw20(token) => {
            if amount == Uint128::zero() {
                return Err(StdError::GenericErr {
                    msg: "Cannot send 0 amount".into(),
                });
            }
            let msg = Cw20ExecuteMsg::Transfer {
                recipient: to.into(),
                amount: amount,
            };
            SubMsg::new(WasmMsg::Execute {
                contract_addr: token.to_string(),
                msg: cosmwasm_std::to_binary(&msg)?,
                funds: vec![],
            })
        }
    };

    Ok(res)
}
