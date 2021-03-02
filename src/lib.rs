use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "react")]
extern "C" {
    #[wasm_bindgen(module = "react")]
    pub type Component;

    #[wasm_bindgen(constructor, module = "react")]
    pub fn new() -> Component;
}