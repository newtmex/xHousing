multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    ManagedVecItem,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    PartialEq,
    Debug,
)]
pub struct XPTokenAttributes<M: ManagedTypeApi> {
    pub reward_per_share: BigUint<M>,
    pub token_weight: BigUint<M>,
}

impl<M: ManagedTypeApi> XPTokenAttributes<M> {
    pub fn new(token_weight: BigUint<M>) -> Self {
        Self {
            reward_per_share: 0u64.into(),
            token_weight,
        }
    }
}
