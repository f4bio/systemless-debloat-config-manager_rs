//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

use esig_t3mplar::SignatureTemplate;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_nothing() -> SignatureTemplate {
    let mut signature_template = SignatureTemplate::new();
    signature_template
}

#[cfg(test)]
pub fn expected_nothing() -> SignatureTemplate {
    let mut signature_template = SignatureTemplate::new();
    signature_template
}

#[wasm_bindgen_test]
pub fn test_new() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_object = input_nothing();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_object = expected_nothing();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    // input_universe.tick();
    assert_eq!(&input_object.get_template_string(), &expected_object.get_template_string());
}
