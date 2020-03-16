// INIT
mod utils;
use wasm_bindgen::prelude::*; // interface for javascript

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// window.alert JavaScript function
#[wasm_bindgen]
extern {
    // import this function from javascript
    fn alert(s: &str);
}

//Define your own personal methods
#[wasm_bindgen]
pub fn greet(s: &str) {
    // string format
    alert(&format!("Hello, {}!", s));
}
