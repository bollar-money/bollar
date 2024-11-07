use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, StdError, Uint128};

use crate::{models::Metadata, QueryResponse, StdResult};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MetadataResponse)]
    GetMetadata {},
    // #[returns(DenomsResponse)]
    // GetDenoms {},
}

// #[cw_serde]
// pub struct AllIntentResponse {
//     pub intents: Vec<IntentInfo>,
// }

// #[cw_serde]
// pub struct IntentsOfResponse {
//     pub intents: Vec<IntentInfo>,
// }

// #[cw_serde]
// pub struct DenomsResponse {
//     pub denoms: Vec<String>,
// }

// impl TryFrom<DenomsResponse> for QueryResponse {
//     type Error = StdError;

//     fn try_from(resp: DenomsResponse) -> StdResult<Self> {
//         to_json_binary(&resp)
//     }
// }

#[cw_serde]
pub struct MetadataResponse {
    pub metadata: Metadata,
}

impl From<Metadata> for MetadataResponse {
    fn from(metadata: Metadata) -> Self {
        Self { metadata }
    }
}

impl TryFrom<MetadataResponse> for QueryResponse {
    type Error = StdError;

    fn try_from(resp: MetadataResponse) -> StdResult<Self> {
        to_json_binary(&resp)
    }
}

#[cw_serde]
#[derive(Default)]
pub struct BalanceOfResp {
    pub balance: Uint128,
}

impl From<Uint128> for BalanceOfResp {
    fn from(balance: Uint128) -> Self {
        Self { balance }
    }
}

impl TryFrom<BalanceOfResp> for QueryResponse {
    type Error = StdError;

    fn try_from(resp: BalanceOfResp) -> StdResult<Self> {
        to_json_binary(&resp)
    }
}
