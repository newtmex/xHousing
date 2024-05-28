#![no_std]

multiversx_sc::imports!();

pub use x_housing;

#[multiversx_sc::module]
pub trait XHousingModule {
    fn require_caller_is_x_housing(&self) -> ManagedAddress {
        let caller = self.blockchain().get_caller();
        require!(self.get_x_housing_addr() == caller, "not authorized");

        caller
    }

    fn get_x_housing_addr(&self) -> ManagedAddress {
        let store = self.x_housing_addr();
        require!(!store.is_empty(), "XHousing address not set");

        let addr = store.get();
        require!(
            self.blockchain().is_smart_contract(&addr),
            "Invalid XHousing address"
        );

        addr
    }

    fn set_x_housing_addr(&self, addr: ManagedAddress) {
        require!(
            self.blockchain().is_smart_contract(&addr),
            "Invalid XHousing address"
        );

        self.x_housing_addr().set(addr);
    }

    fn call_x_housing(&self) -> x_housing::ProxyTo<Self::Api> {
        self.x_housing_proxy(self.get_x_housing_addr())
    }

    #[proxy]
    fn x_housing_proxy(&self, address: ManagedAddress) -> x_housing::Proxy<Self::Api>;

    #[storage_mapper("x_housing_module::addr")]
    fn x_housing_addr(&self) -> SingleValueMapper<Self::Api, ManagedAddress>;
}
