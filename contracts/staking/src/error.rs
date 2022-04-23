use cosmwasm_std::{StdError, Uint128};
use cw20::Denom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unsupported deposit {}", denom)]
    UnsupportedDeposit { denom: String },

    #[error("No withdrawable balance found")]
    NoWithdrawableBalance {},

    #[error("Invalid withdrawal")]
    InvalidWithdrawal {
        available: Uint128,
        requested: Uint128,
    },

    #[error("No reward sent")]
    NoReward {},

    #[error("No reward sent")]
    InvalidRewardDenomination { expected: Denom, received: Denom },
}
