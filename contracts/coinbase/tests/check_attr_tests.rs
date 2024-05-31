use multiversx_sc::{codec::TopDecode, types::ManagedBuffer};
use multiversx_sc_scenario::{api::StaticApi, DebugApi};
use x_project_funding::lk_xht_module::LkXhtAttributes;

fn decode_values<T: TopDecode + std::fmt::Debug>(values: Vec<&str>) {
    for (index, value) in values.into_iter().enumerate() {
        if index > 0 {
            println!("\n");
        }
        println!(
            "Value at index {index}: {:#?}",
            T::top_decode::<ManagedBuffer<StaticApi>>(hex::decode(&value[2..]).unwrap().into())
                .unwrap()
        )
    }
}

#[ignore = "for checks"]
#[test]
fn decode_values_test() {
    DebugApi::dummy();

    decode_values::<LkXhtAttributes<StaticApi>>(vec![
        "0x0000000b04aceed82ace5cf6ad626e0000000000003a980000000005a7c998",
        "0x0000000b04aceed82ace5cf6ad626e00000000000000000000000005a78f00",
    ]);
}
