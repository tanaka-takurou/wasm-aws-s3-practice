#![deny(warnings)]
#![allow(dead_code)]

use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    title: Option<String>,
    random: Option<bool>,
    questions: Vec<QuestionsConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
struct QuestionsConfig {
    question: String,
    explanation: String,
    answers: Vec<u32>,
    choices: Vec<String>,
}

enum State {
    Init,
    Converted,
}

const DEFAULT_TXT_STR: &str = r#"Sample question 1. 1 is correct.

Sample explanation text. Question 1.

1

Alpha
Beta
Chi

Sample question 2. 1 and 2 are correct.

Sample explanation text. Question 2.

1,2

One
Two
Three

Sample question 3. 1 to 3 are correct.

Sample explanation text. Question 3.

1,2,3

Single
Double
Triple

"#;

fn get_document() -> web_sys::Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}

fn set_body(state: State) {
    let document = get_document();
    match state {
        State::Init => {
            let main_elem = match document.get_element_by_id("main") {
                Some(v) => v,
                None => {
                    let main_elem = document.create_element("div").unwrap();
                    let _ = main_elem.set_attribute("id", "main");
                    document.body().unwrap().append_child(&main_elem).unwrap();
                    main_elem
                }
            };
            main_elem.set_inner_html("");
            {
                let h1_elem = document.create_element("h1").unwrap();
                h1_elem.set_text_content(Some("Text to Toml Example"));
                main_elem.append_child(&h1_elem).unwrap();
            }
            {
                let div_elem = document.create_element("div").unwrap();
                let _ = div_elem.set_attribute("class", "container");
                let textarea_elem = document.create_element("textarea").unwrap();
                let _ = textarea_elem.set_attribute("id", "data");
                div_elem.append_child(&textarea_elem).unwrap();
                main_elem.append_child(&div_elem).unwrap();
            }
            {
                let div_elem = document.create_element("div").unwrap();
                let _ = div_elem.set_attribute("class", "field");
                let span_elem = document.create_element("span").unwrap();
                let _ = span_elem.set_attribute("id", "message");
                div_elem.append_child(&span_elem).unwrap();
                main_elem.append_child(&div_elem).unwrap();
            }
            {
                let div_elem = document.create_element("div").unwrap();
                let _ = div_elem.set_attribute("id", "file_container");
                let _ = div_elem.set_attribute("class", "field");
                let label_elem = document.create_element("label").unwrap();
                label_elem.set_text_content(Some("Load File"));
                let div_elem2 = document.create_element("div").unwrap();
                let input_elem = document.create_element("input").unwrap();
                let _ = input_elem.set_attribute("id", "file");
                let _ = input_elem.set_attribute("type", "file");
                let _ = input_elem.set_attribute("name", "file");
                let _ = input_elem.set_attribute("accept", ".toml,.txt");
                let _ = input_elem.set_attribute("onchange", "ChangeFile();");
                div_elem2.append_child(&input_elem).unwrap();
                div_elem.append_child(&label_elem).unwrap();
                div_elem.append_child(&div_elem2).unwrap();
                main_elem.append_child(&div_elem).unwrap();
            }
            {
                let button_container_elem = document.create_element("div").unwrap();
                let _ = button_container_elem.set_attribute("id", "button_container");
                let _ = button_container_elem.set_attribute("class", "field");
                let button_elem = document.create_element("button").unwrap();
                button_elem.set_text_content(Some("Convert"));
                let _ = button_elem.set_attribute("id", "convert");
                let _ = button_elem.set_attribute("onclick", "Convert();");
                button_container_elem.append_child(&button_elem).unwrap();
                main_elem.append_child(&button_container_elem).unwrap();
            }
        },
        State::Converted => {
            let data_elem = document.get_element_by_id("data").expect("no data_element");
            let _ = data_elem.set_attribute("readonly", "");
            let file_container_elem = document.get_element_by_id("file_container").expect("no file_container_element");
            file_container_elem.set_inner_html("");
            let button_container_elem = document.get_element_by_id("button_container").expect("no button_container_element");
            button_container_elem.set_inner_html("");
            {
                let button_elem = document.create_element("button").unwrap();
                button_elem.set_text_content(Some("Reset"));
                let _ = button_elem.set_attribute("id", "reset");
                let _ = button_elem.set_attribute("onclick", "Reset();");
                button_container_elem.append_child(&button_elem).unwrap();
            }
        }
    };
}

#[wasm_bindgen(start)]
pub fn start() {
    set_body(State::Init);
    let document = get_document();
    let tmp_elem = document.get_element_by_id("data").expect("no data_element");
    let data_elem = tmp_elem.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
    data_elem.set_value(DEFAULT_TXT_STR);
}

fn get_toml_from_txt(s: String) -> String {
    let source_vec: Vec<&str> = s.split('\n').collect();

    let mut result_data = Vec::new();
    let mut counter = 0;
    for i in 0..source_vec.len() {
        if !source_vec.to_vec()[i].is_empty() {
            match counter {
                0 => {
                    result_data.extend(&format!("[[questions]]\nquestion = \"{}\"\n", source_vec[i]).as_bytes().to_vec());
                },
                1 => {
                    result_data.extend(&format!("explanation = \"{}\"\n", source_vec[i]).as_bytes().to_vec());
                },
                2 => {
                    result_data.extend(&format!("answers = [{}]\n", source_vec[i]).as_bytes().to_vec());
                },
                3 => {
                    result_data.extend(&format!("choices = [\n\"{}\"", source_vec[i]).as_bytes().to_vec());
                    counter += 1;
                },
                _ => {
                    result_data.extend(&format!(",\n\"{}\"", source_vec[i]).as_bytes().to_vec());
                },
            };
        } else {
            counter += 1;
            if counter > 3 {
                result_data.extend("\n]\n\n".as_bytes().to_vec());
                counter = 0;
            }
        }
    }
    if result_data.len() > 0 {
        while &result_data[(result_data.len() - 1)..result_data.len()] == "\n".as_bytes() {
            result_data = result_data[..(result_data.len() - 1)].to_vec();
        }
        if &result_data[(result_data.len() - 1)..result_data.len()] != "]".as_bytes() {
            result_data.extend("\n]".as_bytes().to_vec());
        }
    }
    String::from_utf8(result_data).unwrap_or(String::new())
}

#[wasm_bindgen]
pub fn convert() {
    let document = get_document();
    let tmp_elem = document.get_element_by_id("data").expect("no data_element");
    let data_elem = tmp_elem.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
    let message_elem = document.get_element_by_id("message").expect("no message_element");
    let tmp_toml = get_toml_from_txt(data_elem.value().to_string());
    let config: Config = toml::from_str(tmp_toml.to_string().as_ref()).unwrap_or(
        Config{
            title: Some(String::new()),
            random: Some(false),
            questions: vec![],
        }
    );
    if config.questions.len() > 0 {
        let json_string = serde_json::to_string(&config).unwrap_or(String::new());
        if json_string.is_empty() {
            message_elem.set_text_content(Some(&"Syntax Error."));
        } else {
            message_elem.set_text_content(Some(&"Syntax OK."));
            data_elem.set_value(tmp_toml.as_ref());
            set_body(State::Converted);
        }
    } else {
        message_elem.set_text_content(Some(&"Syntax Error."));
    }
}
