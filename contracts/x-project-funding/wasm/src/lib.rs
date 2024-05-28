// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            4
// Async Callback (empty):               1
// Total number of exported functions:   7

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    x_project_funding
    (
        init => init
        upgrade => upgrade
        init_first_x_project => init_first_x_project
        deployXProject => deploy_x_project
        fundProject => fund_project
        getXhtID => xht
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
