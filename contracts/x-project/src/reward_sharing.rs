use multiversx_sc::{api::ManagedTypeApi, types::BigUint};
use utils::{helpers::percent_share_factory, xpt_attributes::XPTokenAttributes};

use crate::rents::DIVISION_SAFTETY_CONST;

pub struct RewardShares<M: ManagedTypeApi> {
    pub user_value: BigUint<M>,
    pub referrer_value: BigUint<M>,
}

pub fn split_reward<M: ManagedTypeApi>(reward: &BigUint<M>) -> RewardShares<M> {
    let mut percent_share = percent_share_factory::<M, _>(reward, 100_00u32);
    let referrer_value = percent_share(6_66); // would amount to approximately 5% of grand total
    let user_value = reward - &referrer_value;

    RewardShares {
        user_value,
        referrer_value,
    }
}

pub fn compute_reward<M: ManagedTypeApi>(
    xpt_attr: &XPTokenAttributes<M>,
    contract_rps: &BigUint<M>,
) -> BigUint<M> {
    if contract_rps <= &xpt_attr.reward_per_share {
        return 0u32.into();
    }

    (contract_rps - &xpt_attr.reward_per_share) * &xpt_attr.token_weight / DIVISION_SAFTETY_CONST
}
