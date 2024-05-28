#![no_std]

pub mod x_project_proxy;

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// # xProject Contract Template
///
/// The `xProject` contract template serves as the foundational blueprint for deploying 
/// individual real estate projects within the xHousing ecosystem.
/// Each `xProject` contract represents a unique real estate development, 
/// managing its ownership, revenue distribution, and participant interactions.
#[multiversx_sc::contract]
pub trait XProject {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
