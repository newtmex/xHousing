multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc::{
    api::{BlockchainApi, BlockchainApiImpl, StorageMapperApi},
    storage::StorageKey,
};
use utils::{storage::StorageBuilder, types::Epoch};
use xht::{
    economics::{emission::EmissionTrait, EntitiesValue},
    XHT,
};
use xst::XstAttributes;

const DIVISION_SAFTETY_CONST: u64 = 1_000_000_000_000_000_000;

#[derive(TopDecode, TopEncode)]
pub struct XProjectDistributionData<M: ManagedTypeApi> {
    pub max_shares: BigUint<M>,
    pub received_rents: BigUint<M>,
}

#[derive(TopDecode, TopEncode)]
pub struct ProjectsStakingRewards<M: ManagedTypeApi> {
    pub to_share: BigUint<M>,
    pub checkpoint: BigUint<M>,
}

impl<M: ManagedTypeApi> Default for ProjectsStakingRewards<M> {
    fn default() -> Self {
        Self {
            to_share: Default::default(),
            checkpoint: Default::default(),
        }
    }
}

impl<M: ManagedTypeApi> AddAssign<&BigUint<M>> for ProjectsStakingRewards<M> {
    fn add_assign(&mut self, rhs: &BigUint<M>) {
        self.to_share += rhs;
        self.checkpoint += rhs;
    }
}

impl<M: ManagedTypeApi> SubAssign<&BigUint<M>> for ProjectsStakingRewards<M> {
    fn sub_assign(&mut self, rhs: &BigUint<M>) {
        self.to_share -= rhs;
    }
}

pub struct DistributionStorage<SA: StorageMapperApi> {
    base_key: StorageKey<SA>,
}

impl<SA: StorageMapperApi + BlockchainApi> DistributionStorage<SA> {
    pub fn add_entity_funds(&self, value: EntitiesValue<SA>) {
        let store = self.entity_funds();
        if store.is_empty() {
            store.set(value)
        } else {
            store.update(|old_value| *old_value += value);
        }
    }

    pub fn generate_rewards(&self) {
        let last_dispatch_epoch_mapper = self.last_funds_dispatch_epoch();

        let current_epoch = self.get_current_distribution_epoch();
        let last_dispatch_epoch = last_dispatch_epoch_mapper.get();

        if current_epoch <= last_dispatch_epoch {
            return;
        }

        let to_dispatch = XHT::emission_through_epoch_range(last_dispatch_epoch, current_epoch);
        let mut entity_values = EntitiesValue::from_total_value(to_dispatch);

        let staking_rewards = core::mem::take(&mut entity_values.staking);
        self.add_entity_funds(entity_values);

        // 70% goes to XHT stakers
        let xht_staking_rewards = &staking_rewards * 7u64 / 10u64;

        let total_xht_weight = self.xht_total_stake_weight().get();
        if total_xht_weight > 0u64 {
            let rps_increase = &xht_staking_rewards * DIVISION_SAFTETY_CONST / total_xht_weight;
            self.xht_reward_per_share().update(|v| *v += rps_increase)
        }

        self.xht_staking_rewards()
            .update(|v| *v += &xht_staking_rewards);
        self.projects_staking_rewards()
            .update(|v| *v += &(staking_rewards - xht_staking_rewards));

        last_dispatch_epoch_mapper.set(current_epoch);
    }

    pub(super) fn set_total_funds(&self, amount: BigUint<SA>) {
        self.require(self.total_funds().is_empty(), b"total funds already set");

        self.total_funds().set(amount);
        self.genesis_epoch()
            .set(SA::blockchain_api_impl().get_block_epoch());
    }

    pub fn get_projects_reward_check_point(&self) -> BigUint<SA> {
        self.get_projects_staking_rewards().checkpoint
    }

    pub fn get_xht_reward_per_share(&self) -> BigUint<SA> {
        self.xht_reward_per_share().get()
    }

    pub(crate) fn enter_staking(&self, attr: &XstAttributes<SA>) {
        self.xht_total_stake_weight()
            .update(|v| *v += &attr.stake_weight)
    }

    fn get_projects_staking_rewards(&self) -> ProjectsStakingRewards<SA> {
        let mapper = self.projects_staking_rewards();
        mapper.set_if_empty(ProjectsStakingRewards::<SA>::default());

        mapper.get()
    }

