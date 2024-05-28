// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            5
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    x_housing
    (
        init => init
        upgrade => upgrade
        createRefID => create_ref_id
        create_ref_id_via_proxy => create_ref_id_via_proxy
        getAffiliateDetails => get_affiliate_details
        top_up_xht => top_up_xht
        getXhtID => xht
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
