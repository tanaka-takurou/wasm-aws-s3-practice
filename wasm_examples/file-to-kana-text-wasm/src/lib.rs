use base64::{encode, decode};
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

fn int_to_kana(i: u8) -> String {
    let hex_e3 = 227u8;
    let u8max_plus1 = (u8::MAX as u16) + 1u16;
    let j: u16;
    if i < 5 {
        j = 2 * i as u16 + 33154u16;
    } else if i < 17 {
        j = 2 * i as u16 + 33153u16;
    } else if i < 20 {
        j = 2 * i as u16 + 33154u16;
    } else if i < 25 {
        j = i as u16 + 33174u16;
    } else if i < 30 {
        j = 3 * i as u16 + 33124u16;
    } else if i < 32 {
        j = i as u16 + 33184u16;
    } else if i < 35 {
        j = i as u16 + 33376u16;
    } else if i < 38 {
        j = 2 * i as u16 + 33342u16;
    } else if i < 43 {
        j = i as u16 + 33379u16;
    } else if i < 55 {
        j = 2 * i as u16 + 33078u16;
    } else if i < 58 {
        j = 2 * i as u16 + 33079u16;
    } else if i < 62 {
        j = 3 * i as u16 + 33026u16;
    } else if i == 62 {
        j = 33423u16;
    } else if i == 63 {
        j = 33426u16;
    } else {
        j = 33427u16;
    }
    std::string::String::from_utf8([hex_e3, (j / u8max_plus1) as u8, (j % u8max_plus1) as u8].to_vec()).unwrap().to_string()
}

fn base64_to_int(i: u8) -> u8 {
    let k: u8;
    if i == 43u8 {
        k = 62u8;
    } else if i == 45u8 {
        k = 62u8;
    } else if i == 47u8 {
        k = 63u8;
    } else if i < 58u8 {
        k = i - 48u8;
    } else if i == 61u8 {
        k = 64u8;
    } else if i < 91u8 {
        k = i - 55u8;
    } else if i == 95u8 {
        k = 63u8;
    } else if i < 123 {
        k = i - 61u8;
    } else {
        k = 64u8;
    }
    k
}

fn kana_to_int(kana_string: String) -> u8 {
    let kana_vec: Vec<u8> = kana_string.as_str().as_bytes().to_vec();
    let hex_e3 = 227u8;
    let mut result = 0u8;
    if kana_vec[0] == hex_e3 {
        if kana_vec[1] == 129u8 {
            if kana_vec[2] < 139u8 {
                result = (kana_vec[2] as f64 / 2f64) as u8 - 65u8;
            } else if kana_vec[2] < 163u8 {
                if kana_vec[2] as f64 % 2f64 == 1f64 {
                    result = (kana_vec[2] as f64 / 2f64).ceil() as u8 - 65u8;
                } else {
                    result = (kana_vec[2] as f64 / 2f64) as u8 - 27u8;
                }
            } else if kana_vec[2] < 170u8 {
                if kana_vec[2] as f64 % 2f64 == 1f64 {
                    result = (kana_vec[2] as f64 / 2f64).ceil() as u8 - 28u8;
                } else {
                    result = (kana_vec[2] as f64 / 2f64) as u8 - 65u8;
                }
            } else if kana_vec[2] < 175u8 {
                result = kana_vec[2] - 150u8;
            } else if kana_vec[2] < 190u8 {
                if kana_vec[2] as f64 % 3f64 == 1f64 {
                    result = (kana_vec[2] as f64 / 3f64).ceil() as u8 - 34u8;
                } else {
                    result = (kana_vec[2] as f64 / 3f64).ceil() as u8 - 1u8;
                }
            } else if kana_vec[2] < 192u8 {
                result = kana_vec[2] - 160u8;
            } else {
                result = kana_vec[2] - 128u8;
            }
        } else {
            if kana_vec[2] < 131u8 {
                result = kana_vec[2] - 96u8;
            } else if kana_vec[2] < 137u8 {
                result = (kana_vec[2] as f64 / 2f64).ceil() as u8 - 31u8;
            } else if kana_vec[2] < 143u8 {
                result = kana_vec[2] - 99u8;
            } else if kana_vec[2] < 144u8 {
                result = 62u8;
            } else if kana_vec[2] < 147u8 {
                result = 63u8;
            } else {
                result = 64u8;
            }
        }
    }
    result
}

fn int_to_base64(i: u8) -> u8 {
    let result: u8;
    if i < 10u8 {
        result = i + 48u8;
    } else if i < 36u8 {
        result = i + 55u8;
    } else if i < 62u8 {
        result = i + 61u8;
    } else if i == 62u8 {
        result = 43u8;
    } else if i == 63u8 {
        result = 47u8;
    } else {
        result = 61u8;
    }
    result
}

#[wasm_bindgen]
pub fn restoration(data: Vec<u8>) {
    let kana_char_vec: Vec<char> = std::str::from_utf8(&data[..]).unwrap().chars().collect();
    let mut res = Vec::new();
    for v in kana_char_vec {
        res.push(int_to_base64(kana_to_int(v.to_string())));
    }
    download_result_file(decode(&res[..]).unwrap(), "text/plain", "restoration.txt");
}

#[wasm_bindgen]
pub fn convert(data: Vec<u8>) {
    let base64_u8_vec: Vec<u8> = encode(data).as_bytes().to_vec();
    let mut res = Vec::new();
    for v in base64_u8_vec {
        res.extend(int_to_kana(base64_to_int(v)).into_bytes().iter().cloned());
    }
    download_result_file(res, "text/plain", "result_base64.txt");
}
