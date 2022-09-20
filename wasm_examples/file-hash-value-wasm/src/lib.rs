use sha2::{Sha256, Sha512, Digest};
use wasm_bindgen::prelude::*;

fn show_hash_value(hash_value: String) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let target_elem = document.get_element_by_id("target").unwrap();
    web_sys::console::log_1(&hash_value.clone().into());
    target_elem.set_text_content(Some(&hash_value));
}

#[wasm_bindgen]
pub fn md5_hash_value(data: Vec<u8>) {
    let digest = md5::compute(&data[..]);
    show_hash_value(format!("{:x}", digest));
}

#[wasm_bindgen]
pub fn sha256_hash_value(data: Vec<u8>) {
    let mut hasher = Sha256::new();
    hasher.update(&data[..]);
    let result = hasher.finalize();
    show_hash_value(format!("{:x}", result));
}

#[wasm_bindgen]
pub fn sha512_hash_value(data: Vec<u8>) {
    let mut hasher = Sha512::new();
    hasher.update(&data[..]);
    let result = hasher.finalize();
    show_hash_value(format!("{:x}", result));
}
