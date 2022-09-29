use js_sys::JSON;
use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn check(s: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let tmp_elem = document.get_element_by_id("data").expect("no data-element");
    let data_elem = tmp_elem.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
    let message_elem = document.get_element_by_id("message").expect("no message-element");
    if s.is_empty() {
        message_elem.set_text_content(Some(&"No Data."));
    } else {
        let json = JSON::parse(s).unwrap_or(JsValue::NULL);
        match Object::try_from(&json) {
            None => {
                message_elem.set_text_content(Some(&"Syntax Error."));
            },
            Some(v) => {
                if Object::keys(v).length() == 0 {
                    message_elem.set_text_content(Some(&"Syntax Error."));
                } else {
                    match JSON::stringify_with_replacer_and_space(
                            &json,
                            &JsValue::NULL,
                            &JsValue::from_str("\t"),
                        ) {
                        Err(_) => {
                            message_elem.set_text_content(Some(&"Syntax Error."));
                        },
                        Ok(x) => {
                            match x.as_string() {
                                None => {
                                    message_elem.set_text_content(Some(&"Syntax Error."));
                                },
                                Some(y) => {
                                    message_elem.set_text_content(Some(&"Syntax OK."));
                                    data_elem.set_value(&y);
                                },
                            };
                        },
                    };
                }
            },
        };
    }
}
