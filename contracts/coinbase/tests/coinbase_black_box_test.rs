mod blockchain;

use blockchain::BlockState;
use multiversx_sc::types::{TestAddress, TestSCAddress};
use multiversx_sc_scenario::{
    api::StaticApi, imports::*, multiversx_chain_vm::world_mock::BlockInfo, ScenarioWorld,
};
use x_housing_utils::contracts_proxy::*;
use x_project::token::attributes::XPTokenAttributes;
use x_project_funding::lk_xht_module::LkXhtAttributes;
use xht::{XHTTrait, XHT};

type Xht = xht::XHT<StaticApi>;
type OptionalAddress = OptionalValue<ManagedAddress<StaticApi>>;

const CONTRACTS_OWNER: TestAddress = TestAddress::new("contracts-owner");
const COINBASE_ADDR: TestSCAddress = TestSCAddress::new("coinbase");
const X_HOUSING_ADDR: TestSCAddress = TestSCAddress::new("x-housing");
const X_PROJECT_FUNDING_ADDR: TestSCAddress = TestSCAddress::new("x-project-funding");
const X_PROJECT_TEMPLATE_ADDR: TestSCAddress = TestSCAddress::new("x-project-template");

const COINBASE_CODE_PATH: MxscPath = MxscPath::new("output/coinbase.mxsc.json");
const X_HOUSING_CODE_PATH: MxscPath = MxscPath::new("../x-housing/output/x-housing.mxsc.json");
const X_PROJECT_FUNDING_CODE_PATH: MxscPath =
    MxscPath::new("../x-project-funding/output/x-project-funding.mxsc.json");
const X_PROJECT_TEMPLATE_CODE_PATH: MxscPath =
    MxscPath::new("../x-project/output/x-project.mxsc.json");

const XHT_ID: &str = "str:XHT-123456";
const XHT_STORE_KEY: &str = "str:xht-module::xht";
const FUNDING_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("FUND-123456");

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::new();

    world.register_contract(COINBASE_CODE_PATH, coinbase::ContractBuilder);
    world.register_contract(X_HOUSING_CODE_PATH, x_housing::ContractBuilder);
    world.register_contract(
        X_PROJECT_FUNDING_CODE_PATH,
        x_project_funding::ContractBuilder,
    );
    world.register_contract(X_PROJECT_TEMPLATE_CODE_PATH, x_project::ContractBuilder);

    world
}

struct CoinbaseTestState {
    world: ScenarioWorld,
    block_state: BlockState,
}

impl CoinbaseTestState {
    fn new() -> Self {
        let mut world = world();

        world.account(CONTRACTS_OWNER);

        Self {
            world,
            block_state: BlockState::new(0, 14_400),
        }
    }

    fn deploy_x_housing(&mut self) {
        self.world
            .tx()
            .from(CONTRACTS_OWNER)
            .typed(x_housing_proxy::XHousingProxy)
            .init(COINBASE_ADDR, X_PROJECT_FUNDING_ADDR)
            .code(X_HOUSING_CODE_PATH)
            .new_address(X_HOUSING_ADDR)
            .run();

        self.world
            .tx()
            .from(CONTRACTS_OWNER)
            .to(X_HOUSING_ADDR)
            .typed(x_housing_proxy::XHousingProxy)
            .register_xst_token()
            .run();
    }

    fn deploy_coinbase(&mut self) {
        self.world
            .tx()
            .from(CONTRACTS_OWNER)
            .typed(coinbase_proxy::CoinbaseProxy)
            .init()
            .code(COINBASE_CODE_PATH)
            .new_address(COINBASE_ADDR)
            .run();
    }

    fn deploy_x_project_template(&mut self) {
        self.world
            .tx()
            .from(CONTRACTS_OWNER)
            .typed(x_project_proxy::XProjectProxy)
            .init(X_HOUSING_ADDR)
            .code(X_PROJECT_TEMPLATE_CODE_PATH)
            .new_address(X_PROJECT_TEMPLATE_ADDR)
            .run();
    }

