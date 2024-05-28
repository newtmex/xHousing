#![no_std]

pub mod token;
pub mod x_project_proxy;

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
    token::XPTokenModule + default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(&self) {
        // There will be total property shares of 1,000,000 units
        self.xp_token_max_supply()
            .set_if_empty(BigUint::from(1_000_000u64))
    }

    #[upgrade]
    fn upgrade(&self) {}
}
