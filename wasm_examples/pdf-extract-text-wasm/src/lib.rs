use lopdf::*;
use pdf_extract::*;
use std::io::Cursor;
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

#[wasm_bindgen]
pub fn extract(data: Vec<u8>) {
    let doc = Document::load_mem(&data[..]).expect("Error: extract text");
    // let pdf_string = extract_text_from_mem()
    let mut pdf_txt_vec: Vec<u8> = Vec::new();
    let _ = output_doc(&doc, (Box::new(PlainTextOutput::new(&mut Cursor::new(&mut pdf_txt_vec) as &mut dyn std::io::Write)) as Box<dyn OutputDev>).as_mut());
    download_result_file(pdf_txt_vec, "text/plain", "extracted.txt");
}
