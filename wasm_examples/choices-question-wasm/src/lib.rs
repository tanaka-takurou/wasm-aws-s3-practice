#![deny(warnings)]
#![allow(dead_code)]

use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct RequestData {
    order: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseData {
    index: u32,
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
    Main,
    Finish,
}

fn get_document() -> web_sys::Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}

fn set_body(state: State, data_option: Option<ResponseData>, answer_log_option: Option<Vec<Vec<u32>>>) {
    let document = get_document();
    match state {
        State::Main => {
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
            let data = data_option.unwrap();
            let index = data.index;
            let question_config = &data.questions[0];
            {
                let h2_elem = document.create_element("h2").unwrap();
                h2_elem.set_inner_html(question_config.question.as_ref());
                main_elem.append_child(&h2_elem).unwrap();
            }
            let type_str = match question_config.answers.len() == 1 {
                true => "radio",
                 _ => "checkbox",
            };
            {
                let div_elem = document.create_element("div").unwrap();
                let _ = div_elem.set_attribute("class", "container");
                let form_elem = document.create_element("form").unwrap();
                let _ = form_elem.set_attribute("id", "choice_form");
                let _ = form_elem.set_attribute("data-index", &index.to_string());
                let fieldset_elem = document.create_element("fieldset").unwrap();
                let _ = fieldset_elem.set_attribute("id", "choices_container");

                for i in 0..question_config.choices.len() {
                    let div_elem2 = document.create_element("div").unwrap();
                    let input_elem = document.create_element("input").unwrap();
                    let _ = input_elem.set_attribute("id", &format!("choice{:?}", i));
                    let _ = input_elem.set_attribute("type", type_str);
                    let _ = input_elem.set_attribute("name", "choices");
                    let _ = input_elem.set_attribute("value", &(i + 1).to_string());
                    let label_elem = document.create_element("label").unwrap();
                    let _ = label_elem.set_attribute("for", &format!("choice{:?}", i));
                    label_elem.set_text_content(Some(&format!("{}", question_config.choices[i])));
                    div_elem2.append_child(&input_elem).unwrap();
                    div_elem2.append_child(&label_elem).unwrap();
                    fieldset_elem.append_child(&div_elem2).unwrap();
                }
                form_elem.append_child(&fieldset_elem).unwrap();
                div_elem.append_child(&form_elem).unwrap();
                main_elem.append_child(&div_elem).unwrap();
            }
            {
                let button_container_elem = document.create_element("div").unwrap();
                let _ = button_container_elem.set_attribute("id", "button_container");
                let _ = button_container_elem.set_attribute("class", "field");
                let button_elem = document.create_element("button").unwrap();
                button_elem.set_text_content(Some("Next"));
                let _ = button_elem.set_attribute("id", "next");
                let _ = button_elem.set_attribute("onclick", "Next();");
                button_container_elem.append_child(&button_elem).unwrap();
                main_elem.append_child(&button_container_elem).unwrap();
            }
        },
        State::Finish => {
            let config = data_option.unwrap().questions;
            let answer_log = answer_log_option.unwrap();
            let mut correction_counter = 0;
            let mut correction_vec: Vec<bool> = vec![];
            for i in 0..config.len() {
                let correction = check_correction(config[i].answers.to_vec(), answer_log[i].to_vec());
                if correction {
                    correction_counter += 1;
                }
                correction_vec.push(correction);
            }

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
                let h2_elem = document.create_element("h2").unwrap();
                h2_elem.set_text_content(Some("Result"));
                main_elem.append_child(&h2_elem).unwrap();
            }
            {
                let result_score_container_elem = document.create_element("div").unwrap();
                let _ = result_score_container_elem.set_attribute("id", "result_score_container");
                let span_elem = document.create_element("span").unwrap();
                span_elem.set_text_content(Some(&format!("{}%  ( {} / {} )",
                    (100f64 *correction_counter as f64 / config.len() as f64).round() as u64,
                    correction_counter,
                    config.len()
                )));
                let _ = span_elem.set_attribute("class", "result_score");
                result_score_container_elem.append_child(&span_elem).unwrap();
                main_elem.append_child(&result_score_container_elem).unwrap();
            }
            let div_elem = document.create_element("div").unwrap();
            let _ = div_elem.set_attribute("class", "container");
            let result_detail_elem = document.create_element("div").unwrap();
            let _ = result_detail_elem.set_attribute("id", "result_detail_container");
            for i in 0..config.len() {
                let (class, text) = match correction_vec[i] {
                    true => ("result_ok", "&#x2B55;"),
                    _ => ("result_ng", "&#x3A7;"),
                };
                {
                    let div_elem2 = document.create_element("div").unwrap();
                    let span_elem = document.create_element("span").unwrap();
                    span_elem.set_text_content(Some(&format!("Question {} ", (i + 1).to_string())));
                    let span_elem2 = document.create_element("span").unwrap();
                    span_elem2.set_inner_html(text);
                    let _ = span_elem2.set_attribute("class", class);
                    div_elem2.append_child(&span_elem).unwrap();
                    div_elem2.append_child(&span_elem2).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                {
                    let div_elem2 = document.create_element("div").unwrap();
                    let span_elem = document.create_element("span").unwrap();
                    span_elem.set_inner_html(&format!("Q : {}", config[i].question));
                    div_elem2.append_child(&span_elem).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                {
                    let div_elem2 = document.create_element("div").unwrap();
                    let span_elem = document.create_element("span").unwrap();
                    span_elem.set_text_content(Some(&format!("A : {}", u32_vec_to_choice_string(config[i].answers.to_vec(), config[i].choices.to_vec()))));
                    div_elem2.append_child(&span_elem).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                {
                    let div_elem2 = document.create_element("div").unwrap();
                    let span_elem = document.create_element("span").unwrap();
                    span_elem.set_text_content(Some(&format!("choiced : {}", u32_vec_to_choice_string(answer_log[i].to_vec(), config[i].choices.to_vec()))));
                    let _ = span_elem.set_attribute("class", "result_choiced");
                    div_elem2.append_child(&span_elem).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                {
                    let div_elem2 = document.create_element("div").unwrap();
                    let span_elem = document.create_element("span").unwrap();
                    span_elem.set_inner_html(config[i].explanation.as_ref());
                    div_elem2.append_child(&span_elem).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                if i < config.len() - 1 {
                    let hr_elem = document.create_element("hr").unwrap();
                    result_detail_elem.append_child(&hr_elem).unwrap();
                }
            }
            div_elem.append_child(&result_detail_elem).unwrap();
            main_elem.append_child(&div_elem).unwrap();

            let button_container_elem = document.create_element("div").unwrap();
            let _ = button_container_elem.set_attribute("id", "button_container");
            let _ = button_container_elem.set_attribute("class", "field");
            {
                let button_elem = document.create_element("button").unwrap();
                button_elem.set_text_content(Some("Retry"));
                let _ = button_elem.set_attribute("id", "retry");
                let _ = button_elem.set_attribute("onclick", "Retry();");
                button_container_elem.append_child(&button_elem).unwrap();
            }
            main_elem.append_child(&button_container_elem).unwrap();
        }
    };
}

fn check_correction(a: Vec<u32>, b: Vec<u32>) -> bool {
    let mut a_ = a.to_vec();
    a_.sort();
    let mut b_ = b.to_vec();
    b_.sort();
    let mut res = true;
    if a_.len() == b_.len() {
        for i in 0..a_.len() {
            if a_[i] != b_[i] {
                res = false;
                break;
            }
        }
    } else {
        res = false;
    }
    res
}

fn u32_vec_to_choice_string(v: Vec<u32>, choices: Vec<String>) -> String {
    let mut v_ = v.to_vec();
    v_.sort();
    let mut tmp_vec: Vec<String> = vec![];
    for i in v_ {
        if i > 0 && (i as usize) <= choices.len() {
            tmp_vec.push(choices[i as usize - 1].to_string());
        }
    }
    tmp_vec.join(", ").to_string()
}

async fn req(order: Vec<u32>) -> String {
    let request_data = RequestData {
        order: order,
    };
    let response = reqwest::Client::new()
        .post(format!("{}/api", get_document().location().unwrap().origin().unwrap()))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_data)
        .send()
        .await;
    match response {
        Err(e) => {
            web_sys::console::log_1(&format!("Error : status ({:?})", e.status()).into());
            if e.is_redirect() {
                web_sys::console::log_1(&"Redirect error".into());
            }
            if e.is_builder() {
                web_sys::console::log_1(&"Builder error".into());
            }
            if e.is_timeout() {
                web_sys::console::log_1(&"Timeout error".into());
            }
            if e.is_request() {
                web_sys::console::log_1(&"Request error".into());
            }
            if e.is_body() {
                web_sys::console::log_1(&"Body error".into());
            }
            if e.is_decode() {
                web_sys::console::log_1(&"Decode error".into());
            }
            String::new()
        },
        Ok(res) => {
            res.text().await.unwrap_or(String::new())
        }
    }
}

#[wasm_bindgen(start)]
pub async fn start() {
    let _ = next("[[]]", "[]").await;
}

#[wasm_bindgen]
pub async fn next(log_str: &str, order_str: &str) {
    let log: Vec<Vec<u32>> = serde_json::from_str(log_str).unwrap_or(vec![vec![]]);
    let order: Vec<u32> = serde_json::from_str(order_str).unwrap_or(vec![]);
    let json_string = req(order.to_vec()).await;
    if json_string.is_empty() {
        web_sys::console::log_1(&"Response Data Empty.".to_string().into());
    } else {
        let res: ResponseData = serde_json::from_str(&json_string).unwrap();
        if res.questions.len() > 0 {
            if res.questions[0].explanation.is_empty() {
                set_body(State::Main, Some(res), Some(log));
            } else {
                set_body(State::Finish, Some(res), Some(log));
            }
        } else {
            web_sys::console::log_1(&"Questions Empty.".to_string().into());
        }
    }
}
