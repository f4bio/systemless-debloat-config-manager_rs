extern crate web_sys;

use gloo::console::log;
use gloo::timers::callback::Timeout;
use gloo::utils::document;
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

  system_apps.iter().for_each(|app| {
    log!("system_app: {:?}", app);

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

  system_packages.iter().for_each(|package| {
    log!("system_package: {:?}", package);

    let html = format!(
      r##"
      <li class="list-group-item d-flex justify-content-between align-items-start">
        <div class="ms-2 me-auto">
         <div class="fw-bold">Subheading</div>
         {}
         </div>
        <span class="badge bg-primary rounded-pill">14</span>
        </li>"##,
      package
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

  Timeout::new(1_000, move || {
    // Do something after the one second timeout is up!
    loading_container_element
      .unwrap()
      .toggle_attribute("hidden")
      .expect("TODO: panic message");
    input_container_element
      .unwrap()
      .toggle_attribute("hidden")
      .expect("TODO: panic message");
  })
  .forget();

  Ok(())
}
