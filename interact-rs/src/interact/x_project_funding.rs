use multiversx_sc::types::ReturnsNewAddress;
use multiversx_sc_snippets::imports::*;
use utils::contracts_proxy::x_project_funding_proxy::XProjectFundingProxy;

use super::Interact;

pub const DEPLOY_GAS: u64 = 500_000_000;
pub const FOLDER: &str = "../contracts/x-project-funding";

impl Interact {
    pub async fn deploy_x_project_funding(&mut self) {
        if self.state.x_project_funding_addr.is_some() {
            println!("xProject Funding address already set");
            return;
        }

        let code = self.build_contract(FOLDER);
        let coinbase_addr = self.get_coinbase_addr();

        let new_address = self
            .interactor
            .tx()
            .from(&self.contracts_owner)
            .gas(DEPLOY_GAS)
            .typed(XProjectFundingProxy)
            .init(coinbase_addr)
            .returns(ReturnsNewAddress)
            .code(code)
            .prepare_async()
            .run()
            .await;

        self.state.x_project_funding_addr = Some(Bech32Address::from_bech32_string(
            bech32::encode(&new_address),
        ));
    }

    pub fn get_x_project_funding_addr(&self) -> Bech32Address {
        self.state
            .x_project_funding_addr
            .clone()
            .unwrap_or_else(|| panic!("xProject Funding address not set"))
    }
}
