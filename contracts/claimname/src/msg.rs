use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::VerificationRequest;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub verifier: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RequestVerification {
        twitter_handle: String,
        address: String,
        tweet_id: String,
    },

    // verifier only
    Verify {
        request_id: u64,
        approved: bool,
    },
    ChangeVerifier {
        new_verifier: String,
    },
    SetOwnerNftsAddress {
        ownernfts_address: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetVerificationRequest returns the verification request for a specific request_id
    GetVerificationRequest { request_id: u64 },
    // TODO add iterator query for verification requests
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetVerificationRequestResponse {
    pub request_id: u64,
    pub verification_request: VerificationRequest,
}

pub struct GetVerificationRequestsResponse {
    pub verification_requests: Vec<GetVerificationRequestResponse>,
}
