#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen;
extern crate wasm_bindgen_test;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
pub struct ConsumeRetString;

#[wasm_bindgen]
impl ConsumeRetString {
    // https://github.com/rustwasm/wasm-bindgen/issues/329#issuecomment-411082013
    //
    // This used to cause two `const ptr = ...` declarations, which is invalid
    // JS.
    pub fn consume(self) -> String {
        String::new()
    }
}

#[wasm_bindgen_test]
fn works() {
    ConsumeRetString.consume();
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen_test]
fn can_log_html_strings() {
    log("<script>alert('lol')</script>");
}
