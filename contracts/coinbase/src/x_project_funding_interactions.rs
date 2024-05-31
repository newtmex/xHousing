use x_housing_utils::contracts_proxy::x_project_funding_proxy;
use xht::{XHTTrait, XHT};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait XProjectFundingInteraction: xht::XHTModule {
    /// `x_housing_address` is supplied only when doing genesis tx
    #[only_owner]
    #[endpoint]
    fn start_ico(
        &self,
        x_project_funding_addr: ManagedAddress,
        xproject_template: ManagedAddress,
        xhousing_address: ManagedAddress,
        funding_token_id: EgldOrEsdtTokenIdentifier,
        funding_goal: BigUint,
        funding_deadline: u64,
    ) {
        self.tx()
            .to(x_project_funding_addr)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .init_first_x_project(
                xproject_template,
                xhousing_address,
                funding_token_id,
                funding_goal,
                funding_deadline,
            )
            .payment(self.make_xht_payment(XHT::ico_funds()))
            .sync_call();
    }
}
