use cosmwasm_std::{Binary, Empty};
use cw721::{ContractInfoResponse, Expiration};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// We use the execute messages from `cw721_base`
pub type InstantiateMsg = cw721_base::InstantiateMsg;

// We use the execute messages from `cw721_base`
pub type ExecuteMsg = cw721_base::ExecuteMsg<Empty>;

pub type MintMsg = cw721_base::MintMsg<Empty>;

// We use the query messages from `cw721_base`
pub type QueryMsg = cw721_base::QueryMsg;

pub type OwnerOfResponse = cw721::OwnerOfResponse;

// TODO add migrate msg
