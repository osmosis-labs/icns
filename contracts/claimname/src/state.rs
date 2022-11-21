use cosmwasm_std::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub ownernfts_address: Option<Addr>,
    pub verifier: Addr,
    pub next_request_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VerificationRequest {
    pub twitter_handle: String,
    pub owner_address: Addr,
    pub tweet_id: String,
    pub expiration_time: Timestamp,
}

pub const REQUESTS: Map<u64, VerificationRequest> = Map::new("requests");

pub const STATE: Item<State> = Item::new("state");
