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

    #[view(getUserAddress)]
    fn get_user_address(&self, id: usize) -> ManagedAddress {
        self.users_storage()
            .get_user_address(id)
            .unwrap_or_else(|| sc_panic!("No user with supplied id"))
    }

    #[view(getUserReferrals)]
    fn get_user_referrals(
        &self,
        id: usize,
    ) -> MultiValueEncoded<Self::Api, MultiValue2<usize, ManagedAddress<Self::Api>>> {
        self.users_storage().get_user_referrals(id)
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
