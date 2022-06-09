extern crate web_sys;

use gloo::console::log;
use gloo::events::EventListener;
use gloo::timers::callback::Timeout;
use gloo::utils::{body, document};
use wasm_bindgen::prelude::*;

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
  // let system_packages = find_system_packages(data.clone());

  // console_log!("system_apps: {:?}", system_apps);
  // console_log!("system_packages: {:?}", system_packages);

  system_apps.iter().for_each(|app| {
    console_log!("system_app: {:?}", app);

    let html = format!(
      r##"
      <li class="list-group-item d-flex justify-content-between align-items-start">
        <div class="ms-2 me-auto">
         <div class="fw-bold">Subheading</div>
         {}
         </div>
        <span class="badge bg-primary rounded-pill">14</span>
        </li>"##,
      app
    );
    log!("{}", html);
  });

  Ok(())
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  utils::set_panic_hook();

  log!("Hello wasm from macro!");

  let loading_container_element: Option<web_sys::Element> =
    document().get_element_by_id("loading-container");
  let input_container_element: Option<web_sys::Element> =
    document().get_element_by_id("input-container");
  let file_input_element: Option<web_sys::Element> = document().get_element_by_id("selectFile");
  // let result_container_element: Option<web_sys::Element> =
  //   document().get_element_by_id("result-container");
  // let result_list_element = root.find("#result-list");

  let timeout = Timeout::new(1_000, move || {
    // Do something after the one second timeout is up!
    loading_container_element
      .unwrap()
      .toggle_attribute("hidden")
      .expect("TODO: panic message");
    input_container_element
      .unwrap()
      .toggle_attribute("hidden")
      .expect("TODO: panic message");
  });
  // Since we don't plan on cancelling the timeout, call `forget`.
  timeout.forget();

  let listener = EventListener::new(&file_input_element.unwrap(), "change", move |event| {
    log!("Hello wasm event listener!");

    // let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
    //
    // // ...
  });

  Ok(())
}
