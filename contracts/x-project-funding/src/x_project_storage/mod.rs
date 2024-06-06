multiversx_sc::imports!();

use data::{Status, XProjectData, XProjectStorage};

pub mod data;
use lk_xht_module::default_issue_callbacks;
use utils::contracts_proxy::x_project_proxy;
use xht::{XHTTrait, XHT};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait XProjectInteraction:
    xht::XHTModule
    + lk_xht_module::LkXhtModule
    + default_issue_callbacks::DefaultIssueCallbacksModule
    + x_housing_module::XHousingModule
{
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerLkXht)]
    fn register_lk_xht_token(&self) {
        // TODO: is this neccessary?
        // let genesis_project = self.x_project_storage().get_project(1);
        // require!(
        //     genesis_project.status() == Status::Successful,
        //     "First XProject Funding not yet successful"
        // );

        let payment_amount = self.call_value().egld_value();

        let timestamp = self.blockchain().get_block_timestamp();
        self.lk_xht_start_timestamp().set(timestamp);

        self.lk_xht().issue_and_set_all_roles(
            EsdtTokenType::Meta,
            payment_amount.clone_value(),
            b"LockedXHT".into(),
            b"LKXHT".into(),
            18,
            None,
        );
    }

    #[only_owner]
    #[endpoint]
    #[payable("EGLD")]
    fn set_x_project_token(&self, project_id: usize, name: ManagedBuffer) {
        let project = self.x_project_storage().get_project(project_id);
        // TODO add this after demo, since we can't control blockchain state on public networks
        // require!(
        //     project.status() == Status::Successful,
        //     "XProject Funding not yet successful"
        // );

        if project.id == 1 {
            require!(!self.lk_xht().is_empty(), "Locked XHT not set yet");
        }

        self.call_x_housing()
            .add_x_project(&project.address)
            .gas(5_000_000)
            .sync_call();

        let _: IgnoreValue = self
            .call_x_project(project.address)
            .register_xp_token(name, project.collected_funds, self.xht().get_token_id())
            .egld(self.call_value().egld_value())
            .gas(60_000_000)
            .execute_on_dest_context();
    }

    #[endpoint(claimXProjectToken)]
    fn claim_x_project_tokens(&self, project_id: usize) {
        let depositor = self.blockchain().get_caller();

        let (project, deposit_amount) = self
            .x_project_storage()
            .take_deposit(project_id, depositor.clone());

        let back_transfers = self
            .call_x_project(project.address)
            .mint_xp_token(&deposit_amount, &depositor, &project.collected_funds)
            .returns(ReturnsBackTransfers)
            .sync_call();

        require!(
            back_transfers.esdt_payments.len() == 1,
            "Only one XPT payment expected"
        );

        self.tx()
            .to(&depositor)
            .esdt(back_transfers.esdt_payments.get(0))
            .transfer();

        if project.id == 1 {
            // Send XHT tokens earned from ICO
            let xht_amount = deposit_amount * XHT::ico_funds() / project.collected_funds;
            self.mint_lk_xht_token(xht_amount, &depositor);
        }
    }

    fn call_x_project(
        &self,
        addr: ManagedAddress,
    ) -> x_project_proxy::XProjectProxyMethods<TxScEnv<Self::Api>, (), ManagedAddress<Self::Api>, ()>
    {
        self.tx().to(addr).typed(x_project_proxy::XProjectProxy)
    }

    #[view(getXProjectTokenID)]
    fn x_project_token_id(&self, project_id: usize) -> TokenIdentifier<Self::Api> {
        let project_addr = self.x_project_storage().get_project(project_id).address;

        self.call_x_project(project_addr)
            .xp_token()
            .execute_on_dest_context()
    }

    #[view(getXProjectAddress)]
    fn x_project_address(&self, project_id: usize) -> ManagedAddress {
        self.x_project_storage().get_project(project_id).address
    }

    #[view(getXProjectData)]
    fn x_project_data(&self, project_id: usize) -> XProjectData<Self::Api> {
        self.x_project_storage().get_project(project_id)
    }

    #[view(getAllXProjectData)]
    fn all_x_projects(&self) -> ManagedVec<Self::Api, XProjectData<Self::Api>> {
        self.x_project_storage().get_projects()
    }

    #[storage_mapper("xproject_template")]
    fn xproject_template(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("xproject_store")]
    fn x_project_storage(&self) -> XProjectStorage<Self::Api>;
}
