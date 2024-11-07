use anyhow::Result;
use babylon_bindings_test::BabylonApp;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_multi_test::{AppResponse, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, MetadataResponse, QueryMsg},
    QueryResponse, StdResult,
};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Copy)]
pub struct DBankCodeId(u64);

impl DBankCodeId {
    pub fn store_code(app: &mut BabylonApp) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query);

        let code_id = app.store_code(Box::new(contract));

        DBankCodeId(code_id)
    }

    pub fn instantiate(
        self,
        app: &mut BabylonApp,
        bollar_vault: &str,
        denoms: Vec<String>,
        sender: Addr,
        label: &str,
    ) -> Result<DBankContract> {
        DBankContract::instantiate(app, self, bollar_vault, denoms, sender, label)
    }
}

impl From<DBankCodeId> for u64 {
    fn from(code_id: DBankCodeId) -> Self {
        code_id.0
    }
}

pub struct DBankContract(Addr);

impl DBankContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut BabylonApp,
        code_id: DBankCodeId,
        bollar_vault: &str,
        denoms: Vec<String>,
        sender: Addr,
        label: &str,
    ) -> Result<Self> {
        let init_msg = InstantiateMsg::new(bollar_vault.to_string(), denoms);

        app.instantiate_contract(code_id.0, sender, &init_msg, &[], label, None)
            .map(Self::from)
    }

    pub fn stake(
        &self,
        app: &mut BabylonApp,
        sender: Addr,
        funds: &[Coin],
    ) -> Result<AppResponse> {
        let msg = ExecuteMsg::Stake {};
        app.execute_contract(sender, self.addr(), &msg, funds)
    }

    
    pub fn query_metadata(&self, app: &BabylonApp) -> StdResult<MetadataResponse> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::GetMetadata {})
    }

    pub fn query_denoms(&self, app: &BabylonApp) -> StdResult<Vec<String>> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::GetDenoms {})
    }

    pub fn query_balance(&self, app: &BabylonApp, address: String, denom: String) -> StdResult<Coin> {
        app.wrap()
            .query_balance(address, denom)
    }

}

impl From<Addr> for DBankContract {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}