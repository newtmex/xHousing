multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc::{
    api::{BlockchainApi, BlockchainApiImpl, StorageMapperApi},
    storage::StorageKey,
};
use utils::storage::StorageBuilder;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
}

impl<M: BlockchainApi> XProjectData<M> {
    pub fn status(&self) -> Status {
        if M::blockchain_api_impl().get_block_timestamp() < self.funding_deadline {
            Status::FundingPeriod
        } else if self.collected_funds >= self.funding_goal {
            Status::Successful
        } else {
            Status::Failed
        }
    }
}

pub struct XProjectStorage<SA: StorageMapperApi> {
    base_key: StorageKey<SA>,
}

impl<SA: StorageMapperApi + BlockchainApi> XProjectStorage<SA> {
    pub(crate) fn create_new(
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
        };

        let data_store = self.x_project(data.id);
        self.require(data_store.is_empty(), b"invalid computed project id");

        self.id_n_address().insert(data.id, data.address.clone());
        data_store.set(data);
    }

    pub(crate) fn fund(
        &self,
        project_id: usize,
        depositor: ManagedAddress<SA>,
        payment: EgldOrEsdtTokenPayment<SA>,
    ) {
        self.require(payment.amount > BigUint::zero(), b"Invalid funding amount");

        let project_store = self.get_project_store(project_id);

        project_store.update(|data| {
            self.require(
                data.status() == Status::FundingPeriod,
                b"cannot fund project after deadline",
            );
            self.require(
                payment.token_identifier == data.funding_token_id,
                b"wrong token payment",
            );

            data.collected_funds += &payment.amount;

            self.user_x_project_deposit(data.id)
                .entry(depositor)
                .and_modify(|(_, deposit)| *deposit += &payment.amount)
                .or_insert((false, payment.amount));
        });
    }

    pub(super) fn take_deposit(
        &self,
        project_id: usize,
        depositor: ManagedAddress<SA>,
    ) -> (XProjectData<SA>, BigUint<SA>) {
        let project = self.get_project(project_id);
        self.require(
            project.status() == Status::Successful,
            b"Project Funding not yet successful",
        );

        let mut deposit_mapper = self.user_x_project_deposit(project_id);
        self.require(
            deposit_mapper.contains_key(&depositor),
            b"User did not participate in project funding",
        );

        let mut deposit = BigUint::zero();
        deposit_mapper
            .entry(depositor)
            .and_modify(|(taken, amount)| {
                self.require(!*taken, b"User project deposit amount already used");

                *taken = true;
                deposit = amount.clone();
            });

        self.require(
            deposit > BigUint::zero(),
            b"Invalid user project deposited amount",
        );

        (project, deposit)
    }

    fn get_current_time(&self) -> u64 {
        SA::blockchain_api_impl().get_block_timestamp()
    }

    pub(crate) fn get_project(&self, project_id: usize) -> XProjectData<SA> {
        self.get_project_store(project_id).get()
    }

    fn get_project_store(&self, project_id: usize) -> SingleValueMapper<SA, XProjectData<SA>> {
        let project_store = self.x_project(project_id);
        self.require(!project_store.is_empty(), b"Invalid Project ID");

        project_store
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

    fn id_n_address(&self) -> BiDiMapper<SA, usize, ManagedAddress<SA>> {
        BiDiMapper::new(self.build_key_by_key(b"_project_id_n_address"))
    }

    fn x_project(&self, id: usize) -> SingleValueMapper<SA, XProjectData<SA>> {
        SingleValueMapper::new(self.build_key_by_id(&id))
    }

    fn user_x_project_deposit(
        &self,
        project_id: usize,
    ) -> MapMapper<SA, ManagedAddress<SA>, (bool, BigUint<SA>)> {
        MapMapper::new(self.build_key(b"_deposits", &project_id))
    }
}
