use anyhow::Result;
use babylon_bindings_test::BabylonApp;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{AppResponse, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, MetadataResponse, QueryMsg},
    StdResult,
};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Copy)]
pub struct IntentCodeId(u64);

impl IntentCodeId {
    pub fn store_code(app: &mut BabylonApp) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query);

        let code_id = app.store_code(Box::new(contract));

        IntentCodeId(code_id)
    }

    pub fn instantiate(
        self,
        app: &mut BabylonApp,
        bollar_vault: &str,
        leverage: u8,
        name: &str,
        sender: Addr,
        funds: Coin,
        label: &str,
    ) -> Result<IntentContract> {
        IntentContract::instantiate(
            app,
            self,
            bollar_vault,
            leverage,
            name,
            sender,
            funds,
            label,
        )
    }
}

impl From<IntentCodeId> for u64 {
    fn from(code_id: IntentCodeId) -> Self {
        code_id.0
    }
}

pub struct IntentContract(Addr);

impl IntentContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[track_caller]
    pub fn instantiate(
        app: &mut BabylonApp,
        code_id: IntentCodeId,
        bollar_vault: &str,
        leverage: u8,
        name: &str,
        sender: Addr,
        funds: Coin,
        label: &str,
    ) -> Result<Self> {
        let init_msg = InstantiateMsg::new(name.to_string(), leverage, bollar_vault.to_string());

        app.instantiate_contract(code_id.0, sender, &init_msg, &[funds], label, None)
            .map(Self::from)
    }

    pub fn stake(&self, app: &mut BabylonApp, sender: Addr, funds: &[Coin]) -> Result<AppResponse> {
        let msg = ExecuteMsg::Stake {};
        app.execute_contract(sender, self.addr(), &msg, funds)
    }

    pub fn query_metadata(&self, app: &BabylonApp) -> StdResult<MetadataResponse> {
        app.wrap()
            .query_wasm_smart(self.addr(), &QueryMsg::GetMetadata {})
    }

    pub fn query_balance(
        &self,
        app: &BabylonApp,
        address: String,
        denom: String,
    ) -> StdResult<Coin> {
        app.wrap().query_balance(address, denom)
    }
}

impl From<Addr> for IntentContract {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}
