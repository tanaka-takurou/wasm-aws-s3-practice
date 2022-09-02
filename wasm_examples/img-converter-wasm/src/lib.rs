extern crate base64;

use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::{ImageFormat, ImageOutputFormat};
use base64::decode;
use wasm_bindgen::prelude::*;

fn split_base64_text(base64_text: &str) -> (String, String) {
    let split_text: Vec<&str> = base64_text.split(',').collect();
    let mime_type =
        match split_text.get(0) {
            Some(v) => {
                let tmp: Vec<&str> = v.split(':').collect();
                let tmp2: Vec<&str> = tmp.get(1).unwrap().split(';').collect();
                tmp2.get(0).unwrap().to_string()
            },
            None    => "text/plain".to_string(),
        }
    ;
    let base64_data =
        match split_text.get(1) {
            Some(v) => (*v).to_string(),
            None    => "".to_string(),
        }
    ;
    (base64_data, mime_type)
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
pub fn convert(source_text: &str, result_extension: &str) {
    let (base64_data, mime_type) = split_base64_text(source_text);

    let (output_format, output_mime_type) =
        match result_extension.trim() {
            "gif" => (ImageOutputFormat::Gif, "image/gif"),
            "jpeg" => (ImageOutputFormat::Jpeg(255u8), "image/jpeg"),
            "png" => (ImageOutputFormat::Png, "image/png"),
            _ => (ImageOutputFormat::Png, "image/png"),
        }
    ;
    let image_bytes = decode(base64_data).unwrap();
    let img = ImageReader::with_format(Cursor::new(image_bytes), ImageFormat::from_mime_type(mime_type).unwrap()).decode().unwrap();
    let mut converted_bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut converted_bytes), output_format).unwrap();

    download_result_file(converted_bytes, output_mime_type, &format!("result.{}", result_extension.trim()));
}
