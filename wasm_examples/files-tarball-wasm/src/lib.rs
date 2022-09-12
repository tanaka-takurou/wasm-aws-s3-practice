use serde::Deserialize;
use std::io::{Cursor, Read};
use tar::{Archive, Builder, Header};
use wasm_bindgen::prelude::*;

#[derive(Default, Deserialize)]
struct Data {
    name: String,
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
pub fn extract(data: Vec<u8>) {
    let mut tmp = data.to_vec();
    let mut a = Archive::new(Cursor::new(&mut tmp));
    for file in a.entries().unwrap() {
        let mut buf = Vec::new();
        let mut tmp = file.unwrap();
        let _ = tmp.read_to_end(&mut buf).unwrap();
        let file_name = &tmp.path().unwrap().into_owned().to_string_lossy().into_owned();
        let guess_mime = mime_guess::from_path(file_name);
        let content_type = format!("{:?}/{:?}",
            guess_mime.first_or_text_plain().type_(), guess_mime.first_or_text_plain().subtype());
        download_result_file(buf, &content_type, file_name);
    }
}

#[wasm_bindgen]
pub fn archive(val: JsValue) {
    let data: Vec<Data> = serde_wasm_bindgen::from_value(val).unwrap();
    let mut a = Builder::new(Vec::new());
    for v in data {
        let mut header = Header::new_gnu();
        header.set_size(v.value.len() as u64);
        header.set_cksum();
        a.append_data(&mut header, v.name, &(v.value)[..]).unwrap();
    }
    download_result_file(a.into_inner().unwrap(), "text/plain", "archive.tar");
}
