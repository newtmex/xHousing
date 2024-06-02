use utils::{contracts_proxy::x_project_proxy, types::Epoch};
use xst::{default_issue_callbacks, XstAttributes, MAX_EPOCHS_LOCK, MIN_EPOCHS_LOCK};

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
            distro_store.get_projects_reward_check_point(),
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

    #[payable("*")]
    #[endpoint(claimRewards)]
    fn claim_rewards(&self, referrer_id: OptionalValue<usize>) {
        let caller = self.blockchain().get_caller();
        // Try create new user
        self.create_or_get_user_id(&caller, referrer_id);

        let xst_mapper = self.xst();
        let xst = self.call_value().single_esdt();

        xst_mapper.require_same_token(&xst.token_identifier);
        let mut xst_attr: XstAttributes<Self::Api> =
            xst_mapper.get_token_attributes(xst.token_nonce);

        let distro_store = self.distribution_storage();
        distro_store.generate_rewards();

        let mut claimed_xht = distro_store.claim_rewards(&mut xst_attr);
        self.xst().nft_update_attributes(xst.token_nonce, &xst_attr);

        // Claim rent rewards from XProjects
        let xht_id = self.xht().get_token_id();
        for xpt in &xst_attr.x_project_tokens {
            let project_address = self
                .projects_token()
                .get_value(xpt.token_identifier.as_managed_buffer());
            let tokens = self
                .tx()
                .to(project_address)
                .typed(x_project_proxy::XProjectProxy)
                .claim_rent_reward()
                .payment(xpt)
                .returns(ReturnsBackTransfers)
                .sync_call();

            for payment in tokens.esdt_payments.iter() {
                if payment.token_identifier == xht_id {
                    claimed_xht += payment.amount
                }
            }
        }

        let mut return_tokens = ManagedVec::from_single_item(xst);

        if claimed_xht > 0 {
            let referrer_value = &claimed_xht * 25u64 / 1000u64;
            claimed_xht -= &referrer_value;

            // Do referrer ops
            let (_, referrer_dets) = self.get_affiliate_details(&caller);
            if let Some((_, referrer_addr)) = referrer_dets {
                self.tx()
                    .to(referrer_addr)
                    .payment(self.make_xht_payment(referrer_value))
                    .transfer();
            } else {
                self.xht().burn(&referrer_value)
            }

            return_tokens.push(self.make_xht_payment(claimed_xht));
        }

        self.tx().to(&caller).multi_esdt(return_tokens).transfer();
    }
}
