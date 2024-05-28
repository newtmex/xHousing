multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc::{
    api::{BlockchainApi, BlockchainApiImpl, StorageMapperApi},
    storage::StorageKey,
};
use utils::storage::StorageBuilder;

#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[derive(TopDecode, TopEncode)]
pub struct XProjectData<M: ManagedTypeApi> {
    pub id: usize,
    pub address: ManagedAddress<M>,
    pub funding_goal: BigUint<M>,
    pub funding_deadline: u64,
    pub funding_token_id: EgldOrEsdtTokenIdentifier<M>,
    pub collected_funds: BigUint<M>,
    pub status: Status,
}

pub struct XProjectStorage<SA: StorageMapperApi> {
    base_key: StorageKey<SA>,
}

impl<SA: StorageMapperApi + BlockchainApi> XProjectStorage<SA> {
    pub(super) fn create_new(
        &self,
        funding_goal: BigUint<SA>,
        funding_deadline: u64,
        funding_token_id: EgldOrEsdtTokenIdentifier<SA>,
        address: ManagedAddress<SA>,
    ) {
        self.require(funding_goal > 0, b"Funding goal must be more than 0");
        self.require(
            funding_deadline > self.get_current_time(),
            b"Deadline can't be in the past",
        );
        self.require(funding_token_id.is_valid(), b"Invalid token provided");

        let data = XProjectData {
            id: self.total().update(|total| {
                *total += 1;

                *total
            }),
            address,
            funding_goal,
            funding_deadline,
            funding_token_id,
            collected_funds: BigUint::zero(),
            status: Status::FundingPeriod,
        };

        let data_store = self.x_project(data.id);
        self.require(data_store.is_empty(), b"invalid computed project id");

        self.id_n_address().insert(data.id, data.address.clone());
        data_store.set(data);
    }

    fn get_current_time(&self) -> u64 {
        SA::blockchain_api_impl().get_block_timestamp()
    }
}

impl<SA: StorageMapperApi> StorageMapper<SA> for XProjectStorage<SA> {
    fn new(base_key: StorageKey<SA>) -> Self {
        XProjectStorage { base_key }
    }
}

impl<SA: StorageMapperApi> StorageBuilder<SA> for XProjectStorage<SA> {
    fn get_base_key(&self) -> StorageKey<SA> {
        self.base_key.clone()
    }
}

/// Storage Mappers
impl<SA: StorageMapperApi> XProjectStorage<SA> {
    fn total(&self) -> SingleValueMapper<SA, usize> {
        SingleValueMapper::new(self.build_key_by_key(b"_total"))
    }

    fn id_n_address(&self) -> MapMapper<SA, usize, ManagedAddress<SA>> {
        MapMapper::new(self.build_key_by_key(b"_id_n_address"))
    }

    fn x_project(&self, id: usize) -> SingleValueMapper<SA, XProjectData<SA>> {
        SingleValueMapper::new(self.build_key_by_id(&id))
    }
}