    fn deploy_x_project_funding(&mut self) {
        self.world
            .tx()
            .from(CONTRACTS_OWNER)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .init(COINBASE_ADDR)
            .code(X_PROJECT_FUNDING_CODE_PATH)
            .new_address(X_PROJECT_FUNDING_ADDR)
            .run();
    }

    fn register_xht(&mut self) {
        self.world
            .tx()
            .from(CONTRACTS_OWNER)
            .to(COINBASE_ADDR)
            .gas(90_000_000)
            .typed(coinbase_proxy::CoinbaseProxy)
            .register_xht()
            .run();

        let mut coinbase_state = Account::new()
            .owner(CONTRACTS_OWNER)
            .esdt_balance(XHT_ID, Xht::max_supply());
        coinbase_state
            .storage
            .insert(XHT_STORE_KEY.into(), XHT_ID.into());
        let coinbase_code = self
            .world
            .code_expression(&COINBASE_CODE_PATH.eval_to_expr());
        coinbase_state.code = Some(coinbase_code);

        self.world
            .set_state_step(SetStateStep::new().put_account(COINBASE_ADDR, coinbase_state));
    }

    fn start_ico(&mut self) {
        self.deploy_coinbase();
        self.register_xht();

        self.deploy_x_housing();
        self.deploy_x_project_template();
        self.deploy_x_project_funding();

        self.world
            .tx()
            .to(X_PROJECT_FUNDING_ADDR)
            .from(CONTRACTS_OWNER)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .register_lk_xht_token()
            .run();

        let lk_xht_id = self.get_lk_xht_id();
        self.world.set_esdt_local_roles(
            X_PROJECT_FUNDING_ADDR,
            lk_xht_id.to_boxed_bytes().as_slice(),
            &[EsdtLocalRole::NftUpdateAttributes, EsdtLocalRole::NftCreate],
        );

        self.world
            .tx()
            .to(COINBASE_ADDR)
            .from(CONTRACTS_OWNER)
            .typed(coinbase_proxy::CoinbaseProxy)
            .start_ico(
                X_PROJECT_FUNDING_ADDR,
                X_PROJECT_TEMPLATE_ADDR,
                X_HOUSING_ADDR,
                FUNDING_TOKEN_ID,
                20_000u64,
                10_000u64,
            )
            .run();

        self.world
            .tx()
            .from(CONTRACTS_OWNER)
            .to(COINBASE_ADDR)
            .typed(coinbase_proxy::CoinbaseProxy)
            .feed_x_housing(OptionalAddress::Some(X_HOUSING_ADDR.to_address().into()))
            .run();
    }

    fn fund_project(
        &mut self,
        depositor: TestAddress,
        project_id: usize,
        referrer_id: OptionalValue<usize>,
        amount: u64,
    ) {
        self.world
            .tx()
            .to(X_PROJECT_FUNDING_ADDR)
            .from(depositor)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .fund_project(project_id, referrer_id)
            .payment(EsdtTokenPayment::new(
                FUNDING_TOKEN_ID.into(),
                0,
                amount.into(),
            ))
            .run();
    }

    fn stake(
        &mut self,
        staker: TestAddress,
        epochs_lock: u64,
        referrer_id: OptionalValue<usize>,
        payments: Vec<EsdtTokenPayment<StaticApi>>,
    ) {
        self.world
            .tx()
            .to(X_HOUSING_ADDR)
            .from(staker)
            .typed(x_housing_proxy::XHousingProxy)
            .stake(epochs_lock, referrer_id)
            .multi_esdt(payments)
            .run();
    }

    fn unlock_xht(&mut self, depositor: TestAddress, token_nonce: u64, amount: BigUint<StaticApi>) {
        let lk_xht_id = self.get_lk_xht_id();

        self.world
            .tx()
            .to(X_PROJECT_FUNDING_ADDR)
            .from(depositor)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .unlock_xht()
            .payment(EsdtTokenPayment::new(lk_xht_id, token_nonce, amount))
            .run();
    }

    fn get_xht_id(&mut self) -> TokenIdentifier<StaticApi> {
        self.world
            .query()
            .to(COINBASE_ADDR)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .xht()
            .returns(ReturnsResult)
            .run()
    }

