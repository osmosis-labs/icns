#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};
use cw2::set_contract_version;
use ownernfts::msg::{ExecuteMsg as ExecuteMsgNFT, MintMsg};

use crate::error::ContractError;
use crate::helpers::must_get_nft_address;
use crate::msg::{ExecuteMsg, GetVerificationRequestResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, VerificationRequest, REQUESTS, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:claimname";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        verifier: deps.api.addr_validate(&msg.verifier)?,
        nft_address: None,
        next_request_id: 0,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("verifier", msg.verifier))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RequestVerification {
            twitter_handle,
            address,
            tweet_id,
        } => request_verification(deps, env, twitter_handle, address, tweet_id),
        ExecuteMsg::Verify {
            request_id,
            approved,
        } => verify(deps, env, info, request_id, approved),
        ExecuteMsg::ChangeVerifier { new_verifier } => change_verifier(deps, info, new_verifier),
    }
}

pub fn request_verification(
    deps: DepsMut,
    env: Env,
    twitter_handle: String,
    address: String,
    tweet_id: u64,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    let request = VerificationRequest {
        twitter_handle,
        owner_address: deps.api.addr_validate(&address)?,
        tweet_id,
        expiration_time: env.block.time.plus_seconds(604800),
    };

    REQUESTS.save(deps.storage, state.next_request_id, &request);

    state.next_request_id += 1;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "request_verification")) // TODO: add more attributes
}

pub fn verify(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    request_id: u64,
    approved: bool,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    if info.sender != state.verifier {
        return Err(ContractError::Unauthorized {});
    }

    if !approved {
        REQUESTS.remove(deps.storage, request_id);
        return Ok(Response::new().add_attribute("action", "verify")); // TODO add more attributes
    }

    let request = REQUESTS.load(deps.storage, request_id)?;

    let mint_msg = WasmMsg::Execute {
        contract_addr: must_get_nft_address(state)?.to_string(),
        msg: to_binary(&ExecuteMsgNFT::Mint(MintMsg {
            token_id: request.twitter_handle,
            owner: request.owner_address.into_string(),
            token_uri: None,
            extension: Empty {},
        }))?,
        funds: vec![],
    };

    REQUESTS.remove(deps.storage, request_id);

    Ok(Response::new()
        .add_attribute("action", "verify") // TODO: add more attributes
        .add_message(mint_msg))
}

pub fn change_verifier(
    deps: DepsMut,
    info: MessageInfo,
    new_verifier: String,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    if info.sender != state.verifier {
        return Err(ContractError::Unauthorized {});
    }

    state.verifier = deps.api.addr_validate(&new_verifier)?;

    STATE.save(deps.storage, &state);

    Ok(Response::new()
        .add_attribute("action", "change_verifier")
        .add_attribute("new_verifier", new_verifier))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetVerificationRequest { request_id } => {
            to_binary(&get_verification_request(deps, request_id)?)
        }
    }
}

fn get_verification_request(
    deps: Deps,
    request_id: u64,
) -> StdResult<GetVerificationRequestResponse> {
    Ok(GetVerificationRequestResponse {
        request_id,
        verification_request: REQUESTS.load(deps.storage, request_id)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1000, "earth"));

        let msg = InstantiateMsg {
            verifier: "creator".to_string(),
        };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetCountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }
}
