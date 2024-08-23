// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback:                       1
// Total number of exported functions:  18

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    mystery_box
    (
        init => init
        setupMysteryBox => setup_mystery_box
        updateMysteryBoxUris => update_mystery_box_uris
        issue => issue
        get_token_issued => get_token_issued
        set_roles => set_roles
        createMysteryBox => create_mystery_box
        openMysteryBox => open_mystery_box
        lastErrorMessage => last_error_message
        getMysteryBoxTokenIdentifier => mystery_box_token_id
        getGlobalCooldownEpoch => global_cooldown_epoch
        getWinningRates => winning_rates
        getMysteryBoxUris => mystery_box_uris
        isAdmin => is_admin
        addAdmin => add_admin
        removeAdmin => remove_admin
        getAdmins => admins
    )
}

multiversx_sc_wasm_adapter::async_callback! { mystery_box }
