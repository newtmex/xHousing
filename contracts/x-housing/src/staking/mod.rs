use utils::types::Epoch;
use xst::{default_issue_callbacks, MAX_EPOCHS_LOCK, MIN_EPOCHS_LOCK};

use crate::{distribution, permission, users};
use xst::lk_xht;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StakeModule:
    distribution::DistributionModule
    + xht::XHTModule
    + xst::XstModule
    + utils::UtilsModule
    + coinbase_module::CoinbaseModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
    + users::UsersModule
    + permission::PermissionsModule
    + lk_xht::LkXhtModule
{
    #[payable("*")]
    #[endpoint]
    fn stake(&self, epochs_lock: Epoch, referrer_id: OptionalValue<usize>) {
        require!(
            (MIN_EPOCHS_LOCK..=MAX_EPOCHS_LOCK).contains(&epochs_lock),
            "Minimum Epochs Lock is {}, Maximum Epochs Lock is {}",
            MIN_EPOCHS_LOCK,
            MAX_EPOCHS_LOCK
        );

        let caller = self.get_caller_as_user_address();
        // Try create ID
        self.create_or_get_user_id(&caller, referrer_id);

        let distro_store = self.distribution_storage();
        distro_store.generate_rewards();

        let staking_tokens = self.call_value().all_esdt_transfers().clone_value();
        let new_xst_dets = self.mint_xst_token(
            staking_tokens,
            distro_store.get_projects_total_rewards(),
            distro_store.get_xht_reward_per_share(),
            epochs_lock,
            self.xht().get_token_id(),
        );

        distro_store.enter_staking(&new_xst_dets.attributes);
        self.tx()
            .to(&caller)
            .payment(new_xst_dets.payment)
            .sync_call();
    }
}
