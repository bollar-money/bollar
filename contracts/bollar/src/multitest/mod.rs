use anyhow::Result;
use babylon_bindings_test::BabylonApp;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw20::AllowanceResponse;
use cw20_base::state::TokenInfo;
use cw_multi_test::{AppResponse, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    QueryResponse, StdResult,
};

#[cfg(test)]
pub mod tests;

#[derive(Clone, Debug, Copy)]
pub struct BollarCodeId(u64);

impl BollarCodeId {
    pub fn store_code(app: &mut BabylonApp) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query);

        let code_id = app.store_code(Box::new(contract));

        BollarCodeId(code_id)
    }

    pub fn instantiate(
        self,
        app: &mut BabylonApp,
        name: &str,
        symbol: &str,
        decimals: u8,
        amount: Uint128,
        sender: Addr,
        label: &str,
    ) -> Result<BollarContract> {
        BollarContract::instantiate(app, self, name, symbol, decimals, amount, sender, label)
    }
}

impl From<BollarCodeId> for u64 {
    fn from(code_id: BollarCodeId) -> Self {
        code_id.0
    }
}

pub struct BollarContract(Addr);

impl BollarContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut BabylonApp,
        code_id: BollarCodeId,
        name: &str,
        symbol: &str,
        decimals: u8,
        amount: Uint128,
        sender: Addr,
        label: &str,
    ) -> Result<Self> {
        let init_msg = InstantiateMsg::new(name, symbol, decimals, amount);

        app.instantiate_contract(code_id.0, sender, &init_msg, &[], label, None)
            .map(Self::from)
    }

    pub fn mint(
        &self,
        app: &mut BabylonApp,
        sender: Addr,
        _recipient: String,
        amount: Uint128,
        funds: &[Coin],
    ) -> Result<AppResponse> {
        let msg = ExecuteMsg::Mint { amount };
        app.execute_contract(sender, self.addr(), &msg, funds)
    }

    pub fn transfer(
        &self,
        app: &mut BabylonApp,
        sender: Addr,
        recipient: String,
        amount: Uint128,
        funds: &[Coin],
    ) -> Result<AppResponse> {
        let msg = ExecuteMsg::Transfer { recipient, amount };
        app.execute_contract(sender, self.addr(), &msg, funds)
    }

    pub fn set_exchange_rate(
        &self,
        app: &mut BabylonApp,
        sender: Addr,
        denom: String,
        rate: Uint128,
        funds: &[Coin],
    ) -> Result<AppResponse> {
        let msg = ExecuteMsg::SetExchangeRate { denom, rate };
        app.execute_contract(sender, self.addr(), &msg, funds)
    }

    pub fn exchange(
        &self,
        app: &mut BabylonApp,
        sender: Addr,
        // recipient: String,
        // amount_of_bollar: Uint128,
        funds: &[Coin],
    ) -> Result<AppResponse> {
        let msg = ExecuteMsg::Exchange { };
        app.execute_contract(sender, self.addr(), &msg, funds)
    }

    pub fn send(
        &self,
        app: &mut BabylonApp,
        sender: Addr,
        contract: String,
        amount: Uint128,
        funds: &[Coin],
    ) -> Result<AppResponse> {
        let msg = ExecuteMsg::Send {
            contract,
            amount,
            msg: QueryResponse::default(),
        };
        app.execute_contract(sender, self.addr(), &msg, funds)
    }

    pub fn token_info(&self, app: &BabylonApp) -> StdResult<TokenInfo> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::TokenInfo {})
    }

    pub fn query_balance(&self, app: &BabylonApp, address: String) -> StdResult<Uint128> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Balance { address })
    }

    pub fn query_exchange_rate(&self, app: &BabylonApp, denom: String) -> StdResult<Uint128> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::GetExchangeRate { denom })
    }

    pub fn allowance(
        &self,
        app: &BabylonApp,
        owner: String,
        spender: String,
    ) -> StdResult<AllowanceResponse> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::Allowance { owner, spender })
    }
}

impl From<Addr> for BollarContract {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}
