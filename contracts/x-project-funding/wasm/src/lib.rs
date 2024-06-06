// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           13
// Async Callback:                       1
// Total number of exported functions:  16

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
        unlockXht => unlock_xht
        getXhtID => xht
        getLkXhtID => lk_xht
        registerLkXht => register_lk_xht_token
        set_x_project_token => set_x_project_token
        claimXProjectToken => claim_x_project_tokens
        getXProjectTokenID => x_project_token_id
        getXProjectAddress => x_project_address
        getXProjectData => x_project_data
        getAllXProjectData => all_x_projects
    )
}

multiversx_sc_wasm_adapter::async_callback! { x_project_funding }
