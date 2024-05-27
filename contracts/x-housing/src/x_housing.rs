#![no_std]

pub mod coinbase_interaction;
pub mod distribution;
pub mod users;
pub mod x_housing_proxy;

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// xHousing leverages blockchain technology to revolutionise real estate investment and development by enabling the tokenisation of properties,
/// allowing for fractional ownership and ease of investment.This innovative approach addresses the high costs and limited access to real estate
/// investments in Abuja, Nigeria, making the market more inclusive and accessible. By selling tokens, xHousing provides developers with
/// immediate access to liquid funds, ensuring the timely and quality completion of affordable development projects.
///
/// The XHousing Contract is the main contract for the xHousing ecosystem.
///
/// This contract owns and deploys xProject contracts which will represent the properties owned and managed by the xHousing project.
/// The management of ecosystem users will also be done in this contract.
#[multiversx_sc::contract]
pub trait XHousing:
    users::UsersModule
    + utils::UtilsModule
    + coinbase_interaction::CoinbaseInteraction
    + xht::XHTModule
    + distribution::DistributionModule
{
    #[init]
    fn init(&self, coinbase: ManagedAddress) {
        self.coinbase_addr().set_if_empty(coinbase);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(createRefID)]
    /// Creates a new user and returns ID or just returns their ref ID if they already are members
    ///
    /// Anyone can call this endpoint to register their wallet address as users of the xHousing platform
    /// so they can get a referral ID that they can use to leverage other earning opportunities on the platform
    fn create_ref_id(&self, referrer_id: OptionalValue<usize>) -> usize {
        let user_addr = self.get_caller_as_user_address();

        self.create_or_get_user_id(&user_addr, referrer_id)
    }
}
