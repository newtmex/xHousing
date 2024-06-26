#![no_std]

use multiversx_sc::api::{BlockchainApi, BlockchainApiImpl};
pub use multiversx_sc_modules::default_issue_callbacks;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// 3 years
pub const LOCK_DURATION: u64 = 3 * 366 * 24 * 60 * 60;

#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, Clone, Debug)]
pub struct LkXhtAttributes<M: ManagedTypeApi> {
    pub initial_xht_amount: BigUint<M>,
    pub xht_amount: BigUint<M>,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

impl<M: ManagedTypeApi + BlockchainApi> LkXhtAttributes<M> {
    pub fn new(start_timestamp: u64, xht_amount: BigUint<M>) -> Self {
        Self {
            start_timestamp,
            end_timestamp: start_timestamp + LOCK_DURATION,
            initial_xht_amount: xht_amount.clone(),
            xht_amount,
        }
    }

    pub fn unlock_matured(mut self) -> (BigUint<M>, Self) {
        let unlocked_xht = &self.xht_amount * self.elapsed_time() / LOCK_DURATION;
        self.xht_amount -= &unlocked_xht;

        (unlocked_xht, self)
    }

    fn elapsed_time(&self) -> u64 {
        let timestamp = M::blockchain_api_impl().get_block_timestamp();
        if timestamp >= self.end_timestamp {
            LOCK_DURATION
        } else {
            timestamp - self.start_timestamp
        }
    }
}

#[multiversx_sc::module]
pub trait LkXhtModule: default_issue_callbacks::DefaultIssueCallbacksModule {
    fn mint_lk_xht_token(&self, amount: BigUint, receiver: &ManagedAddress) {
        let token_id = self.lk_xht().get_token_id();
        let token_nonce = self.send().esdt_nft_create_compact(
            &token_id,
            &amount,
            &LkXhtAttributes::new(self.lk_xht_start_timestamp().get(), amount.clone()),
        );

        self.tx()
            .to(receiver)
            .esdt((token_id, token_nonce, amount))
            .transfer();
    }

    #[view(getLkXhtID)]
    #[storage_mapper("lk-xht-module::lk-xht")]
    fn lk_xht(&self) -> NonFungibleTokenMapper;

    #[storage_mapper("lk-xht-module::start-timestamp")]
    fn lk_xht_start_timestamp(&self) -> SingleValueMapper<u64>;
}

#[cfg(test)]
mod tests {
    use multiversx_sc_scenario::api::SingleTxApi;

    use crate::{LkXhtAttributes, LOCK_DURATION};

    #[test]
    fn test_unlock_matured() {
        let xht_amount = 250_000u64;
        let attr = LkXhtAttributes::<SingleTxApi>::new(0, xht_amount.into());

        let check_matured =
            |time_percent: u64, expected_matured: u64, attr: LkXhtAttributes<SingleTxApi>| {
                SingleTxApi::with_global(|data| {
                    data.current_block_info.block_timestamp = LOCK_DURATION * time_percent / 100;
                });
                let (matured, attr) = attr.unlock_matured();

                assert_eq!(
                    matured.to_u64().unwrap(),
                    expected_matured,
                    "None sohuld be matured"
                );

                attr
            };

        let attr = check_matured(0, 0, attr);
        let attr = check_matured(10, 25_000, attr);
        let attr = check_matured(30, 67_500, attr);
        let attr = check_matured(50, 78_750, attr);
        let attr = check_matured(120, 78_750, attr);
        let _attr = check_matured(130, 0, attr);
    }
}