    fn get_xst_id(&mut self) -> TokenIdentifier<StaticApi> {
        self.world
            .query()
            .to(X_HOUSING_ADDR)
            .typed(x_housing_proxy::XHousingProxy)
            .xst()
            .returns(ReturnsResult)
            .run()
    }

    fn get_lk_xht_id(&mut self) -> TokenIdentifier<StaticApi> {
        self.world
            .query()
            .to(X_PROJECT_FUNDING_ADDR)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .lk_xht()
            .returns(ReturnsResult)
            .run()
    }

    fn get_project_token_id(&mut self, project_id: usize) -> TokenIdentifier<StaticApi> {
        self.world
            .query()
            .to(X_PROJECT_FUNDING_ADDR)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .x_project_token_id(project_id)
            .returns(ReturnsResult)
            .run()
    }

    fn get_project_address(&mut self, project_id: usize) -> ManagedAddress<StaticApi> {
        self.world
            .query()
            .to(X_PROJECT_FUNDING_ADDR)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .x_project_address(project_id)
            .returns(ReturnsResult)
            .run()
    }

    fn set_x_project_token(
        &mut self,
        project_id: usize,
        name: &str,
    ) -> (ManagedAddress<StaticApi>, TokenIdentifier<StaticApi>) {
        self.world
            .tx()
            .to(X_PROJECT_FUNDING_ADDR)
            .from(CONTRACTS_OWNER)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .set_x_project_token(project_id, name)
            .run();

        let project_addr = self.get_project_address(project_id);
        let project_token_id = self.get_project_token_id(project_id);

        let xht_id = self.get_xht_id();
        self.world.set_esdt_local_roles(
            &project_addr,
            xht_id.to_boxed_bytes().as_slice(),
            &[EsdtLocalRole::Burn],
        );
        self.world.set_esdt_local_roles(
            &project_addr,
            project_token_id.to_boxed_bytes().as_slice(),
            &[EsdtLocalRole::NftUpdateAttributes, EsdtLocalRole::NftCreate],
        );

        (project_addr, project_token_id)
    }

    fn claim_x_project_tokens(&mut self, depositor: TestAddress, project_id: usize) {
        self.world
            .tx()
            .to(X_PROJECT_FUNDING_ADDR)
            .from(depositor)
            .typed(x_project_funding_proxy::XProjectFundingProxy)
            .claim_x_project_tokens(project_id)
            .run();
    }

    fn move_block_rounds(&mut self, blocks: u64) {
        let _step = self.block_state.move_block_round(blocks, None);

        let BlockInfo {
            block_timestamp,
            block_nonce,
            block_round,
            block_epoch,
            ..
        } = self.block_state.info.clone();

        self.world
            .current_block()
            .block_epoch(block_epoch)
            .block_round(block_round)
            .block_timestamp(block_timestamp)
            .block_nonce(block_nonce);
    }

    fn get_block_info(&self) -> &BlockInfo {
        &self.block_state.info
    }

    fn check_xht_bal(&mut self, depositor: TestAddress, balance: &BigUint<StaticApi>) {
        let xht_id = self.get_xht_id();
        self.world
            .check_account(depositor)
            .esdt_balance(&xht_id, balance);
    }

    fn check_xpt_bal(
        &mut self,
        project_id: usize,
        depositor: TestAddress,
        token_nonce: u64,
        amount: BigUint<StaticApi>,
    ) {
        let project_token_id = self.get_project_token_id(project_id);
        self.world
            .check_account(depositor)
            .esdt_nft_balance_and_attributes(
                &project_token_id,
                token_nonce,
                &amount,
                XPTokenAttributes::<StaticApi>::new(
                    amount.clone(),
                    depositor.resolve_address(&TxScEnv::default()),
                ),
            );
    }

    fn check_lk_xht_bal(
        &mut self,
        depositor: TestAddress,
        token_nonce: u64,
        balance: &BigUint<StaticApi>,
        attr: &LkXhtAttributes<StaticApi>,
    ) {
        let lk_xht_id = self.get_lk_xht_id();
        self.world
            .check_account(depositor)
            .esdt_nft_balance_and_attributes(&lk_xht_id, token_nonce, balance, attr);
    }
}

