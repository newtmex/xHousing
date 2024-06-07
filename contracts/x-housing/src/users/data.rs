use multiversx_sc::{api::StorageMapperApi, storage::StorageKey};
use utils::storage::StorageBuilder;

multiversx_sc::imports!();

pub(super) struct UsersStorage<SA: StorageMapperApi> {
    base_key: StorageKey<SA>,
}

impl<SA: StorageMapperApi> UsersStorage<SA> {
    #[inline]
    pub(super) fn get_user_address(&self, id: usize) -> Option<ManagedAddress<SA>> {
        self.users().get_user_address(id)
    }

    /// Returns `user_id`
    pub(super) fn add_user(
        &self,
        address: &ManagedAddress<SA>,
        referrer_id: Option<usize>,
    ) -> usize {
        let user_mapper = self.users();
        let mut user_id = user_mapper.get_user_id(address);
        if user_id > 0 {
            return user_id;
        }

        user_id = user_mapper.get_or_create_user(address);
        if let Some(referrer_id) = referrer_id {
            // Referrer ID should be within range of older user IDs
            if referrer_id < 1 || referrer_id >= user_mapper.get_user_count() {
                self.panic(b"invalid referrer id")
            }

            self.user_referrals(referrer_id).push_back(user_id);
            self.referred_by().insert(user_id, referrer_id);
        }

        user_id
    }

    pub(super) fn get_referrer_data(&self, user_id: usize) -> Option<(usize, ManagedAddress<SA>)> {
        if user_id == 0 {
            self.panic(b"user not found")
        }

        self.referred_by()
            .get(&user_id)
            .map(|id| (id, self.get_user_address(id).unwrap()))
    }

    pub(super) fn get_user_id(&self, address: &ManagedAddress<SA>) -> usize {
        self.users().get_user_id(address)
    }

    pub(super) fn get_user_referrals(
        &self,
        id: usize,
    ) -> MultiValueEncoded<SA, MultiValue2<usize, ManagedAddress<SA>>> {
        let mut referrals = MultiValueEncoded::new();

        for node in self.user_referrals(id).into_iter() {
            let user_id = node.into_value();
            let addr = self
                .users()
                .get_user_address(user_id)
                .unwrap_or_else(|| self.panic(b"no referral with id"));
            referrals.push((user_id, addr).into());
        }

        return referrals;
    }
}

impl<SA: StorageMapperApi> StorageMapper<SA> for UsersStorage<SA> {
    fn new(base_key: StorageKey<SA>) -> Self {
        UsersStorage { base_key }
    }
}

impl<SA: StorageMapperApi> StorageBuilder<SA> for UsersStorage<SA> {
    fn get_base_key(&self) -> StorageKey<SA> {
        self.base_key.clone()
    }
}

/// StorageMappers
impl<SA: StorageMapperApi> UsersStorage<SA> {
    fn users(&self) -> UserMapper<SA> {
        UserMapper::new(self.get_base_key())
    }

    fn user_referrals(&self, id: usize) -> LinkedListMapper<SA, usize> {
        LinkedListMapper::new(self.build_key(b"_referrals", &id))
    }

    fn referred_by(&self) -> MapMapper<SA, usize, usize> {
        MapMapper::new(self.build_key_by_key(b"_referred_by"))
    }
}
