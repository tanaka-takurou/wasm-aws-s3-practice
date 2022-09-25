use image::{GenericImageView, ImageBuffer,
    ImageFormat, ImageOutputFormat, Rgba};
use image::io::Reader as ImageReader;
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
pub fn restoration(data: Vec<u8>) {
    let img_data = ImageReader::with_format(Cursor::new(data), ImageFormat::Png).decode().unwrap();
    let (width, height) = img_data.dimensions();
    let mut res_bytes: Vec<u8> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = img_data.get_pixel(x, y);

            let red = pixel[0];
            let green = pixel[1];
            let blue = pixel[2];
            let alpha = pixel[3];
            if alpha > 0 {
                res_bytes.push(red);
                if alpha > 1 {
                    res_bytes.push(green);
                    if alpha > 2 {
                        res_bytes.push(blue);
                    }
                }
            } else {
                break;
            }
        }
    }
    download_result_file(res_bytes, "text/plain", "restoration.txt");
}

#[wasm_bindgen]
pub fn convert(data: Vec<u8>) {
    let data_length = data.len();
    let width = (((data_length as f64)/3.0).sqrt().ceil()) as u32;
    let scalex = 3.0 / width as f32;

    let mut res_bytes: Vec<u8> = Vec::new();
    let mut img_buf = ImageBuffer::new(width, width);
    for y in 0..width {
        for x in 0..width {
            let pixel = img_buf.get_pixel_mut(x, y);
            if (3 * (x + width * y)) >= data_length as u32 {
                *pixel = Rgba([0, 0, 0, 0]);
            } else {
                let mut a = 1u8;
                let r = data[(3 * (x + width * y)) as usize];
                let g = match (3 * (x + width * y) + 1) as usize >= data_length {
                    true => 0u8,
                    false => {
                        a = 2u8;
                        data[(3 * (x + width * y) + 1) as usize]
                    },
                };
                let b = match (3 * (x + width * y) + 2) as usize >= data_length {
                    true => 0u8,
                    false => {
                        a = 3u8;
                        data[(3 * (x + width * y) + 2) as usize]
                    },
                };
                if a > 2u8 {
                    let cx = y as f32 * scalex - 1.5;
                    let cy = x as f32 * scalex - 1.5;
                    let mut z = num_complex::Complex::new(cx, cy);

                    let dx = (data_length % 1000) as f32 * -0.0001 - 0.72;
                    let dy = match (data_length % 100) < 50 {
                        true => (data_length % 100) as f32 * 0.002 - 0.13,
                        false => (data_length % 100) as f32 * 0.002 - 0.05,
                    };
                    let c = num_complex::Complex::new(dx, dy);

                    while a < 255 && z.norm() <= 2.0 {
                        z = z * z + c;
                        a += 1;
                    }
                }
                *pixel = Rgba([r, g, b, a]);
            }
        }
    }
    img_buf.write_to(&mut Cursor::new(&mut res_bytes), ImageOutputFormat::Png).unwrap();
    download_result_file(res_bytes, "image/png", "result.png");
}
