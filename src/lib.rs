extern crate web_sys;

use serde::Serialize;
use tera::{Context, Tera};
use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PrepareConfig {
  template_string: String,
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

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

#[wasm_bindgen]
impl PrepareConfig {
  pub fn new() -> PrepareConfig {
    utils::set_panic_hook();

    PrepareConfig {
      template_string: utils::get_template(),
    }
  }

  pub fn get_template_string(&self) -> String {
    String::from(&self.template_string)
  }

  pub fn interpolate(
    &self,
    name: &str,
    position: &str,
    phone_input: &str,
    email_input: &str,
    website_input: &str,
  ) -> String {
    let user = User {
      name: name.to_string(),
      position: position.to_string(),
    };

    let phone = Link {
      pretty: phone_input.to_string(),
      url: format!("tel:+49{}", phone_input.to_string()),
    };

    let email = Link {
      pretty: email_input.to_string(),
      url: format!("mailto:{}", email_input.to_string()),
    };

    let website = Link {
      pretty: website_input.to_string(),
      url: format!("https://www.{}", website_input),
    };

    let mut context = Context::new();
    context.insert("user", &user);
    context.insert("phone", &phone);
    context.insert("email", &email);
    context.insert("website", &website);

    Tera::one_off(self.get_template_string().as_str(), &context, false).unwrap()
  }
}
