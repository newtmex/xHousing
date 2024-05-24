use data::UsersStorage;

mod data;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait UsersModule {
    #[view(getAffiliateDetails)]
    fn get_affiliate_details(
        &self,
        user_addr: &ManagedAddress,
    ) -> (usize, Option<(usize, ManagedAddress)>) {
        let store = self.users_storage();

        let user_id = store.get_user_id(user_addr);
        if user_id == 0 {
            return (0, None);
        }

        let referrer_data = store.get_referrer_data(user_id);

        (user_id, referrer_data)
    }

    fn create_or_get_user_id(
        &self,
        user_addr: &ManagedAddress,
        referrer_id: OptionalValue<usize>,
    ) -> usize {
        let store = self.users_storage();

        store.add_user(user_addr, referrer_id.into_option())
    }

    #[storage_mapper("users")]
    fn users_storage(&self) -> UsersStorage<Self::Api>;
}
