#![no_std]
pub mod economics;

multiversx_sc::imports!();

pub type XHT<M> = BigUint<M>;

impl<M: ManagedTypeApi> XHTTrait<M> for XHT<M> {}

pub trait XHTTrait<M: ManagedTypeApi> {
    fn decimals() -> u32 {
        18
    }

    fn one() -> XHT<M> {
        XHT::from(10u32).pow(Self::decimals())
    }

    #[inline]
    fn ico_funds() -> XHT<M> {
        Self::max_supply() - Self::ecosystem_distibution_funds()
    }

    fn ecosystem_distibution_funds() -> XHT<M> {
        Self::from_parts(13_650_000, 2_248_573_618_499_339)
    }

    fn max_supply() -> XHT<M> {
        Self::from_parts(21_000_000, 0)
    }

    fn from_parts(integer: u32, mantissa: u64) -> XHT<M> {
        Self::one().mul(integer).add(mantissa)
    }
}

/// The `XHTModule` is responsible for managing the XHT token within the xHousing platform.
/// It includes functionalities for minting, burning, and querying the balance and total supply
/// of XHT tokens. This module ensures secure and efficient handling of token operations,
/// supporting the overall token economy of the platform. By providing core token functionalities,
/// the `XHTModule` enables the distribution of tokens for various platform activities and
/// supports deflationary mechanisms through token burning.
#[multiversx_sc::module]
pub trait XHTModule {
    fn make_xht_payment(&self, xht: XHT<Self::Api>) -> EsdtTokenPayment {
        EsdtTokenPayment::new(self.xht().get_token_id(), 0, xht)
    }

    #[view(getXhtID)]
    #[storage_mapper("xht-module::xht")]
    fn xht(&self) -> FungibleTokenMapper;
}
