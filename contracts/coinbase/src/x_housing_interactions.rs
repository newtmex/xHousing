use xht::{XHTTrait, XHT};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait XHousingModule: xht::XHTModule + x_housing_module::XHousingModule {
    /// `x_housing_address` is supplied only when doing genesis tx
    #[only_owner]
    #[endpoint]
    fn feed_x_housing(&self, x_housing_address: OptionalValue<ManagedAddress>) {
        let last_dispatch_mapper = self.x_housing_is_dispatched();
        let is_dispatched = last_dispatch_mapper.get() > 0;

        if let Some(x_housing_address) = x_housing_address.into_option() {
            self.set_x_housing_addr(x_housing_address);
        }

        // Ensure data integrity
        require!(!is_dispatched, "Already dispatched");

        let amount_to_dispatch = XHT::ecosystem_distibution_funds();
        let xht_payment = self.make_xht_payment(amount_to_dispatch);

        let _: IgnoreValue = self
            .call_x_housing()
            .set_up_xht()
            .with_esdt_transfer(xht_payment)
            .execute_on_dest_context();

        last_dispatch_mapper.set(1);
    }

    /// The last epoch that x_housing received XHT
    #[view(lastDispatchEpoch)]
    #[storage_mapper("x_housing::isDispatched")]
    fn x_housing_is_dispatched(&self) -> SingleValueMapper<u64>;
}
