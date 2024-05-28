use coinbase::*;

use multiversx_sc::types::{TestAddress, TestSCAddress};
use multiversx_sc_scenario::{api::StaticApi, imports::*, ScenarioWorld};
use x_housing_module::x_housing::{self, x_housing_proxy};
use x_project::x_project_proxy;
use x_project_funding::x_project_funding_proxy;
use xht::XHTTrait;

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

const XHT_ID: &str = "str:XCR-123456";
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

const BLOCKS_PER_EPOCH: u64 = 14_400;

struct CoinbaseTestState {
    world: ScenarioWorld,
}

impl CoinbaseTestState {
    fn new() -> Self {
        let mut world = world();

        world.account(CONTRACTS_OWNER);

        Self { world }
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
            .init()
            .code(X_PROJECT_TEMPLATE_CODE_PATH)
            .new_address(X_PROJECT_TEMPLATE_ADDR)
            .run();
    }

    fn deploy_x_project_funding(&mut self) {
        self.world
            .tx()
            .from(CONTRACTS_OWNER)
            .typed(x_project_funding::x_project_funding_proxy::XProjectFundingProxy)
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
    }
}

#[test]
fn test_x_housing_genesis() {
    let mut current_block: u64 = 124_138;
    let mut current_epoch = current_block / BLOCKS_PER_EPOCH;

    let mut move_blocks = |blocks: u64| {
        current_block += blocks;
        current_epoch = current_block / BLOCKS_PER_EPOCH;

        SetStateStep::new()
            .block_epoch(current_epoch)
            .block_round(current_block)
    };

    let mut state = CoinbaseTestState::new();

    state.deploy_coinbase();
    state.deploy_x_housing();
    state.register_xht();

    state
        .world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .gas(90_000_000)
        .typed(coinbase_proxy::CoinbaseProxy)
        .feed_x_housing(OptionalAddress::Some(X_HOUSING_ADDR.to_address().into()))
        .run();

    // Feeding too early
    state
        .world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .gas(90_000_000)
        .typed(coinbase_proxy::CoinbaseProxy)
        .feed_x_housing(OptionalAddress::None)
        .returns(ExpectError(4, "feed epoch not reached"))
        .run();

    state
        .world
        .set_state_step(move_blocks(BLOCKS_PER_EPOCH * 25));
    state
        .world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .gas(90_000_000)
        .typed(coinbase_proxy::CoinbaseProxy)
        .feed_x_housing(OptionalAddress::None)
        .run();
}

#[test]
fn test_start_ico() {
    let mut state = CoinbaseTestState::new();

    state.start_ico();

    let xht_token = state
        .world
        .query()
        .to(X_PROJECT_FUNDING_ADDR)
        .typed(x_project_funding_proxy::XProjectFundingProxy)
        .xht()
        .returns(ReturnsResult)
        .run();

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
        .run();

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
