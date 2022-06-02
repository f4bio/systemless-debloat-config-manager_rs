pub static TEMPLATE: &str = "
<div style=\"font-family: Arial, sans-serif;\">
  <p style=\"color: rgb(68, 68, 68); font-size: 1.2em; font-weight: bold\">
  {{ user.name }}
  </p>
  <p style=\"color: rgb(51, 204, 204); font-size: 1.3em; font-weight: bold\">
  {{ user.position }}
  </p>
  <p>
  <ul style=\"list-style: none; font-size: 0.8em; margin-left: 0; padding-left: 0; line-height: 1em;\">
    <li>
      <span style=\"color: #999999\">Mobile:</span>
      <a href=\"{{ phone.url }}\" target=\"_blank\">{{ phone.pretty }}</a>
    </li>
    <li>
      <span style=\"color: #999999\">E-Mail:</span>
      <a href=\"{{ email.url }}\" target=\"_blank\">{{ email.pretty }}</a>
    </li>
    <li>
      <span style=\"color: #999999\">Website:</span>
      <a href=\"{{ website.url }}\" target=\"_blank\">{{ website.pretty }}</a>
    </li>
  </ul>
  </p>
  <p style=\"color: rgb(51, 204, 204); font-size: 1.3em; font-weight: bold\">
    Digital Venture Consultants
  </p>
  <p>
    <ul style=\"list-style: none; font-size: 0.7em; margin-left: 0; padding-left: 0; line-height: 1em;\">
      <li>
        <span style=\"color: #999999\">ZiTOS GmbH</span>
      </li>
      <li>
        <span style=\"color: #999999\">Am Silberberg 16</span>
      </li>
      <li>
        <span style=\"color: #999999\">65510 Hünstetten</span>
      </li>
      <li>
        <span style=\"color: #999999\">
          <a href=\"https://dvc.ventures/imprint/\" target=\"_blank\">Impressum</a>
          &amp;
          <a href=\"https://dvc.ventures/privacy-policy/\" target=\"_blank\">Datenschutz</a>
        </span>
      </li>
    </ul>
  </p>
</div>
";

pub fn set_panic_hook() {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

pub fn get_template() -> String {
  String::from(TEMPLATE)
}
