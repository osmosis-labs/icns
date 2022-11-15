#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, StdError,
    StdResult, WasmQuery,
};
use cw2::set_contract_version;

use ownernfts::msg::{OwnerOfResponse, QueryMsg as QueryMsgNFT};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetNameResolutionResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:icns";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        admin: deps.api.addr_validate(&msg.admin)?,
        ownernfts_address: deps.api.addr_validate(&msg.ownernfts_address)?,
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", msg.admin)
        .add_attribute("ownernfts_address", msg.ownernfts_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetNameResolution {
            namespace,
            name,
            address,
        } => set_name_resolution(deps, info, namespace, name, address),
    }
}

pub fn set_name_resolution(
    deps: DepsMut,
    info: MessageInfo,
    namespace: String,
    name: String,
    address: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    let owner_of_res: OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.ownernfts_address.to_string(),
            msg: to_binary(&QueryMsgNFT::OwnerOf {
                token_id: name,
                include_expired: None,
            })?,
        }))?;

    if info.sender != deps.api.addr_validate(&owner_of_res.owner)? {
        return Err(ContractError::Unauthorized {});
    }

    // TODO: Finish this
    Ok(Response::new().add_attribute("method", "set_name_resolution"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNameResolution { name, namespace } => {
            to_binary(&get_name_resolution(deps, &name, &namespace)?)
        }
    }
}

fn get_name_resolution(
    deps: Deps,
    name: &str,
    namespace: &str,
) -> StdResult<GetNameResolutionResponse> {
    Err(StdError::GenericErr {
        msg: String::from("Not implemented"),
    })
    // name_to_address.load(deps.storage, name)
}
