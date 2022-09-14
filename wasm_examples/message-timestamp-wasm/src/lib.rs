use chrono::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn update() {
    let utc: DateTime<Utc> = Utc::now();
    web_sys::console::log_1(&format!("{:?}.{:?}", utc.timestamp(), utc.timestamp_subsec_millis()).into());
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let target_elem = document.get_element_by_id("target").unwrap();
    target_elem.set_text_content(Some(&format!("{:?}.{:?}", utc.timestamp(), utc.timestamp_subsec_millis())));
}