#[test]
fn test_x_housing_genesis() {
    let mut state = CoinbaseTestState::new();

    state.deploy_coinbase();
    state.deploy_x_housing();
    state.register_xht();

    state
        .world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .typed(coinbase_proxy::CoinbaseProxy)
        .feed_x_housing(OptionalAddress::Some(X_HOUSING_ADDR.to_address().into()))
        .run();

    // Feeding again
    state
        .world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .typed(coinbase_proxy::CoinbaseProxy)
        .feed_x_housing(OptionalAddress::None)
        .returns(ExpectError(4, "Already dispatched"))
        .run();

    let xht_id = state.get_xht_id();
    state
        .world
        .check_account(X_HOUSING_ADDR)
        .esdt_balance(xht_id, Xht::ecosystem_distibution_funds());
}

#[test]
fn test_start_ico() {
    let mut state = CoinbaseTestState::new();

    state.start_ico();

    let xht_token = state.get_xht_id();

    state
        .world
        .check_account(X_PROJECT_FUNDING_ADDR)
        .esdt_balance(xht_token, Xht::ico_funds());
}

#[test]
fn test_fund_project() {
    let mut state = CoinbaseTestState::new();

    let depositor = TestAddress::new("depositor");
    state
        .world
        .account(depositor)
        .esdt_balance(FUNDING_TOKEN_ID, 50_000);

    state.start_ico();

    state.fund_project(depositor, 1, OptionalValue::None, 50);

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(15_000));

    // Late funding
    state
        .world
        .tx()
        .to(X_PROJECT_FUNDING_ADDR)
        .from(depositor)
        .typed(x_project_funding_proxy::XProjectFundingProxy)
        .fund_project(1usize, OptionalValue::<usize>::None)
        .payment(EsdtTokenPayment::new(
            FUNDING_TOKEN_ID.into(),
            0,
            50u64.into(),
        ))
        .returns(ExpectError(4, "cannot fund project after deadline"))
        .run();
}

#[test]
fn test_claim_x_project_tokens() {
    let mut state = CoinbaseTestState::new();

    let depositor1 = TestAddress::new("depositor1");
    let depositor2 = TestAddress::new("depositor2");

    state
        .world
        .account(depositor1)
        .esdt_balance(FUNDING_TOKEN_ID, 50_000)
        .account(depositor2)
        .esdt_balance(FUNDING_TOKEN_ID, 15_000);

    state.start_ico();

    state.fund_project(depositor2, 1, OptionalValue::None, 15_000);
    state.fund_project(depositor1, 1, OptionalValue::None, 50_000);

    state.move_block_rounds(25);
    state.set_x_project_token(1, "Ulo Gold");

    // Claim
    state.claim_x_project_tokens(depositor1, 1usize);
    // Claim again
    state
        .world
        .tx()
        .to(X_PROJECT_FUNDING_ADDR)
        .from(depositor1)
        .typed(x_project_funding_proxy::XProjectFundingProxy)
        .claim_x_project_tokens(1usize)
        .returns(ExpectError(4, "User project deposit amount already used"))
        .run();
    // Claim for depositor2
    state.claim_x_project_tokens(depositor2, 1usize);

    let xht_id = state.get_xht_id();
    let lk_attr = |balance| LkXhtAttributes::new(0, balance);

    let depositor1_xht_claim = XHT::from_parts(5653846, 152116481831923585);
    state.check_xht_bal(depositor1, &0u64.into());
    state.check_lk_xht_bal(
        depositor1,
        1,
        &depositor1_xht_claim,
        &lk_attr(depositor1_xht_claim.clone()),
    );
    state.check_xpt_bal(1, depositor1, 1, 769_230u64.into());

    let balance = XHT::from_parts(1_696_153, 845_634_944_549_577_075);
    state.check_xht_bal(depositor2, &0u64.into());
    state.check_lk_xht_bal(depositor2, 2, &balance, &lk_attr(balance.clone()));
    state.check_xpt_bal(1, depositor2, 2, 230_769u64.into());

    state.move_block_rounds(5);

    state.check_xht_bal(depositor1, &0u64.into());

    state.unlock_xht(depositor1, 1, depositor1_xht_claim.clone());

    let depositor1_xht_claimed = XHT::from_parts(10_727, 546584920464920923);
    state
        .world
        .check_account(depositor1)
        .esdt_balance(&xht_id, &depositor1_xht_claimed);
    state.check_lk_xht_bal(
        depositor1,
        1,
        &depositor1_xht_claim,
        &lk_attr(&depositor1_xht_claim - &depositor1_xht_claimed),
    );

    // Move far away from locked duration
    state.move_block_rounds((x_project_funding::lk_xht_module::LOCK_DURATION / 6_000) + 50);
    state.unlock_xht(depositor1, 1, depositor1_xht_claim.clone());

    state.check_xht_bal(
        depositor1,
        &XHT::from_parts(5_653_846, 152_116_481_831_923_585),
    );
    state.check_lk_xht_bal(depositor1, 1, &depositor1_xht_claim, &lk_attr(0u64.into()));
}

