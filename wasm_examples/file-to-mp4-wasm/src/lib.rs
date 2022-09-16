use bytes::{BufMut, BytesMut};
use std::io::Cursor;
use mp4::{AvcConfig, Bytes,
    Mp4Config, Mp4Reader, Mp4Sample,
    Mp4Writer, TrackConfig};
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

fn data_from_mp4(data: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let tmp = Cursor::new(data.to_vec());
    let mut mp4 = Mp4Reader::read_header(tmp, data.len() as u64).unwrap();
    match mp4.read_sample(1u32, 1u32) {
        Ok(v) => {
            match v {
                Some(w) => {
                    res = w.bytes.as_ref().to_vec();
                },
                None => web_sys::console::log_1(&"Error: Read Sample".into()),
            };
        },
        Err(_) => web_sys::console::log_1(&"Error: Read Sample".into()),
    };
    res
}

fn data_into_mp4(data: Vec<u8>) -> Vec<u8> {
    let mut buf = BytesMut::new();
    buf.put_slice(&data[..]);

    let config = Mp4Config {
        major_brand: str::parse("isom").unwrap(),
        minor_version: 512,
        compatible_brands: vec![
            str::parse("isom").unwrap(),
            str::parse("iso2").unwrap(),
            str::parse("avc1").unwrap(),
            str::parse("mp41").unwrap(),
        ],
        timescale: 1000,
    };

    let tmp = Cursor::new(Vec::<u8>::new());
    let mut writer = Mp4Writer::write_start(tmp, &config).unwrap();
    let avc_config = AvcConfig {
        width: 1920,
        height: 1080,
        seq_param_set: vec![103, 100, 0, 40, 172, 217, 64, 120, 2, 39, 229, 132, 0, 0, 3, 0, 4, 0, 0, 3, 0, 8, 60, 96, 198, 88],
        pic_param_set: vec![104, 235, 227, 203, 34, 192],
    };
    writer.add_track(&TrackConfig::from(avc_config)).unwrap();
    let mp4_sample = Mp4Sample {
        start_time: 0u64,
        duration: 1000u32,
        rendering_offset: 0i32,
        is_sync: true,
        bytes: Bytes::from(buf),
    };
    writer.write_sample(1, &mp4_sample).unwrap();
    writer.write_end().unwrap();
    writer.into_writer().into_inner()
}

#[wasm_bindgen]
pub fn restoration(data: Vec<u8>) {
    let res: Vec<u8> = data_from_mp4(data);
    download_result_file(res, "text/plain", "restoration.txt");
}

#[wasm_bindgen]
pub fn convert(data: Vec<u8>) {
    let res: Vec<u8> = data_into_mp4(data);
    download_result_file(res, "video/mp4", "converted.mp4");
}
