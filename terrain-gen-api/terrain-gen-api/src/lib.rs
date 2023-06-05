#![feature(lazy_cell)]
use std::sync::LazyLock;
use num::BigUint as bigint; 
use wasm_bindgen::prelude::*;
use web_sys::console;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;
// mod utils;


pub (crate) static ZERO: LazyLock<bigint> = LazyLock::new(|| bigint::from(0u32));
pub (crate) static ONE: LazyLock<bigint> = LazyLock::new(|| bigint::from(1u32));
pub (crate) static TWO: LazyLock<bigint> = LazyLock::new(|| bigint::from(2u32));
pub (crate) static THREE: LazyLock<bigint> = LazyLock::new(|| bigint::from(3u32));



