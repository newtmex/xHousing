use multiversx_sc::{
    api::ManagedTypeApi,
    codec::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::EsdtTokenPayment,
};

pub type Epoch = u64;
pub type Block = u64;

#[derive(Clone)]
pub struct PaymentAttributesPair<
    M: ManagedTypeApi,
    T: Clone + TopEncode + TopDecode + NestedEncode + NestedDecode,
> {
    pub payment: EsdtTokenPayment<M>,
    pub attributes: T,
}
