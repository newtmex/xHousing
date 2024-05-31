use data::DistributionStorage;
use xht::{XHTTrait, XHT};
use xst::{default_issue_callbacks, lk_xht};

use crate::permission;

multiversx_sc::imports!();

mod data;

#[multiversx_sc::module]
pub trait DistributionModule:
    xht::XHTModule
    + utils::UtilsModule
    + coinbase_module::CoinbaseModule
    + permission::PermissionsModule
    + xst::XstModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
    + lk_xht::LkXhtModule
{
    #[endpoint]
    #[payable("*")]
    fn set_up_xht(&self) {
        self.require_caller_is_coinbase();

        let xht_payment = self.call_value().single_esdt();
        let mut xht_mapper = self.xht();

        require!(xht_mapper.is_empty(), "XHT set already");
        assert!(
            xht_payment.amount == XHT::ecosystem_distibution_funds(),
            "must send all ecosystem funds"
        );

        xht_mapper.set_token_id(xht_payment.token_identifier);
        self.distribution_storage()
            .set_total_funds(xht_payment.amount);
    }

    #[endpoint]
    fn add_project_rent(
        &self,
        project_token_id: TokenIdentifier<Self::Api>,
        rent_amount: BigUint,
        max_shares: BigUint,
    ) {
        self.require_caller_is_x_project();
        let project_address = self.blockchain().get_caller();

        self.distribution_storage()
            .add_project_rent(&project_token_id, rent_amount, max_shares);

        self.projects_token()
            .insert(project_token_id.into_managed_buffer(), project_address);
    }

    #[storage_mapper("distribution-module:storage")]
    fn distribution_storage(&self) -> DistributionStorage<Self::Api>;
}
