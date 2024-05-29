#![no_std]

pub mod rents;
pub mod reward_sharing;
pub mod token;
pub mod x_project_proxy;

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc_modules::default_issue_callbacks;
use reward_sharing::RewardShares;
use token::attributes::XPTokenAttributes;
use x_housing_module::x_housing::users::ProxyTrait as _;

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

    #[payable("*")]
    #[endpoint(claimRentReward)]
    fn claim_rent_reward(&self) -> XPTokenAttributes<Self::Api> {
        let caller = self.blockchain().get_caller();
        let current_rps = self.reward_per_share().get();

        let xpt_payment = self.call_value().single_esdt();
        self.xp_token()
            .require_same_token(&xpt_payment.token_identifier);

        let mut xpt_attr: XPTokenAttributes<Self::Api> = self
            .xp_token()
            .get_token_attributes(xpt_payment.token_nonce);

        require!(
            current_rps > 0 && xpt_attr.reward_per_share < current_rps,
            "No accumulated rewards yet"
        );

        let mut reward = reward_sharing::compute_reward(&xpt_attr, &current_rps);
        self.rewards_reserve().update(|reserve| {
            require!(*reserve >= reward, "Computed rewards too large");

            let RewardShares {
                user_value,
                referrer_value,
            } = reward_sharing::split_reward(&reward);
            *reserve -= core::mem::take(&mut reward);

            {
                // We use original owner since we are much more sure that this user is registered
                let (_, referrer_data) = self
                    .call_x_housing()
                    .get_affiliate_details(&xpt_attr.original_owner)
                    .returns(ReturnsResult)
                    .sync_call();

                if referrer_value > 0 {
                    if let Some(referrer) = referrer_data.map(|(_, address)| address) {
                        self.send_xht(&referrer, referrer_value);
                    } else {
                        // Add value to ecosystem
                        self.xht().burn(&referrer_value);
                    }
                }
            }

            xpt_attr.reward_per_share = current_rps;
            self.xp_token()
                .nft_update_attributes(xpt_payment.token_nonce, &xpt_attr);

            self.send_xht(&caller, user_value);
            // Return updated token
            self.send().direct_esdt(
                &caller,
                &xpt_payment.token_identifier,
                xpt_payment.token_nonce,
                &xpt_payment.amount,
            );
        });

        xpt_attr
    }
}
