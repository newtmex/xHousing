#![no_std]

pub mod x_project_funding_proxy;
pub mod x_project_storage;

pub use lk_xht_module;
use lk_xht_module::{default_issue_callbacks, LkXhtAttributes};

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use x_housing_module::x_housing::ProxyTrait as _;
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
pub trait XProjectFunding:
    coinbase_module::CoinbaseModule
    + xht::XHTModule
    + x_housing_module::XHousingModule
    + x_project_storage::XProjectInteraction
    + lk_xht_module::LkXhtModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(&self, coinbase: ManagedAddress) {
        self.set_coinbase_addr(coinbase);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint]
    fn init_first_x_project(
        &self,
        xproject_template: ManagedAddress,
        x_housing_address: ManagedAddress,
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
        self.set_x_housing_addr(x_housing_address);

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
        let x_housing_addr = self.get_x_housing_addr();
        require!(
            self.blockchain()
                .is_smart_contract(&x_project_template_addr),
            "invalid x_project template address"
        );

        let (address, ()) = self
            .x_project_proxy()
            .init(x_housing_addr)
            .deploy_from_source(&x_project_template_addr, CodeMetadata::UPGRADEABLE);

        self.x_project_storage().create_new(
            funding_goal,
            funding_deadline,
            funding_token_id,
            address,
        );
    }

    #[endpoint(fundProject)]
    #[payable("*")]
    fn fund_project(&self, project_id: usize, referrer_id: OptionalValue<usize>) {
        let deposit_payment = self.call_value().egld_or_single_esdt();
        let depositor = self.blockchain().get_caller();

        // Try register user
        let _: IgnoreValue = self
            .call_x_housing()
            .create_ref_id_via_proxy(&depositor, referrer_id)
            .execute_on_dest_context();

        self.x_project_storage()
            .fund(project_id, depositor, deposit_payment);
    }

    #[payable("*")]
    #[endpoint(unlockXht)]
    fn unlock_xht(&self) {
        let mut payments = self.call_value().all_esdt_transfers().clone_value();

        let lk_xht_mapper = self.lk_xht();
        lk_xht_mapper.require_all_same_token(&payments);

        let mut xht_amount = BigUint::zero();

        for token in &payments {
            let (unlocked, new_attr) = lk_xht_mapper
                .get_token_attributes::<LkXhtAttributes<Self::Api>>(token.token_nonce)
                .unlock_matured();

            xht_amount += unlocked;
            lk_xht_mapper.nft_update_attributes(token.token_nonce, &new_attr);
        }

        // Add XHT to payments, then send payments back to user
        if xht_amount > 0 {
            payments.push(self.make_xht_payment(xht_amount));
        }
        self.tx().to(ToCaller).multi_esdt(payments).transfer();
    }
}
