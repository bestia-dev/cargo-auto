// src/lib.rs
// This file has just the wasm_bindgen_start() function
// and calls into main_mod.rs.
// So the structure of the project modules can be similar to a binary CLI executable.

#![doc=include_str!("../README.md")]

use wasm_bindgen::prelude::*;

mod main_mod;
/// LibraryError must be accessible in every module.
pub use main_mod::LibraryError;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    dbg!("pwa_short_name v{}", env!("CARGO_PKG_VERSION"));

    main_mod::main();
    // return
    Ok(())
}
