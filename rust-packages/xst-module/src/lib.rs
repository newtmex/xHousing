#![no_std]
#![feature(trait_alias)]

pub use lk_xht;

use lk_xht::LkXhtAttributes;
use multiversx_sc::api::BlockchainApi;
pub use multiversx_sc_modules::default_issue_callbacks;
use multiversx_sc_modules::transfer_role_proxy::PaymentsVec;
use utils::types::{Epoch, PaymentAttributesPair};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const MIN_EPOCHS_LOCK: Epoch = 180;
pub const MAX_EPOCHS_LOCK: Epoch = 1080;

#[derive(Clone, TopEncode, TopDecode, NestedDecode, NestedEncode)]
pub struct XstAttributes<M: ManagedTypeApi> {
    pub x_project_tokens: ArrayVec<EsdtTokenPayment<M>, 10>,
    /// Last total rewards for XPT since last reward dispatch
    pub x_projects_share_checkpoint: BigUint<M>,
    pub xht_reward_per_share: BigUint<M>,
    pub xht_amount: BigUint<M>,
    pub stake_weight: BigUint<M>,
    pub lk_duration: u64,
    pub lk_xht: Option<(u64, LkXhtAttributes<M>)>,
}

impl<M: ManagedTypeApi + BlockchainApi> XstAttributes<M> {
    fn compute_stake_weight(mut self) -> Self {
        self.lk_duration = self.lk_duration.clamp(MIN_EPOCHS_LOCK, MAX_EPOCHS_LOCK);
        self.stake_weight = (&self.xht_amount
            + &self
                .lk_xht
                .clone()
                .map_or(BigUint::zero(), |(_, attr)| attr.xht_amount))
            * self.lk_duration;

        self
    }
}

#[multiversx_sc::module]
pub trait XstModule:
    default_issue_callbacks::DefaultIssueCallbacksModule + lk_xht::LkXhtModule
{
    fn mint_xst_token(
        &self,
        payments: PaymentsVec<Self::Api>,
        x_projects_share_checkpoint: BigUint,
        xht_reward_per_share: BigUint,
        lk_duration: u64,
        xht_id: TokenIdentifier<Self::Api>,
    ) -> PaymentAttributesPair<Self::Api, XstAttributes<Self::Api>> {
        let mut x_project_tokens = ArrayVec::new();
        let mut xht_amount = BigUint::zero();
        let mut lk_xht: Option<(u64, LkXhtAttributes<Self::Api>)> = None;

        let max_xpt_capacity = x_project_tokens.capacity();
        for payment in &payments {
            if payment.token_identifier == xht_id {
                xht_amount = payment.amount
            } else if payment.token_identifier == self.lk_xht().get_token_id() {
                let attr: LkXhtAttributes<Self::Api> =
                    self.lk_xht().get_token_attributes(payment.token_nonce);
                require!(
                    payment.amount == attr.initial_xht_amount,
                    "Must send all amount: {}",
                    (attr.initial_xht_amount)
                );

                lk_xht = Some((payment.token_nonce, attr));
            } else {
                require!(
                    self.projects_token()
                        .contains_id(payment.token_identifier.as_managed_buffer()),
                    "Invalid xProject Token"
                );

                require!(
                    x_project_tokens.len() < max_xpt_capacity,
                    "Max XProject Tokens is {}",
                    max_xpt_capacity
                );
                x_project_tokens.push(payment);
            }
        }

        require!(
            xht_amount > 0 || (lk_xht.is_some() && lk_xht.clone().unwrap().1.xht_amount > 0),
            "Must send XHT"
        );
        require!(!x_project_tokens.is_empty(), "Must send XProject tokens");

        let attributes = XstAttributes::<Self::Api> {
            x_project_tokens,
            x_projects_share_checkpoint,
            xht_reward_per_share,
            xht_amount,
            stake_weight: BigUint::zero(),
            lk_xht,
            lk_duration,
        }
        .compute_stake_weight();
        let payment = self.xst().nft_create(1u64.into(), &attributes);

        PaymentAttributesPair {
            payment,
            attributes,
        }
    }

    #[view(getXstID)]
    #[storage_mapper("xst-module::xst")]
    fn xst(&self) -> NonFungibleTokenMapper;

    #[storage_mapper("xst-module::projects-token")]
    fn projects_token(&self) -> BiDiMapper<Self::Api, ManagedBuffer, ManagedAddress>;
}
