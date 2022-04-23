use cosmwasm_std::{StdError, Uint128};
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
}
