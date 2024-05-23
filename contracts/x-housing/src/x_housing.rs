#![no_std]

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
pub trait XHousing {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
