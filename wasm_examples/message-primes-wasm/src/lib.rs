use primes;
use wasm_bindgen::prelude::*;

const MAX_MESSAGE_COUNT: u32 = 256;
struct Data(u64, u64);

fn check(x: u64, data: &mut Vec<Data>, index: &mut u64) {
    if primes::is_prime(x) {
        data.push(Data(*index, x));
        web_sys::console::log_1(&format!("{:?}th prime number is {:?}.", index, x).into());
        *index += 1;
    }
}

#[wasm_bindgen]
pub fn start(s: &str) -> String {
    let params: Vec<&str> = s.split('_').collect();
    let mut counter: u64 = params[0].parse().unwrap();
    let mut index: u64 = params[1].parse().unwrap();
    if counter < u64::MAX {
        // initialize
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let base = document.get_element_by_id("base").unwrap();
        let parent = document.get_element_by_id("parent").unwrap();
        parent.remove();
        let table_elem = document.create_element("table").unwrap();
        let tr_elem = document.create_element("tr").unwrap();
        let th_elem1 = document.create_element("th").unwrap();
        let th_elem2 = document.create_element("th").unwrap();
        table_elem.set_id("parent");
        th_elem1.set_text_content(Some("Index"));
        th_elem2.set_text_content(Some("Prime Number"));
        tr_elem.append_child(&th_elem1).unwrap();
        tr_elem.append_child(&th_elem2).unwrap();
        table_elem.append_child(&tr_elem).unwrap();
        base.append_child(&table_elem).unwrap();

        // main
        let mut data: Vec<Data> = Vec::new();
        if counter < 3 {
            for i in counter..=3 {
                counter = i;
                check(counter, &mut data, &mut index);
            }
        }
        loop {
            counter += 2;
            check(counter, &mut data, &mut index);
            if data.len() == MAX_MESSAGE_COUNT as usize || counter == u64::MAX {
                for i in 0..data.len() {
                    let tr_elem = document.create_element("tr").unwrap();
                    let th_elem = document.create_element("th").unwrap();
                    let td_elem = document.create_element("td").unwrap();
                    th_elem.set_text_content(Some(&format!("{:?}th", data[i].0)));
                    td_elem.set_text_content(Some(&format!("{:?}", data[i].1)));
                    tr_elem.append_child(&th_elem).unwrap();
                    tr_elem.append_child(&td_elem).unwrap();
                    table_elem.append_child(&tr_elem).unwrap();
                }
                break;
            }
        }
    }
    format!("{}_{}", counter, index).to_string()
}
