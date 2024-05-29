// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            9
// Async Callback:                       1
// Total number of exported functions:  12

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    x_project
    (
        init => init
        upgrade => upgrade
        claimRentReward => claim_rent_reward
        registerXPToken => register_xp_token
        mint_xp_token => mint_xp_token
        getXPTokenId => xp_token
        getXPTokenSupply => xp_token_supply
        getXPTokenMaxSupply => xp_token_max_supply
        receiveRent => receive_rent
        getRewardPerShare => reward_per_share
        getXhtID => xht
    )
}

multiversx_sc_wasm_adapter::async_callback! { x_project }
