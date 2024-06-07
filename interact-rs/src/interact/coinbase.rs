use multiversx_sc::types::ReturnsNewAddress;
use multiversx_sc_snippets::imports::*;
use utils::contracts_proxy::coinbase_proxy;

use crate::constants::ESDT_REG_COST;

use super::Interact;

pub const DEPLOY_GAS: u64 = 50_000_000;
pub const FOLDER: &str = "../contracts/coinbase";

impl Interact {
    pub async fn deploy_coinbase(&mut self) {
        if self.state.coinbase_addr.is_some() {
            println!("Coinbase address already set");
            return;
        }

        let code = self.build_contract(FOLDER);

        let new_address = self
            .interactor
            .tx()
            .from(&self.contracts_owner)
            .gas(DEPLOY_GAS)
            .typed(coinbase_proxy::CoinbaseProxy)
            .init()
            .returns(ReturnsNewAddress)
            .code(code)
            .prepare_async()
            .run()
            .await;

        self.state.coinbase_addr = Some(Bech32Address::from_bech32_string(bech32::encode(
            &new_address,
        )));
    }

    pub fn get_coinbase_addr(&self) -> Bech32Address {
        self.state
            .coinbase_addr
            .clone()
            .unwrap_or_else(|| panic!("Coinbase address not set"))
    }

    pub async fn register_xht(&mut self) {
        if self.state.xht_id.is_some() {
            println!("XHT ID already set");

            return;
        }

        let contract_addr = self.get_coinbase_addr();

        let xht_id = self
            .interactor
            .tx()
            .from(&self.contracts_owner)
            .gas(90_000_000)
            .to(contract_addr)
            .typed(coinbase_proxy::CoinbaseProxy)
            .register_xht()
            .egld(ESDT_REG_COST)
            // TODO testnet is not returning result
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await;

        self.state.xht_id = Some(xht_id);
    }

    // TODO this fails on testnet
    async fn query_xht_id(&mut self) -> String {
        let coinbase_addr = self.get_coinbase_addr();

        self.interactor
            .query()
            .to(&coinbase_addr)
            .typed(coinbase_proxy::CoinbaseProxy)
            .xht()
            .returns(ReturnsNewTokenIdentifier)
            .prepare_async()
            .run()
            .await
    }
}
