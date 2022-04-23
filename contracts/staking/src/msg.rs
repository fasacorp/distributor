use cosmwasm_std::Uint128;
use cw20::{Cw20ReceiveMsg, Denom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The incensitive denomination for this contract
    pub incensitive_denom: Denom,

    /// cw20 token that can be stacked on this contract
    pub stakable_denom: Denom,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Receive CW20 and process the inner message.
    Receive(Cw20ReceiveMsg),

    /// Withdraw the deposited assets
    Withdraw { amount: Uint128 },
}
/// The Receive message is used to handle CW20 being sent to this contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    /// Execute a deposit of the asset being stacked.
    Deposit {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get the balance for the given address, including accrued earnings
    Balance { address: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub amount: Uint128,
    pub denom: Denom,
    pub earned:  Uint128,
    pub earned_denom: Denom,
}
