use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn req() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let target_elem = document.get_element_by_id("target").unwrap();
    match document.location() {
        Some(location) => {
            let text = reqwest::Client::new()
                .get(&format!("{}/sitemap.xml", location.origin().unwrap()))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            web_sys::console::log_1(&format!("{:?}", text).into());
            target_elem.set_text_content(Some(&text));
        },
        None => {
            web_sys::console::log_1(&"Error : document.location".into());
        },
    };
}
