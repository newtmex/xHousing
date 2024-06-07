use multiversx_sc_modules::default_issue_callbacks;
use utils::{helpers::percent_share_factory, xpt_attributes::XPTokenAttributes};

use crate::{
    reward_sharing::{self, RewardShares},
    token,
};

multiversx_sc::imports!();

pub const DIVISION_SAFTETY_CONST: u64 = 1_000_000_000_000_000_000;

#[multiversx_sc::module]
pub trait RentsModule:
    xht::XHTModule
    + token::XPTokenModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
    + x_housing_module::XHousingModule
{
    #[payable("*")]
    #[endpoint(receiveRent)]
    fn receive_rent(&self) {
        let rent_payment = self.call_value().single_esdt();
        self.xht()
            .require_same_token(&rent_payment.token_identifier);
        // TODO set the appropriate rent per xProject
        require!(rent_payment.amount > 0, "Insufficient rent amount");

        {
            let mut rent_share = percent_share_factory::<Self::Api, _>(&rent_payment.amount, 100);

            let rent_reward = rent_share(75);
            let ecosystem_reward = rent_share(18);
            let facility_reward = rent_share(7);

            let all_shares = self.xp_token_max_supply().get();
            let rps_increase = (&rent_reward * DIVISION_SAFTETY_CONST) / all_shares;

            self.reward_per_share().update(|rps| *rps += rps_increase);
            self.rewards_reserve()
                .update(|reserve| *reserve += rent_reward);
            self.facility_management_funds()
                .update(|fund| *fund += facility_reward);

            self.xht().burn(&ecosystem_reward);
        }

        self.call_x_housing()
            .add_project_rent(
                self.xp_token().get_token_id(),
                rent_payment.amount,
                self.xp_token_max_supply().get(),
            )
            .sync_call();
    }

    #[payable("*")]
    #[endpoint(claimRentReward)]
    fn claim_rent_reward(&self) -> XPTokenAttributes<Self::Api> {
        let caller = self.blockchain().get_caller();
        let current_rps = self.reward_per_share().get();

        let mut xpt_payment = self.call_value().single_esdt();
        self.xp_token()
            .require_same_token(&xpt_payment.token_identifier);

        let mut xpt_attr: XPTokenAttributes<Self::Api> = self
            .xp_token()
            .get_token_attributes(xpt_payment.token_nonce);

        let reward_shares = self.compute_reward_shares(&xpt_attr);
        let mut total_reward = reward_shares.total();

        if total_reward == 0 {
            // Fail silently
            self.tx().to(&caller).payment(xpt_payment).transfer();
            return xpt_attr;
        }

        self.rewards_reserve().update(|reserve| {
            require!(*reserve >= total_reward, "Computed rewards too large");

            let RewardShares {
                user_value,
                referrer_value,
            } = reward_shares;
            *reserve -= core::mem::take(&mut total_reward);

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
            // Since this is an SFT, we create a new one
            xpt_payment = self.xp_token().nft_create(xpt_payment.amount, &xpt_attr);

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

    #[view(getRentClaimAble)]
    fn rent_cliamable(&self, attr: &XPTokenAttributes<Self::Api>) -> BigUint {
        self.compute_reward_shares(attr).user_value
    }

    fn compute_reward_shares(
        &self,
        attr: &XPTokenAttributes<Self::Api>,
    ) -> RewardShares<Self::Api> {
        let current_rps = self.reward_per_share().get();

        if current_rps == 0 || attr.reward_per_share >= current_rps {
            return RewardShares {
                user_value: 0u64.into(),
                referrer_value: 0u64.into(),
            };
        }

        let reward = reward_sharing::compute_reward(attr, &current_rps);
        reward_sharing::split_reward(&reward)
    }

    #[storage_mapper("rewards_reserve")]
    fn rewards_reserve(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("facility_management_funds")]
    fn facility_management_funds(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardPerShare)]
    #[storage_mapper("reward_per_share")]
    fn reward_per_share(&self) -> SingleValueMapper<BigUint>;
}
