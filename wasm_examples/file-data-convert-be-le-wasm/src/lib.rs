use std::convert::TryInto;
use wasm_bindgen::prelude::*;

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

fn convert_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn vec_u8_to_vec_u64(v: Vec<u8>, be: bool) -> (Vec<u64>, Vec<u8>) {
    let mut buf = Vec::new();

    let u64_u8 = 8;
    let param_a = v.len() / u64_u8;
    let param_b = v.len() % u64_u8;
    for i in 0..param_a {
        let x = (u64_u8 * i) as usize;
        let x_array: [u8; 8] = convert_to_array(v[x..(x + u64_u8)].to_vec());
        if be {
            buf.push(u64::from_be_bytes(x_array));
        } else {
            buf.push(u64::from_le_bytes(x_array));
        }
    }

    (buf, v[(v.len() - param_b)..v.len()].to_vec())
}

fn convert_main(v: Vec<u8>, be: bool) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let (u64_vec, remainder) = vec_u8_to_vec_u64(v.to_vec(), be);
    buf.try_reserve(v.len() - remainder.len()).unwrap();
    if be {
        for w in u64_vec {
            buf.extend(w.to_le_bytes().to_vec());
        }
    } else {
        for w in u64_vec {
            buf.extend(w.to_be_bytes().to_vec());
        }
    }
    if remainder.len() > 0 {
        buf.extend(remainder.to_vec());
    }
    buf
}

#[wasm_bindgen]
pub fn restoration(data: Vec<u8>) {
    download_result_file(convert_main(data, false), "text/plain", "restoration.txt");
}

#[wasm_bindgen]
pub fn convert(data: Vec<u8>) {
    download_result_file(convert_main(data, true), "text/plain", "converted.txt");
}
