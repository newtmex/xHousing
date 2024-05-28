#![no_std]
multiversx_sc::imports!();

pub mod storage;
pub mod types;

#[multiversx_sc::module]
pub trait UtilsModule {
    fn require_is_sc_addr(&self, addr: &ManagedAddress) {
        require!(
            self.blockchain().is_smart_contract(addr),
            "address is not smart contract"
        );
    }

    fn get_caller_as_user_address(&self) -> ManagedAddress {
        let caller = self.blockchain().get_caller();
        require!(
            !self.blockchain().is_smart_contract(&caller),
            "caller is smart contract"
        );

        caller
    }

    fn require_queried(&self) -> ManagedAddress {
        let caller = self.blockchain().get_caller();
        let sc_address = self.blockchain().get_sc_address();
        require!(
            caller == sc_address,
            "May only call this function through VM query"
        );

        self.queried().set(1);

        sc_address
    }

    fn call_is_query(&self) -> bool {
        self.queried().get() == 1
    }

    // Stores a handle to know if request is a query
    #[storage_mapper("utils_module::queried")]
    fn queried(&self) -> SingleValueMapper<u8>;
}
