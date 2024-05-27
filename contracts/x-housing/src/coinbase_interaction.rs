use xht::{
    economics::{emission::EmissionTrait, EntitiesValue},
    XHT,
};

use crate::distribution;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CoinbaseInteraction:
    xht::XHTModule + distribution::DistributionModule + utils::UtilsModule
{
    #[endpoint]
    #[payable("*")]
    fn top_up_xht(&self) {
        self.require_caller_is_coinbase();

        let xht_payment = self.call_value().single_esdt();
        let mut xht_mapper = self.xht();

        if xht_mapper.is_empty() {
            assert!(
                xht_payment.amount >= XHT::epoch_zero_emission(),
                "must send atleast epoch zero amount"
            );

            xht_mapper.set_token_id(xht_payment.token_identifier);
        } else {
            self.xht().require_same_token(&xht_payment.token_identifier);
            require!(xht_payment.amount > 0, "Invalid sent tokens");
        }

        self.distribution_storage()
            .add_entity_funds(EntitiesValue::from_total_value(xht_payment.amount));

        // TODO TOP up staking rewards for distribution
    }

    fn require_caller_is_coinbase(&self) -> ManagedAddress {
        let caller = self.blockchain().get_caller();

        require!(self.get_coinbase_address() == caller, "not authorized");

        caller
    }

    fn get_coinbase_address(&self) -> ManagedAddress<Self::Api> {
        let addr = self.coinbase_addr().get();

        require!(!addr.is_zero(), "Invalid ls address");

        addr
    }

    #[storage_mapper("coinbase_addr")]
    fn coinbase_addr(&self) -> SingleValueMapper<Self::Api, ManagedAddress>;
}
