multiversx_sc::imports!();

use multiversx_sc::{api::StorageMapperApi, storage::StorageKey};

/// Used to easily build storage mappers
/// that hide some of their api
pub trait StorageBuilder<SA: StorageMapperApi> {
    fn get_base_key(&self) -> StorageKey<SA>;

    fn panic(&self, message: &[u8]) -> ! {
        SA::error_api_impl().signal_error(message);
    }

    fn build_key_optional<T: NestedEncode>(&self, key: Option<&[u8]>, id: Option<&T>) -> StorageKey<SA> {
        if let (None, None) = (&id, &key) {
            self.panic(b"either key or id must be set");
        }

        let mut built_key = self.get_base_key();

        if let Some(key) = key {
            built_key.append_bytes(key);
        }

        if let Some(id) = id {
            built_key.append_item(id);
        }

        built_key
    }

    fn build_key_by_key(&self, key: &[u8]) -> StorageKey<SA> {
        self.build_key_optional(Some(key), Option::<&usize>::None)
    }

    fn build_key_by_id<T: NestedEncode>(&self, id: &T) -> StorageKey<SA> {
        self.build_key_optional(None, Some(id))
    }

    fn build_key<T: NestedEncode>(&self, key: &[u8], id: &T) -> StorageKey<SA> {
        self.build_key_optional(Some(key), Some(id))
    }
}
