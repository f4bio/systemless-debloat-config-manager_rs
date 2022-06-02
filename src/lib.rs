extern crate web_sys;

use serde::Serialize;
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ConfigHandler {
  config_string: String,
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Link {
  pretty: String,
  url: String,
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct User {
  name: String,
  position: String,
}

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

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

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
  log("Hello from Rust!");

  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");

  console_log!("Hello {}!", "world");

  // Manufacture the element we're gonna append
  let val = document.create_element("p")?;
  val.set_text_content(Some("Hello from Rust!"));

  body.append_child(&val)?;

  Ok(())
}

#[wasm_bindgen]
impl ConfigHandler {
  pub fn new() -> ConfigHandler {
    utils::set_panic_hook();

    ConfigHandler {
      config_string: utils::get_template(),
    }
  }

  pub fn get_template_string(&self) -> String {
    String::from(&self.config_string)
  }

  pub fn prepare(&self, data: &str) -> String {
    let prepared_data = data.replace("{", "{{").replace("}", "}}");
    console_log!("prepared data: {}", prepared_data);

    prepared_data
  }
}
