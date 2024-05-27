use x_housing::coinbase_interaction::ProxyTrait as _;
use xht::{economics::emission::EmissionTrait, XHT};

multiversx_sc::imports!();

const FEED_INTERVAL: u64 = 7;

#[multiversx_sc::module]
pub trait XHousingModule: xht::XHTModule {
    /// `x_housing_address` is supplied only when doing genesis tx
    #[only_owner]
    #[endpoint]
    fn feed_x_housing(&self, x_housing_address: OptionalValue<ManagedAddress>) {
        let last_dispatch_mapper = self.x_housing_last_dispatch();
        let epoch_start = last_dispatch_mapper.get();
        let block_epoch = self.blockchain().get_block_epoch();

        if let Some(x_housing_address) = x_housing_address.into_option() {
            require!(epoch_start == 0, "genesis dispatch already done");

            self.x_housing_address().set(&x_housing_address);
            self.x_housing_genesis_epoch().set(block_epoch)
        }

        let epoch_start = last_dispatch_mapper.get();
        let current_epoch = block_epoch - self.x_housing_genesis_epoch().get();
        require!(epoch_start <= current_epoch, "feed epoch not reached");

        let mut epoch_end = current_epoch + FEED_INTERVAL;
        // Ensure it's multiple of FEED_INTERVAL
        epoch_end /= FEED_INTERVAL;
        epoch_end *= FEED_INTERVAL;

        // Ensure data integrity
        require!(epoch_end > epoch_start, "Epochs computation error");

        let amount_to_dispatch = XHT::emission_through_epoch_range(epoch_start, epoch_end);
        let xht_payment = self.make_xht_payment(amount_to_dispatch);

        let _: IgnoreValue = self
            .call_x_housing()
            .top_up_xht()
            .with_esdt_transfer(xht_payment)
            .execute_on_dest_context();

        last_dispatch_mapper.set(epoch_end);
    }

    fn get_x_housing_address(&self) -> ManagedAddress {
        let address = self.x_housing_address().get();
        require!(
            self.blockchain().is_smart_contract(&address),
            "invalid x_housing address"
        );

        address
    }

    fn call_x_housing(&self) -> x_housing::ProxyTo<Self::Api> {
        self.x_housing_proxy_obj(self.get_x_housing_address())
    }

    #[proxy]
    fn x_housing_proxy_obj(&self, address: ManagedAddress) -> x_housing::Proxy<Self::Api>;

    /// The last epoch that x_housing received XHT
    #[view(lastDispatchEpoch)]
    #[storage_mapper("x_housing::lastXHTsDispatch")]
    fn x_housing_last_dispatch(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("x_housing::address")]
    fn x_housing_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("x_housing::genesis_epoch")]
    fn x_housing_genesis_epoch(&self) -> SingleValueMapper<u64>;
}
