use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Default, Deserialize)]
struct Data {
    _name: String,
    value: Vec<u8>,
}

fn download_result_file(result_bytes: Vec<u8>, mime_type: &str, file_name: &str) {
    let u8_array = js_sys::Uint8Array::new_with_length(result_bytes.len() as u32);
    u8_array.copy_from(&result_bytes[..]);
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&u8_array.buffer());
    let mut blob_property_bag = web_sys::BlobPropertyBag::new();
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, blob_property_bag.type_(mime_type)).unwrap();
    let downlad_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let a_elem = document.create_element("a").unwrap();
    let _ = a_elem.set_attribute("href", &downlad_url);
    let _ = a_elem.set_attribute("download", file_name);

    let mouse_event = web_sys::MouseEvent::new("click").unwrap();
    mouse_event.init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(
        "click", true, true, Some(&window));
    let _ = a_elem.dispatch_event(&web_sys::Event::from(mouse_event));
}

#[wasm_bindgen]
pub fn join(val: JsValue) {
    let data: Vec<Data> = serde_wasm_bindgen::from_value(val).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    for v in data {
        buf.extend(v.value);
    }
    download_result_file(buf, "text/plain", "join.txt");
}

#[wasm_bindgen]
pub fn split(data: Vec<u8>) {
    let mut count = 0;
    let split_limit = 10;
    let split_length = (data.len() as f64 / split_limit as f64).ceil() as usize;
    loop {
        let tmp = match (count + 1) * split_length >= data.len() {
            true => data.len(),
            _    => (count + 1) * split_length,
        };
        download_result_file(data[(count * split_length)..tmp].to_vec(), "text/plain", &format!("split{:?}.txt", count));
        if tmp == data.len() {
            break;
        }
        count += 1;
    }
}
