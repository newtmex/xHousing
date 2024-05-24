use multiversx_sc::{
    imports::OptionalValue,
    types::{TestAddress, TestSCAddress},
};
use multiversx_sc_scenario::imports::*;

use x_housing::*;

const X_HOUSING_CODE_PATH: MxscPath = MxscPath::new("output/x-housing.mxsc.json");
const X_HOUSING_ADDR: TestSCAddress = TestSCAddress::new("x-hosuing");
const X_HOUSING_OWNER_ADDR: TestAddress = TestAddress::new("x-housing-owner");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(X_HOUSING_CODE_PATH, x_housing::ContractBuilder);

    blockchain
}

pub struct XHousingTestState {
    world: ScenarioWorld,
}

impl XHousingTestState {
    fn new() -> Self {
        let mut world = world();

        world.account(X_HOUSING_OWNER_ADDR);

        Self { world }
    }

    fn deploy_x_housing(&mut self) {
        self.world
            .tx()
            .from(X_HOUSING_OWNER_ADDR)
            .typed(x_housing_proxy::XHousingProxy)
            .init()
            .code(X_HOUSING_CODE_PATH)
            .new_address(X_HOUSING_ADDR)
            .run();
    }

    fn create_ref_id(
        &mut self,
        sender: TestAddress,
        expected_ref_id: usize,
        referrer_id: Option<usize>,
    ) {
        self.world
            .tx()
            .from(sender)
            .to(X_HOUSING_ADDR)
            .typed(x_housing_proxy::XHousingProxy)
            .create_ref_id(OptionalValue::from(referrer_id))
            .returns(ExpectValue(expected_ref_id))
            .run();

        if let Some(referrer_id) = referrer_id {
            let (user_id, referrer_data) = self
                .world
                .query()
                .to(X_HOUSING_ADDR)
                .typed(x_housing_proxy::XHousingProxy)
                .get_affiliate_details(sender)
                .returns(ReturnsResult)
                .run();

            assert_eq!(user_id, expected_ref_id, "ref id mismatch");
            assert!(referrer_data.is_some(), "no referrer data returned");
            assert_eq!(
                referrer_data.unwrap().0,
                referrer_id,
                "referrer id mismatch"
            )
        }
    }
}

#[test]
fn test_create_ref_id() {
    let mut state = XHousingTestState::new();

    state.deploy_x_housing();

    let user1_addr = TestAddress::new("user1");
    let user2_addr = TestAddress::new("user2");

    state.world.account(user1_addr).account(user2_addr);

    state.create_ref_id(user1_addr, 1, None);
    state.create_ref_id(user2_addr, 2, Some(1));
}

#[test]
fn test_create_ref_id_sc_not_allowed() {
    let mut state = XHousingTestState::new();

    state.deploy_x_housing();

    let sc_addr = TestSCAddress::new("i-am-smart-contract");
    state.world.account(sc_addr).code(X_HOUSING_CODE_PATH);

    state
        .world
        .tx()
        .from(sc_addr)
        .to(X_HOUSING_ADDR)
        .typed(x_housing_proxy::XHousingProxy)
        .create_ref_id(OptionalValue::<usize>::None)
        .returns(ExpectError(4, "caller is smart contract"))
        .run();
}
