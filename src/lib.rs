extern crate web_sys;

use regex::RegexSetBuilder;
use wasm_bindgen::prelude::*;

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

fn find_system_apps(data: String) -> Vec<String> {
  let mut system_apps: Vec<String> = vec![];
  let mut eat: bool = false;

  for line in data.lines() {
    if eat {
      system_apps.push(String::from(line.clone()))
    }
    if line.eq("System apps, not debloated:") && eat == false {
      eat = true;
    }
    if line.eq("") && eat == true {
      eat = false;
    }
  }
  system_apps
}

fn find_system_packages(data: String) -> Vec<String> {
  let mut system_packages: Vec<String> = vec![];
  let mut eat: bool = false;

  for line in data.lines() {
    if eat {
      system_packages.push(String::from(line.clone()))
    }
    if line.eq("System packages:") && eat == false {
      eat = true;
    }
    if line.eq("") && eat == true {
      eat = false;
    }
  }
  system_packages
}

#[wasm_bindgen]
pub fn parse(data: String) -> Result<(), web_sys::ErrorEvent> {
  let system_apps = find_system_apps(data.clone());
  let system_packages = find_system_packages(data.clone());

  console_log!("system_apps: {:?}", system_apps);
  console_log!("system_packages: {:?}", system_packages);

  Ok(())
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
  utils::set_panic_hook();

  console_log!("Hello wasm from macro!");

  // TODO:
  // Use `web_sys`'s global `window` function to get a handle on the global
  // window object.
  // let window: web_sys::Window = web_sys::window().expect("no global `window` exists");
  // let document: web_sys::Document = window.document().expect("should have a document on window");
  // let body = document.body().expect("document should have a body");

  // let file_select_input: web_sys::Element = document.get_element_by_id("selectFile").unwrap_throw();
  // let on_change = EventListener::new(&file_select_input, "change", move |_event| {
  //   console_log!("file_select_input changed");
  //
  //   let event: &JsValue = _event.dyn_ref::<JsValue>().unwrap_throw();
  //   console_log!("event: {:?}", event);
  //
  // EventTarget { obj: Object { obj: JsValue(HTMLInputElement) } }
  // let element: HtmlInputElement = _event
  //   .target()
  //   .unwrap()
  //   .dyn_into::<HtmlInputElement>()
  //   .unwrap();
  // console_log!("element: {:?}", element);
  // });
  // on_change.forget();
  //
  // let input_form: web_sys::Element = document.get_element_by_id("fileInputForm").unwrap_throw();
  // let on_submit = EventListener::new(&input_form, "submit", move |_event| {
  //   console_log!("input_form submitted");
  // });
  // on_submit.forget();

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
