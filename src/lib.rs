extern crate web_sys;

use gloo::{events, events::EventListener, timers::callback::Timeout};
use wasm_bindgen::prelude::*;
use web_sys::console;

mod parser;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  // The `console.log` is quite polymorphic, so we can bind it with multiple
  // signatures. Note that we need to use `js_name` to ensure we always call
  // `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  console_log!("Hello wasm from macro!");

  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  let window: web_sys::Window = web_sys::window().expect("no global `window` exists");
  let document: web_sys::Document = window.document().expect("should have a document on window");
  // let body = document.body().expect("document should have a body");

  let file_select_input = document.get_element_by_id("selectFile").unwrap_throw();
  let on_change = EventListener::new(&file_select_input, "change", move |_event| {
    // After a one second timeout, update the button's text content.
    Timeout::new(1_000, move || {
      console_log!("selectFile clicked");
    })
    .forget();
  });
  console_log!("on_change = {:?}", on_change);

  // let body = document.body().expect("document should have a body");
  //
  // Manufacture the element we're gonna append
  // let val = document.create_element("p")?;
  // val.set_text_content(Some("Hello from Rust!"));
  //
  // body.append_child(&val)?;

  // selectFileElement = document("selectFile");

  Ok(())
}
