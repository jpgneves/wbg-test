use wasm_bindgen_test::wasm_bindgen_test;
use wbg_test::Component;

#[wasm_bindgen_test]
fn trivial() {
    let _c = Component::new();
    assert!(true)
}