    pub(crate) fn claim_rewards(&self, attr: &mut XstAttributes<SA>) -> BigUint<SA> {
        let mut xht_claimed = BigUint::zero();

        // Claim XPT rewards
        let xpt_reward_checkpoint = self.get_projects_reward_check_point();
        if xpt_reward_checkpoint > 0 {
            for xpt in &attr.x_project_tokens {
                xht_claimed += self.compute_reward_for_xpt(
                    xpt,
                    &attr.x_projects_share_checkpoint,
                    &xpt_reward_checkpoint,
                );
            }

            self.projects_staking_rewards().update(|rewards| {
                if rewards.to_share < xht_claimed {
                    xht_claimed = rewards.to_share.clone();
                }

                *rewards -= &xht_claimed;
            });
        }
        // Claim XHT rewards
        let xht_rps = self.xht_reward_per_share().get();
        if xht_rps > 0 && attr.xht_reward_per_share < xht_rps {
            let xht_reward = (&xht_rps - &attr.xht_reward_per_share) * &attr.stake_weight
                / DIVISION_SAFTETY_CONST;
            self.xht_staking_rewards().update(|rewards| {
                if *rewards < xht_reward {
                    xht_claimed = rewards.clone();
                }

                *rewards -= &xht_reward;
            });

            xht_claimed += &xht_reward;
        }

        // Update claim parameters
        attr.xht_reward_per_share = xht_rps;
        attr.x_projects_share_checkpoint = xpt_reward_checkpoint;

        xht_claimed
    }

    pub(super) fn add_project_rent(
        &self,
        project_token_id: &TokenIdentifier<SA>,
        amount: BigUint<SA>,
        max_shares: BigUint<SA>,
    ) {
        self.projects_total_received_rents()
            .update(|v| *v += &amount);

        self.project_dets()
            .entry(project_token_id.clone().into_managed_buffer())
            .and_modify(|dets| {
                dets.received_rents += &amount;
            })
            .or_insert(XProjectDistributionData {
                max_shares,
                received_rents: amount,
            });
    }

    fn compute_reward_for_xpt(
        &self,
        xpt: &EsdtTokenPayment<SA>,
        xst_share_checkpoint: &BigUint<SA>,
        xpt_reward_checkpoint: &BigUint<SA>,
    ) -> BigUint<SA> {
        if xpt_reward_checkpoint <= xst_share_checkpoint {
            return 0u64.into();
        }

        let global_total_rents = self.projects_total_received_rents().get();

        let project_key = xpt.token_identifier.as_managed_buffer();
        let project_dets = self
            .project_dets()
            .get(project_key)
            .unwrap_or_else(|| self.panic(b"unaccepted project token"));
        self.require(
            xpt.amount <= project_dets.max_shares,
            b"XProject token amount too large",
        );

        let rewards_since_check_point = xpt_reward_checkpoint - xst_share_checkpoint;
        let project_allocation =
            rewards_since_check_point * project_dets.received_rents / global_total_rents;

        // Computed reward
        project_allocation * &xpt.amount / project_dets.max_shares
    }

    fn get_current_distribution_epoch(&self) -> u64 {
        SA::blockchain_api_impl().get_block_epoch() - self.genesis_epoch().get()
    }
}

// Mappers
impl<SA: StorageMapperApi> DistributionStorage<SA> {
    fn entity_funds(&self) -> SingleValueMapper<SA, EntitiesValue<SA>> {
        SingleValueMapper::new(self.build_key_by_key(b"_entity_funds"))
    }

    fn total_funds(&self) -> SingleValueMapper<SA, BigUint<SA>> {
        SingleValueMapper::new(self.build_key_by_key(b"total_funds"))
    }

    fn genesis_epoch(&self) -> SingleValueMapper<SA, u64> {
        SingleValueMapper::new(self.build_key_by_key(b"gen_epoch"))
    }

    fn projects_staking_rewards(&self) -> SingleValueMapper<SA, ProjectsStakingRewards<SA>> {
        SingleValueMapper::new(self.build_key_by_key(b"projects_staking_rewards"))
    }

    fn project_dets(&self) -> MapMapper<SA, ManagedBuffer<SA>, XProjectDistributionData<SA>> {
        MapMapper::new(self.build_key_by_key(b"project_dets"))
    }

    fn projects_total_received_rents(&self) -> SingleValueMapper<SA, BigUint<SA>> {
        SingleValueMapper::new(self.build_key_by_key(b"projects_total_received_rents"))
    }

    fn xht_staking_rewards(&self) -> SingleValueMapper<SA, BigUint<SA>> {
        SingleValueMapper::new(self.build_key_by_key(b"xht_staking_rewards"))
    }

    fn xht_reward_per_share(&self) -> SingleValueMapper<SA, BigUint<SA>> {
        SingleValueMapper::new(self.build_key_by_key(b"xht_reward_per_share"))
    }

    fn xht_total_stake_weight(&self) -> SingleValueMapper<SA, BigUint<SA>> {
        SingleValueMapper::new(self.build_key_by_key(b"xht_total_stake_weight"))
    }

    fn last_funds_dispatch_epoch(&self) -> SingleValueMapper<SA, Epoch> {
        SingleValueMapper::new(self.build_key_by_key(b"last_funds_dispatch_epoch"))
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