#[test]
fn test_claim_rent_reward() {
    let mut state = CoinbaseTestState::new();

    let depositor1 = TestAddress::new("depositor1");
    let depositor2 = TestAddress::new("depositor2");

    state.start_ico();

    let xht_id = state.get_xht_id();

    state
        .world
        .account(depositor1)
        .esdt_balance(FUNDING_TOKEN_ID, 50_000)
        .account(depositor2)
        .esdt_balance(FUNDING_TOKEN_ID, 15_000);

    state.fund_project(depositor1, 1, OptionalValue::None, 50_000);
    state.fund_project(depositor2, 1, OptionalValue::Some(1), 15_000);

    state.move_block_rounds(15_000);
    let (project1_addr, project1_token_id) = state.set_x_project_token(1, "Ulo Chukwu");

    // Claim
    state.claim_x_project_tokens(depositor2, 1usize);

    state
        .world
        .tx()
        .to(&project1_addr)
        .from(X_HOUSING_ADDR)
        .typed(x_project_proxy::XProjectProxy)
        .receive_rent()
        .payment(EsdtTokenPayment::new(xht_id.clone(), 0, 20_000u64.into()))
        .run();

    // Claim rent reward
    state
        .world
        .tx()
        .to(&project1_addr)
        .from(depositor2)
        .typed(x_project_proxy::XProjectProxy)
        .claim_rent_reward()
        .payment(EsdtTokenPayment::new(project1_token_id, 1, 0u64.into()))
        .run();

    state
        .world
        .check_account(depositor2)
        .esdt_balance(&xht_id, 3_231);
    // Deposistor1 receives becase they are the referrer of depositor1
    state
        .world
        .check_account(depositor1)
        .esdt_balance(&xht_id, 230);
}

#[test]
fn test_staking() {
    let mut state = CoinbaseTestState::new();

    let depositor1 = TestAddress::new("depositor1");
    let depositor2 = TestAddress::new("depositor2");

    state.start_ico();

    state
        .world
        .account(depositor1)
        .esdt_balance(FUNDING_TOKEN_ID, 50_000)
        .account(depositor2)
        .esdt_balance(FUNDING_TOKEN_ID, 15_000);

    state.fund_project(depositor1, 1, OptionalValue::None, 50_000);
    state.fund_project(depositor2, 1, OptionalValue::Some(1), 15_000);

    state.move_block_rounds(25);

    let (project1_addr, project1_token_id) = state.set_x_project_token(1, "Ulo Chukwu");

    // Claim XPT
    state.claim_x_project_tokens(depositor2, 1usize);
    let xht_id = state.get_xht_id();
    let lk_xht_id = state.get_lk_xht_id();
    state
        .world
        .tx()
        .to(&project1_addr)
        .from(X_HOUSING_ADDR)
        .typed(x_project_proxy::XProjectProxy)
        .receive_rent()
        .payment(EsdtTokenPayment::new(xht_id.clone(), 0, 20_000u64.into()))
        .run();

    state.stake(
        depositor2,
        200,
        OptionalValue::None,
        vec![
            EsdtTokenPayment::new(project1_token_id, 1, 0u64.into()),
            EsdtTokenPayment::new(lk_xht_id, 1, 0u64.into()),
        ],
    );
}
