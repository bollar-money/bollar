use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, StdError, Uint128};
use cw20::{AllowanceResponse, BalanceResponse, TokenInfoResponse};

use crate::{models::Metadata, QueryResponse, StdResult};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BalanceResponse)]
    Balance { address: String },
    /// Implements CW20. Returns metadata on the contract - name, decimals, supply, etc.
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    /// Implements CW20 "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },

    #[returns(TotalSupplyCirculatingResponse)]
    TotalSupplyCirculating
}

#[cw_serde]
pub struct TotalSupplyCirculatingResponse {
    pub total_supply: Uint128,
    pub circulating_shares: Uint128,
}

impl TryFrom<TotalSupplyCirculatingResponse> for QueryResponse {
    type Error = StdError;

    fn try_from(resp: TotalSupplyCirculatingResponse) -> StdResult<Self> {
        to_json_binary(&resp)
    }
}

#[cw_serde]
pub struct MetadataResp {
    pub metadata: Metadata,
}

impl From<Metadata> for MetadataResp {
    fn from(metadata: Metadata) -> Self {
        Self { metadata }
    }
}

impl TryFrom<MetadataResp> for QueryResponse {
    type Error = StdError;

    fn try_from(resp: MetadataResp) -> StdResult<Self> {
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
