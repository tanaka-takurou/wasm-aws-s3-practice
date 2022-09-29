use std::collections::HashMap;
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
            let res = reqwest::Client::new()
                .post(&format!("{}/api/sample", location.origin().unwrap()))
                .json(&map)
                .send()
                .await
                .unwrap();
            let mut res_string: String;
            res_string = format!("\"status\": {:?}\n", res.status());
            for (key, value) in res.headers().iter() {
                res_string = format!("{}{:?}: {:?}\n", res_string, key, value);
            }
            res_string = format!("{}\"body\": {:?}\n", res_string, res.text().await.unwrap());
            web_sys::console::log_1(&format!("{:?}", res_string).into());
            target_elem.set_text_content(Some(&format!("{}", res_string)));
        },
        None => {
            web_sys::console::log_1(&"Error : document.location".into());
        },
    };
}
