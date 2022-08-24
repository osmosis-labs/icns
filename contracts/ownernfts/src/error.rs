use cosmwasm_std::StdError;
use thiserror::Error;

// We use the query messages from `cw721_base`
pub type ContractError = cw721_base::ContractError;
