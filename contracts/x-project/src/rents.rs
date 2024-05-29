use multiversx_sc_modules::default_issue_callbacks;
use utils::helpers::percent_share_factory;

use crate::token;

multiversx_sc::imports!();

pub const DIVISION_SAFTETY_CONST: u64 = 1_000_000_000_000_000_000;

#[multiversx_sc::module]
pub trait RentsModule:
    xht::XHTModule + token::XPTokenModule + default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[payable("*")]
    #[endpoint(receiveRent)]
    fn receive_rent(&self) {
        let rent_payment = self.call_value().single_esdt();
        self.xht()
            .require_same_token(&rent_payment.token_identifier);
        // TODO set the appropriate rent per xProject
        require!(rent_payment.amount > 0, "Insufficient rent amount");

        let mut rent_share = percent_share_factory::<Self::Api, _>(&rent_payment.amount,100);

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

    #[storage_mapper("rewards_reserve")]
    fn rewards_reserve(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("facility_management_funds")]
    fn facility_management_funds(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardPerShare)]
    #[storage_mapper("reward_per_share")]
    fn reward_per_share(&self) -> SingleValueMapper<BigUint>;
}
