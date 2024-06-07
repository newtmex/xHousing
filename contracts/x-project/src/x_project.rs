#![no_std]

pub mod rents;
pub mod reward_sharing;
pub mod token;

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc_modules::default_issue_callbacks;

/// # xProject Contract Template
///
/// The `xProject` contract template serves as the foundational blueprint for deploying
/// individual real estate projects within the xHousing ecosystem.
/// Each `xProject` contract represents a unique real estate development,
/// managing its ownership, revenue distribution, and participant interactions.
#[multiversx_sc::contract]
pub trait XProject:
    token::XPTokenModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
    + rents::RentsModule
    + xht::XHTModule
    + x_housing_module::XHousingModule
{
    #[init]
    fn init(&self, x_housing_addr: ManagedAddress) {
        // There will be total property shares of 1,000,000 units
        self.xp_token_max_supply()
            .set_if_empty(BigUint::from(1_000_000u64));
        self.set_x_housing_addr(x_housing_addr);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
