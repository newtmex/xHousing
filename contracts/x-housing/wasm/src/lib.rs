// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           10
// Async Callback:                       1
// Total number of exported functions:  13

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
        add_x_project => add_x_project
        registerXst => register_xst_token
        getAffiliateDetails => get_affiliate_details
        getXhtID => xht
        getXstID => xst
        set_up_xht => set_up_xht
        add_project_rent => add_project_rent
        stake => stake
    )
}

multiversx_sc_wasm_adapter::async_callback! { x_housing }
