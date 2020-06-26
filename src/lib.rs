mod utils;

use wasm_bindgen::prelude::*;

extern crate js_sys;

mod Game;

// use serde::Serialize;
// #[macro_use]
// extern crate serde_derive;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, poker-game!");
}

/*

heart = 0
club = 1
diamond = 2
spades= 3
error = 4

*/

#[wasm_bindgen]
pub fn pass_value_to_js() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&Game::Game::new_game()).map_err(|err| err.into())
}
