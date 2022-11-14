use base64::encode;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Clone, Default, Deserialize)]
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

fn get_target_data(data: Vec<Data>, target_ext: &str) -> Data {
    let mut res = Data{name: String::new(), value: Vec::new()};
    for v in data {
        if v.name.ends_with(target_ext) {
            res.name = v.name.to_string();
            res.value = v.value.to_vec();
            break;
        }
    }
    res
}

fn set_data_as_url(base_file_data: Vec<u8>, target_file_data: Vec<u8>, target_file_name: String) -> Vec<u8> {
    let base64_string = encode(target_file_data);
    let prefix = match &target_file_name.ends_with("wasm") {
        true => "data:application/wasm;base64,".to_string(),
        _ => "data:text/javascript;base64,".to_string()
    };
    //split string
    let base_file_string = String::from_utf8(base_file_data.clone()).unwrap();
    let splitted_str: Vec<&str> = base_file_string.split(&target_file_name).collect();
    let mut trimmed = splitted_str[0];
    while trimmed.len() > 0 && trimmed[(trimmed.len() - 1)..trimmed.len()].ne("\"") && trimmed[(trimmed.len() - 1)..trimmed.len()].ne("'") {
        trimmed = &trimmed[..(trimmed.len() - 1)];
    }
    format!("{}{}{}{}", trimmed.to_string(), prefix, base64_string, splitted_str[1].to_string()).as_bytes().to_vec()
}

#[wasm_bindgen]
pub fn convert(val: JsValue) {
    let data: Vec<Data> = serde_wasm_bindgen::from_value(val).unwrap();

    let html_data = get_target_data(data.to_vec(), "html");
    let js_data = get_target_data(data.to_vec(), "js");
    let wasm_data = get_target_data(data.to_vec(), "wasm");
    // wasm into js
    let tmp_data = set_data_as_url(js_data.value, wasm_data.value, wasm_data.name);
    // js into html
    let result_data = set_data_as_url(html_data.value, tmp_data, js_data.name);

    download_result_file(result_data, "text/html", "result.html");
}
