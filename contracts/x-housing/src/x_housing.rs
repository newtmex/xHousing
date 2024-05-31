#![no_std]

pub mod distribution;
pub mod permission;
pub mod staking;
pub mod users;

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use permission::Permissions;
use xst::{default_issue_callbacks, lk_xht};

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
    + xht::XHTModule
    + xst::XstModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
    + distribution::DistributionModule
    + coinbase_module::CoinbaseModule
    + staking::StakeModule
    + permission::PermissionsModule
    + lk_xht::LkXhtModule
{
    #[init]
    fn init(&self, coinbase: ManagedAddress, x_project_funding: ManagedAddress) {
        self.require_is_sc_addr(&coinbase);
        self.require_is_sc_addr(&x_project_funding);

        self.coinbase_addr().set_if_empty(coinbase);
        self.x_project_funding_addr()
            .set_if_empty(x_project_funding);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(createRefID)]
    /// Creates a new user and returns ID or just returns their ref ID if they already are members
    ///
    /// Anyone can call this endpoint at anytime (during project funding or not) to register their wallet address as users of the xHousing platform
    /// so they can get a referral ID that they can use to leverage other earning opportunities on the platform
    fn create_ref_id(&self, referrer_id: OptionalValue<usize>) -> usize {
        let user_addr = self.get_caller_as_user_address();

        self.create_or_get_user_id(&user_addr, referrer_id)
    }

    #[endpoint]
    fn create_ref_id_via_proxy(
        &self,
        user_addr: ManagedAddress,
        referrer_id: OptionalValue<usize>,
    ) -> usize {
        self.require_caller_is_x_project_funding();
        require!(
            !self.blockchain().is_smart_contract(&user_addr),
            "user address is smart contract"
        );

        self.create_or_get_user_id(&user_addr, referrer_id)
    }

    #[endpoint]
    fn add_x_project(&self, address: ManagedAddress) {
        self.require_caller_is_x_project_funding();

        self.set_permissions(address, Permissions::X_PROJECT);
    }

    #[endpoint]
    fn set_lk_xht_id(&self, token_id: TokenIdentifier<Self::Api>) {
        self.require_caller_is_x_project_funding();

        self.lk_xht().set_if_empty(token_id);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerXst)]
    fn register_xst_token(&self) {
        let issue_amount = self.call_value().egld_value();

        self.xst().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            issue_amount.clone_value(),
            b"XStakingToken".into(),
            b"XST".into(),
            0,
            None,
        );
    }

    fn require_caller_is_x_project_funding(&self) {
        let caller = self.blockchain().get_caller();
        let x_project_funding_addr = self.x_project_funding_addr().get();

        require!(caller == x_project_funding_addr, "not allowed");
    }

    #[storage_mapper("x-project-funding-addr")]
    fn x_project_funding_addr(&self) -> SingleValueMapper<ManagedAddress>;
}
