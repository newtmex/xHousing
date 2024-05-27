use coinbase::*;

use multiversx_sc::types::{TestAddress, TestSCAddress};
use multiversx_sc_scenario::{api::StaticApi, imports::*, ScenarioWorld};
use x_housing::x_housing_proxy;
use xht::XHTTrait;

type Xht = xht::XHT<StaticApi>;
type OptionalAddress = OptionalValue<ManagedAddress<StaticApi>>;

const CONTRACTS_OWNER: TestAddress = TestAddress::new("contracts-owner");
const COINBASE_ADDR: TestSCAddress = TestSCAddress::new("coinbase");
const X_HOUSING_ADDR: TestSCAddress = TestSCAddress::new("x-housing");
const COINBASE_CODE_PATH: MxscPath = MxscPath::new("output/coinbase.mxsc.json");
const X_HOUSING_CODE_PATH: MxscPath = MxscPath::new("../x-housing/output/x-housing.mxsc.json");

const XHT_ID: &str = "str:XCR-123456";
const XHT_STORE_KEY: &str = "str:xht-module::xht";

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::new();

    world.register_contract(COINBASE_CODE_PATH, coinbase::ContractBuilder);
    world.register_contract(X_HOUSING_CODE_PATH, x_housing::ContractBuilder);

    world
}

const BLOCKS_PER_EPOCH: u64 = 14_400;

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

    let mut world = world();

    world.start_trace().account(CONTRACTS_OWNER);
    // .new_address(CONTRACTS_OWNER, 0, COINBASE_ADDR)
    // .new_address(CONTRACTS_OWNER, 1, X_HOUSING_ADDR);

    world
        .tx()
        .from(CONTRACTS_OWNER)
        .typed(coinbase_proxy::CoinbaseProxy)
        .init()
        .code(COINBASE_CODE_PATH)
        .new_address(COINBASE_ADDR)
        .run();

    world
        .tx()
        .from(CONTRACTS_OWNER)
        .typed(x_housing_proxy::XHousingProxy)
        .init(COINBASE_ADDR)
        .code(X_HOUSING_CODE_PATH)
        .new_address(X_HOUSING_ADDR)
        .run();

    world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .gas(90_000_000)
        .typed(coinbase_proxy::CoinbaseProxy)
        .register_xht()
        .run();

    // world.account(COINBASE_ADDR).

    let mut coinbase_state = Account::new()
        .owner(CONTRACTS_OWNER)
        .esdt_balance(XHT_ID, Xht::max_supply());
    coinbase_state
        .storage
        .insert(XHT_STORE_KEY.into(), XHT_ID.into());
    let coinbase_code = world.code_expression(&COINBASE_CODE_PATH.eval_to_expr());
    coinbase_state.code = Some(coinbase_code);

    world.set_state_step(SetStateStep::new().put_account(COINBASE_ADDR, coinbase_state));

    world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .gas(90_000_000)
        .typed(coinbase_proxy::CoinbaseProxy)
        .feed_x_housing(OptionalAddress::Some(X_HOUSING_ADDR.to_address().into()))
        .run();

    // Feeding too early
    world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .gas(90_000_000)
        .typed(coinbase_proxy::CoinbaseProxy)
        .feed_x_housing(OptionalAddress::None)
        .returns(ExpectError(4, "feed epoch not reached"))
        .run();

    world.set_state_step(move_blocks(BLOCKS_PER_EPOCH * 25));
    world
        .tx()
        .from(CONTRACTS_OWNER)
        .to(COINBASE_ADDR)
        .gas(90_000_000)
        .typed(coinbase_proxy::CoinbaseProxy)
        .feed_x_housing(OptionalAddress::None)
        .run();
}
