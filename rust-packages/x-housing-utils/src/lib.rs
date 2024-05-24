#![no_std]
multiversx_sc::imports!();

pub mod storage;

#[multiversx_sc::module]
pub trait UtilsModule {
    fn get_caller_as_user_address(&self) -> ManagedAddress {
        let caller = self.blockchain().get_caller();
        require!(
            !self.blockchain().is_smart_contract(&caller),
            "caller is smart contract"
        );

        caller
    }
}
