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

fn bytes_lowering(v: Vec<u8>) -> Vec<u8> {
    let mut buf = Vec::new();
    for i in 0..v.len() {
        if v[i] == 0u8 {
            continue;
        } else {
            buf.extend(&v[i..v.len()].to_vec());
            break;
        }
    }
    if buf.len() == 0 {
        buf.push(0u8);
    }
    buf
}

fn convert_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn positive_sinusoid(x: f64) -> f64 {
    0.5f64 * x.sin() + 0.5f64
}

fn sin_fit(v: Vec<u8>, counter: usize, current_length: usize, current_baseline: usize, current_term: usize, current_offset: usize) ->
          (usize, usize, usize, usize, usize) {
    let mut max_length = current_length;
    let mut min_baseline = current_baseline;
    let mut min_term = current_term;
    let mut min_offset = current_offset;
    let next_counter: usize;
    if counter < 2 * v.len() {
        let frequency = 1f64 / (counter + 1) as f64;
        for j in 0..u8::MAX as usize {
            for k in 0..u8::MAX as usize {
                let mut idx = 0;
                let mut tmp_length = 0;
                loop {
                    let idx_limit = match idx + 4 >= v.len() {
                        true => v.len(),
                        _    => idx + 4,
                    };
                    for l in idx..idx_limit {
                        let rad = (j + l) as f64 * std::f64::consts::PI * frequency + std::f64::consts::FRAC_PI_2;
                        if v[l] != (k as u8).wrapping_add((positive_sinusoid(rad) * u8::MAX as f64).trunc() as u8) {
                            tmp_length += l - idx;
                            break;
                        } else if l == idx_limit - 1 {
                            tmp_length += l + 1 - idx;
                            break;
                        }
                    }
                    if idx + 4 >= v.len() {
                        if max_length < tmp_length {
                            max_length = tmp_length;
                            min_baseline = k;
                            min_term = counter;
                            min_offset = j;
                        }
                        break;
                    }
                    idx += 4;
                }
                if max_length == v.len() - 1 {
                    break;
                }
            }
            if max_length == v.len() - 1 {
                break;
            }
        }
    }
    if max_length == v.len() - 1 {
        next_counter = 2 * v.len();
    } else {
        next_counter = counter + 1;
    }
    (next_counter, max_length, min_baseline, min_term, min_offset)
}

fn convert_main(v: Vec<u8>, baseline: usize, term: usize, offset: usize) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let remainder = match v.len() % 4 {
        0 => Vec::new(),
        _ => v[(v.len() - (v.len() % 4))..].to_vec(),
    };
    buf.push(remainder.len() as u8);
    if remainder.len() > 0 {
        buf.extend(remainder.to_vec());
    }
    let mut v_fitted: Vec<u8> = Vec::new();
    let min_frequency = 1f64 / (term + 1) as f64;
    for i in 0..v.len() {
        let rad = (offset as usize + i) as f64 * std::f64::consts::PI * min_frequency + std::f64::consts::FRAC_PI_2;
        v_fitted.push(v[i].wrapping_sub((baseline as u8).wrapping_add((positive_sinusoid(rad) * u8::MAX as f64).trunc() as u8)));
    }
    buf.extend(vec![baseline as u8, offset as u8]);
    buf.extend((term as u64).to_be_bytes().to_vec());

    let mut counter = 0;
    let mut tmp_param: u8 = 0;
    let mut tmp_vec: Vec<u8> = Vec::new();
    let mut idx = 0;
    loop {
        if idx + 4 > v_fitted.len() {
            break;
        }
        let tmp = bytes_lowering(v_fitted[idx..idx + 4].to_vec());
        tmp_param += (4f64.powf(counter as f64) * (tmp.len() - 1) as f64).trunc() as u8;
        tmp_vec.extend(tmp);
        counter += 1;
        if counter == 4 || idx + 8 > v_fitted.len() {
            buf.push(tmp_param);
            buf.extend(tmp_vec);
            tmp_param = 0;
            tmp_vec = Vec::new();
            counter = 0;
        }
        idx += 4;
    }
    buf
}

fn show_progress(counter: usize, max: usize) {
    web_sys::console::log_1(&format!("{:?} / {:?}", counter, max).into());
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let target_elem = document.get_element_by_id("target").unwrap();
    target_elem.set_text_content(Some(&format!("{:?}%", (100f64 * counter as f64) / max as f64)));
}

#[wasm_bindgen]
pub fn restoration(data: Vec<u8>) {
    let mut buf: Vec<u8> = Vec::new();
    let remainder_len = data[0] as usize;
    let baseline = data[remainder_len + 1];
    let offset = data[remainder_len + 2];
    let u8_array: [u8; 8] = convert_to_array(data[(remainder_len + 3)..(remainder_len + 11)].to_vec());
    let term = u64::from_be_bytes(u8_array);

    let mut counter = 1;
    let mut tmp_vec: Vec<u8> = Vec::new();
    let mut tmp_param;
    let mut idx = 11 + remainder_len;
    loop {
        if idx >= data.len() {
            break;
        }
        for i in 0..4 {
            tmp_param = ((data[idx] / (4f64.powf(i as f64) as u8)) % 4) as usize + 1;
            if tmp_param < 4 {
                tmp_vec.extend(vec![0; 4 - tmp_param]);
            }
            tmp_vec.extend(data[(idx + counter)..(idx + counter + tmp_param)].to_vec());
            counter += tmp_param;
            if idx + counter >= data.len() {
                break;
            }
        }
        idx += counter;
        counter = 1;
    }

    let frequency = 1f64 / (term + 1) as f64;
    for i in 0..tmp_vec.len() {
        let rad = (offset as usize + i) as f64 * std::f64::consts::PI * frequency + std::f64::consts::FRAC_PI_2;
        buf.push(tmp_vec[i].wrapping_add(baseline.wrapping_add((positive_sinusoid(rad) * u8::MAX as f64).trunc() as u8)));
    }

    if remainder_len > 0 {
        buf.extend(data[1..(1 + remainder_len)].to_vec());
    }

    download_result_file(buf, "text/plain", "restoration.txt");
}

#[wasm_bindgen]
pub fn convert(data: Vec<u8>, s: &str) -> String {
    let params: Vec<&str> = s.split('_').collect();
    let mut counter: usize = params[0].parse().unwrap();
    let mut max_length: usize = params[1].parse().unwrap();
    let mut min_baseline: usize = params[2].parse().unwrap();
    let mut min_term: usize = params[3].parse().unwrap();
    let mut min_offset: usize = params[4].parse().unwrap();
    let mut finish_flag: u8 = 0;

    if counter < 2 * data.len() {
        (counter, max_length, min_baseline, min_term, min_offset) = sin_fit(data.to_vec(), counter, max_length, min_baseline, min_term, min_offset);
        show_progress(counter, 2 * data.len());
    }
    if counter >= 2 * data.len() {
        finish_flag = 1;
        download_result_file(convert_main(data, min_baseline, min_term, min_offset), "text/plain", "converted.txt");
    }
    format!("{}_{}_{}_{}_{}_{}", counter, max_length, min_baseline, min_term, min_offset, finish_flag).to_string()
}
