use multiversx_sc::types::{ManagedAddress, ReturnsNewAddress};
use multiversx_sc_snippets::imports::*;
use utils::contracts_proxy::{coinbase_proxy, x_housing_proxy};

use super::Interact;

pub const DEPLOY_GAS: u64 = 500_000_000;
pub const FOLDER: &str = "../contracts/x-housing";

impl Interact {
    pub async fn deploy_x_housing(&mut self) {
        if self.state.x_housing_addr.is_some() {
            println!("XHousing address already set");
            return;
        }

        let code = self.build_contract(FOLDER);
        let coinbase_addr = self.get_coinbase_addr();
        let x_project_funding_addr = self.get_x_project_funding_addr();

        let new_address = self
            .interactor
            .tx()
            .from(&self.contracts_owner)
            .gas(DEPLOY_GAS)
            .typed(x_housing_proxy::XHousingProxy)
            .init(coinbase_addr, x_project_funding_addr)
            .returns(ReturnsNewAddress)
            .code(code)
            .prepare_async()
            .run()
            .await;

        self.state.x_housing_addr = Some(Bech32Address::from_bech32_string(bech32::encode(
            &new_address,
        )));
    }

    pub fn get_x_housing_addr(&self) -> Bech32Address {
        self.state
            .x_housing_addr
            .clone()
            .unwrap_or_else(|| panic!("XHousing address not set"))
    }
}
