#![allow(clippy::inconsistent_digit_grouping)]

use crate::XHT;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod emission;

/// 100%
pub(crate) const UNITY: u32 = 100_00;

//### Token Distribution ::It is Your Responsibility To make sure the values here add up to unity
const ENTITIES: [u32; 6] = [23_05, 30_05, 15_35, 16_55, 8_00, 7_00];
// pub(crate) const ICO_RATIO: u32 = ENTITIES[0];
const TEAM_AND_ADVISORS_RATIO: u32 = ENTITIES[0];
const PROTOCOL_DEVELOPMENT_RATIO: u32 = ENTITIES[1];
const GROWTH_RATIO: u32 = ENTITIES[2];
const STAKING_RATIO: u32 = ENTITIES[3];
const PROJECTS_RESERVE_RATIO: u32 = ENTITIES[4];
const LP_AND_LISTINGS_RATIO: u32 = ENTITIES[5];
//##end Token Distribution ::It is Your Responsibility To make sure the values here add up to unity

#[derive(Debug, TopDecode, TopEncode)]
pub struct EntitiesValue<M: ManagedTypeApi> {
    pub team: XHT<M>,
    pub protocol: XHT<M>,
    pub growth: XHT<M>,
    pub staking: XHT<M>,
    pub projects_reserve: XHT<M>,
    pub lp_n_listing: XHT<M>,
}

impl<M: ManagedTypeApi> Default for EntitiesValue<M> {
    fn default() -> Self {
        Self {
            team: XHT::zero(),
            growth: XHT::zero(),
            protocol: XHT::zero(),
            staking: XHT::zero(),
            projects_reserve: XHT::zero(),
            lp_n_listing: XHT::zero(),
        }
    }
}

impl<M: ManagedTypeApi> EntitiesValue<M> {
    pub fn from_total_value(total: XHT<M>) -> EntitiesValue<M> {
        let others_total = &total * (UNITY - PROTOCOL_DEVELOPMENT_RATIO) / UNITY;
        let compute = |ratio: u32| &others_total * ratio / UNITY;

        let team = compute(TEAM_AND_ADVISORS_RATIO);
        let growth = compute(GROWTH_RATIO);
        let staking = compute(STAKING_RATIO);
        let projects_reserve = compute(PROJECTS_RESERVE_RATIO);
        let lp_n_listing = compute(LP_AND_LISTINGS_RATIO);

        // Protocol takes the execesses
        let protocol = &total - &team - &growth - &staking - &projects_reserve - &lp_n_listing;

        EntitiesValue {
            team,
            growth,
            protocol,
            staking,
            projects_reserve,
            lp_n_listing,
        }
    }

    pub fn total(&self) -> XHT<M> {
        &self.team
            + &self.growth
            + &self.protocol
            + &self.staking
            + &self.projects_reserve
            + &self.lp_n_listing
    }
}

impl<M: ManagedTypeApi> AddAssign for EntitiesValue<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.team += rhs.team;
        self.growth += rhs.growth;
        self.protocol += rhs.protocol;
        self.staking += rhs.staking;
        self.projects_reserve += rhs.projects_reserve;
        self.lp_n_listing += rhs.lp_n_listing;
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use core::assert_eq;

    use multiversx_sc_scenario::api::SingleTxApi;

    use super::*;

    #[test]

    fn check_entity_ratios() {
        let mut total_ratio = 0;

        for ratio in ENTITIES {
            total_ratio += ratio;
        }

        assert_eq!(total_ratio, UNITY);
    }

    #[test]
    fn entities_value() {
        for index in 0..=100 {
            let total = index as u64 * 1_235u64.pow(index / 23) + 1;

            let value = EntitiesValue::<SingleTxApi>::from_total_value(total.into());
            assert_eq!(value.total(), total);

            let EntitiesValue {
                team,
                growth,
                lp_n_listing,
                projects_reserve,
                protocol,
                staking,
            } = value;
            assert_eq!(
                staking + team + growth + protocol + lp_n_listing + projects_reserve,
                total
            );
        }
    }
}
