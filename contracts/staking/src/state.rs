use cw20::Denom;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    /// The incensitive denomination for this contract
    pub incensitive_denom: Denom,

    /// cw20 token that can be stacked on this contract
    pub stakable_denom: Denom,

    /// The total amount deposited in this contract
    pub total_stacked: Uint128,
}
/// The contract state
pub const STATE: Item<State> = Item::new("state");

/// A deposit record used to keep track of who owns what and how much was earned
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Balance {
    pub amount: Uint128,
    pub owner: Addr,
}
impl Balance {
    /// create a new deposit entry
    pub fn new(owner: Addr) -> Balance {
        Balance {
            amount: Uint128::zero(),
            owner,
        }
    }
}

/// The stacked asset within this contract
pub const DEPOSITED: Map<String, Balance> = Map::new("deposited");

/// The earned reward
pub const EARNED: Map<String, Balance> = Map::new("earned");
