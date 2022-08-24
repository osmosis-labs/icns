use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: Addr,
    pub ownernfts_address: Addr,
}

pub const STATE: Item<State> = Item::new("state");

pub const NAMES_MAPPING: Map<&str, Map<&str, Addr>> = Map::new("names_mapping");
