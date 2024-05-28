#![no_std]

mod data;
pub mod x_project_funding_proxy;

use data::XProjectStorage;
#[allow(unused_imports)]
use multiversx_sc::imports::*;
use x_project::ProxyTrait as _;

/// The `xProjectFunding` contract is designed to manage the crowdfunding process for real estate projects
/// within the xHousing ecosystem. This contract facilitates the collection of funds from participants, handles
/// participant registrations, deploys the `xProject` contract upon successful funding, and disburses tokens (XHT and SFT)
/// to contributors. If the funding goal is not met by the deadline, it allows participants to withdraw their funds.
///
/// ## Key Features
/// - **Receive Funds:** Accept contributions from participants and track the total collected funds.
/// - **Participant Registration:** Register new participants in the xHousing contract if they are not already registered.
/// - **Funding Status Check:** Provide real-time updates on the funding status.
/// - **xProject Deployment:** Deploy the `xProject` contract when the funding goal is reached and disburse tokens.
/// - **Refund Participants:** Allow participants to withdraw their funds if the funding goal is not met by the deadline.
#[multiversx_sc::contract]
pub trait XProjectFunding: coinbase_module::CoinbaseModule + xht::XHTModule {
    #[init]
    fn init(&self, coinbase: ManagedAddress) {
        self.coinbase_addr().set_if_empty(coinbase);
    }

    #[payable("*")]
    #[endpoint]
    fn init_first_x_project(
        &self,
        xproject_template: ManagedAddress,
        xhousing_address: ManagedAddress,
        funding_token_id: EgldOrEsdtTokenIdentifier,
        funding_goal: BigUint,
        funding_deadline: u64,
    ) {
        self.require_caller_is_coinbase();

        let xht_payment = self.call_value().single_esdt();

        let mut xht_mapper = self.xht();
        require!(xht_mapper.is_empty(), "XHT set already");
        xht_mapper.set_token_id(xht_payment.token_identifier);

        self.xproject_template().set(&xproject_template);
        self.xhousing_address().set(&xhousing_address);

        self.deploy_x_project(funding_token_id, funding_goal, funding_deadline);
    }

    #[only_owner]
    #[endpoint(deployXProject)]
    fn deploy_x_project(
        &self,
        funding_token_id: EgldOrEsdtTokenIdentifier,
        funding_goal: BigUint,
        funding_deadline: u64,
    ) {
        let x_project_template_addr = self.xproject_template().get();
        require!(
            self.blockchain()
                .is_smart_contract(&x_project_template_addr),
            "invalid x_project template address"
        );

        let (address, ()) = self
            .x_project_proxy()
            .init()
            .deploy_from_source(&x_project_template_addr, CodeMetadata::UPGRADEABLE);

        self.x_project()
            .create_new(funding_goal, funding_deadline, funding_token_id, address);
    }

    #[proxy]
    fn x_project_proxy(&self) -> x_project::Proxy<Self::Api>;

    #[upgrade]
    fn upgrade(&self) {}

    #[storage_mapper("xproject_template")]
    fn xproject_template(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("xhousing_address")]
    fn xhousing_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("xproject")]
    fn x_project(&self) -> XProjectStorage<Self::Api>;
}
