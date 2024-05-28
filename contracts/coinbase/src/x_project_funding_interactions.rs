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
        let _: IgnoreValue = self
            .x_project_funding_proxy(x_project_funding_addr)
            .init_first_x_project(
                xproject_template,
                xhousing_address,
                funding_token_id,
                funding_goal,
                funding_deadline,
            )
            .with_esdt_transfer(self.make_xht_payment(XHT::ico_funds()))
            .execute_on_dest_context();
    }

    #[proxy]
    fn x_project_funding_proxy(&self, addr: ManagedAddress) -> x_project_funding::Proxy<Self::Api>;
}
