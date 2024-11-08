use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, StdError, Uint128};
use cw20::{AllowanceResponse, BalanceResponse, TokenInfoResponse};

use crate::{QueryResponse, StdResult};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TotalSupplyCirculatingResponse)]
    TotalSupplyCirculating {},

    #[returns(GetExchangeRateResponse)]
    GetExchangeRate { denom: String },

    // For Cw20
    #[returns(BalanceResponse)]
    Balance { address: String },
    /// Implements CW20. Returns metadata on the contract - name, decimals, supply, etc.
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    /// Implements CW20 "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },
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
pub struct GetExchangeRateResponse {
    pub rate: Uint128,
}

impl TryFrom<GetExchangeRateResponse> for QueryResponse {
    type Error = StdError;

    fn try_from(resp: GetExchangeRateResponse) -> StdResult<Self> {
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
