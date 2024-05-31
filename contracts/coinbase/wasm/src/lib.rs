// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            5
// Async Callback:                       1
// Total number of exported functions:   8

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    coinbase
    (
        init => init
        upgrade => upgrade
        register_xht => register_xht
        getXhtID => xht
        feed_x_housing => feed_x_housing
        lastDispatchEpoch => x_housing_is_dispatched
        start_ico => start_ico
    )
}

multiversx_sc_wasm_adapter::async_callback! { coinbase }