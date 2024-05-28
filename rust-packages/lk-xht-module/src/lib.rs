#![no_std]

pub use multiversx_sc_modules::default_issue_callbacks;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// 3 years
const LOCK_DURATION: u64 = 3 * 366 * 24 * 60 * 60;

#[derive(TopEncode)]
pub struct LkXhtAttributes {
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

impl LkXhtAttributes {
    pub fn new(start_timestamp: u64) -> Self {
        Self {
            start_timestamp,
            end_timestamp: start_timestamp + LOCK_DURATION,
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
            &LkXhtAttributes::new(self.lk_xht_start_timestamp().get()),
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
