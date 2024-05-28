#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CoinbaseModule {
    fn require_caller_is_coinbase(&self) -> ManagedAddress {
        let caller = self.blockchain().get_caller();

        require!(self.get_coinbase_address() == caller, "not authorized");

        caller
    }

    fn get_coinbase_address(&self) -> ManagedAddress<Self::Api> {
        let addr = self.coinbase_addr().get();

        require!(
            self.blockchain().is_smart_contract(&addr),
            "Invalid Coinbase address"
        );

        addr
    }

    #[storage_mapper("coinbase_module::addr")]
    fn coinbase_addr(&self) -> SingleValueMapper<Self::Api, ManagedAddress>;
}
