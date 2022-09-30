use js_sys::JSON;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn req() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let target_elem = document.get_element_by_id("target").unwrap();
    let mut map = HashMap::new();

    match document.location() {
        Some(location) => {
            map.insert("hostname", location.hostname().unwrap());
            map.insert("pathname", location.pathname().unwrap());
            let response = reqwest::Client::new()
                .post(&format!("{}/api/sample", location.origin().unwrap()))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .header(reqwest::header::ORIGIN, location.origin().unwrap())
                .json(&map)
                .send()
                .await;
            match response {
                Err(e) => {
                    web_sys::console::log_1(&format!("Error : status ({:?})", e.status()).into());
                    if e.is_redirect() {
                        web_sys::console::log_1(&"Redirect error".into());
                    }
                    if e.is_builder() {
                        web_sys::console::log_1(&"Builder error".into());
                    }
                    if e.is_timeout() {
                        web_sys::console::log_1(&"Timeout error".into());
                    }
                    if e.is_request() {
                        web_sys::console::log_1(&"Request error".into());
                    }
                    if e.is_body() {
                        web_sys::console::log_1(&"Body error".into());
                    }
                    if e.is_decode() {
                        web_sys::console::log_1(&"Decode error".into());
                    }
                },
                Ok(res) => {
                    let mut res_string: String;
                    res_string = format!("\"status\": {:?}\n", res.status());
                    for (key, value) in res.headers().iter() {
                        res_string = format!("{}{:?}: {:?}\n", res_string, key, value);
                    }
                    res_string = format!("{}\"body\":\n", res_string);

                    let json = JSON::parse(&(res.text().await.unwrap_or("".to_string()))).unwrap_or(JsValue::NULL);
                    match JSON::stringify_with_replacer_and_space(
                            &json,
                            &JsValue::NULL,
                            &JsValue::from_str("\t"),
                        ) {
                        Err(_) => {},
                        Ok(x) => {
                            match x.as_string() {
                                None => {},
                                Some(y) => {
                                    res_string = format!("{}{}", res_string, y);
                                },
                            };
                        },
                    };
                    web_sys::console::log_1(&format!("{:?}", res_string).into());
                    target_elem.set_text_content(Some(&format!("{}", res_string)));
                },
            };
        },
        None => {
            web_sys::console::log_1(&"Error : document.location".into());
        },
    };
}
