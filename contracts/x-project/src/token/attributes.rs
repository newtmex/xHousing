multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(
    ManagedVecItem,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    Clone,
    PartialEq,
    Debug,
)]
pub struct XPTokenAttributes<M: ManagedTypeApi> {
    pub reward_per_share: BigUint<M>,
    pub token_weight: BigUint<M>,
    /// Used to share referrer reward
    pub original_owner: ManagedAddress<M>,
}

impl<M: ManagedTypeApi> XPTokenAttributes<M> {
    pub fn new(token_weight: BigUint<M>, original_owner: ManagedAddress<M>) -> Self {
        Self {
            reward_per_share: 0u64.into(),
            token_weight,
            original_owner,
        }
    }
}
