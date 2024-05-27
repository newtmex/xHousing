multiversx_sc::imports!();

use multiversx_sc::{api::StorageMapperApi, storage::StorageKey};
use utils::storage::StorageBuilder;
use xht::economics::EntitiesValue;

pub struct DistributionStorage<SA: StorageMapperApi> {
    base_key: StorageKey<SA>,
}

impl<SA: StorageMapperApi> DistributionStorage<SA> {
    pub(crate) fn add_entity_funds(&self, value: EntitiesValue<SA>) {
        let store = self.entity_funds();
        if store.is_empty() {
            store.set(value)
        } else {
            store.update(|old_value| *old_value += value);
        }
    }
}

// Mappers
impl<SA: StorageMapperApi> DistributionStorage<SA> {
    fn entity_funds(&self) -> SingleValueMapper<SA, EntitiesValue<SA>> {
        SingleValueMapper::new(self.build_key_by_key(b"entity_funds"))
    }
}

impl<SA: StorageMapperApi> StorageMapper<SA> for DistributionStorage<SA> {
    fn new(base_key: StorageKey<SA>) -> Self {
        DistributionStorage { base_key }
    }
}

impl<SA: StorageMapperApi> StorageBuilder<SA> for DistributionStorage<SA> {
    fn get_base_key(&self) -> StorageKey<SA> {
        self.base_key.clone()
    }
}
