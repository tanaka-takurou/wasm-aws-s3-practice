#![deny(warnings)]
#![allow(dead_code)]

use html_escape::encode_text;
use js_sys::JSON;
use js_sys::Object;
use rand::prelude::*;
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
    Ready,
    Main,
    Finish,
}

const DEFAULT_TOML_STR: &str = r#"title = "sample"
random = false

[[questions]]
question = "Sample question 1. 1 is correct."
explanation = "Sample explanation text. Question 1."
answers = [1]
choices = [
"Alpha",
"Beta",
"Chi"
]

[[questions]]
question = "Sample question 2. 1 and 2 are correct."
explanation = "Sample explanation text. Question 2."
answers = [1, 2]
choices = [
"One",
"Two",
"Three"
]

[[questions]]
question = "Sample question 3. 1 to 3 are correct."
explanation = "Sample explanation text. Question 3."
answers = [1, 2, 3]
choices = [
"Single",
"Double",
"Triple"
]
"#;

fn get_document() -> web_sys::Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}

fn set_body(state: State, config_option: Option<Config>, answer_log_option: Option<Vec<Vec<u32>>>) {
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
                h1_elem.set_text_content(Some("Toml Loader Example"));
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
                button_elem.set_text_content(Some("Decode"));
                let _ = button_elem.set_attribute("id", "decode");
                let _ = button_elem.set_attribute("onclick", "Decode();");
                button_container_elem.append_child(&button_elem).unwrap();
                main_elem.append_child(&button_container_elem).unwrap();
            }
        },
        State::Ready => {
            let data_elem = document.get_element_by_id("data").expect("no data_element");
            let _ = data_elem.set_attribute("readonly", "");
            let file_container_elem = document.get_element_by_id("file_container").expect("no file_container_element");
            file_container_elem.set_inner_html("");
            let button_container_elem = document.get_element_by_id("button_container").expect("no button_container_element");
            button_container_elem.set_inner_html("");
            {
                let button_elem = document.create_element("button").unwrap();
                button_elem.set_text_content(Some("Start Choice Question"));
                let _ = button_elem.set_attribute("id", "next");
                let _ = button_elem.set_attribute("onclick", "StartChoiceQuestion();");
                button_container_elem.append_child(&button_elem).unwrap();
            }
            {
                let span_elem = document.create_element("span").unwrap();
                span_elem.set_inner_html("&nbsp;");
                button_container_elem.append_child(&span_elem).unwrap();
            }
            {
                let button_elem = document.create_element("button").unwrap();
                button_elem.set_text_content(Some("Reset"));
                let _ = button_elem.set_attribute("id", "reset");
                let _ = button_elem.set_attribute("onclick", "Reset();");
                button_container_elem.append_child(&button_elem).unwrap();
            }
        },
        State::Main => {
            let config = config_option.unwrap();
            let answer_log = answer_log_option.unwrap();
            let index = match answer_log.len() {
                1 => {
                    if answer_log[0][0] == 0 {
                        0
                    } else {
                        1
                    }
                },
                _ => answer_log.len(),
            };
            let main_elem = document.get_element_by_id("main").expect("no main_element");
            main_elem.set_inner_html("");
            let tmp_question = &config.questions[index];
            {
                let h2_elem = document.create_element("h2").unwrap();
                h2_elem.set_text_content(Some(tmp_question.question.as_ref()));
                main_elem.append_child(&h2_elem).unwrap();
            }
            let type_str = match tmp_question.answers.len() == 1 {
                true => "radio",
                 _ => "checkbox",
            };
            {
                let div_elem = document.create_element("div").unwrap();
                let _ = div_elem.set_attribute("class", "container");
                let form_elem = document.create_element("form").unwrap();
                let _ = form_elem.set_attribute("id", "choice_form");
                let fieldset_elem = document.create_element("fieldset").unwrap();
                let _ = fieldset_elem.set_attribute("id", "choices_container");

                let tmp_vec: Vec<usize> = match config.random.as_ref().unwrap() {
                    true => {
                        let mut rng = rand::thread_rng();
                        let mut nums: Vec<usize> = (0..tmp_question.choices.len()).collect();
                        nums.shuffle(&mut rng);
                        nums
                    },
                    _ => {
                        (0..tmp_question.choices.len()).collect()
                    }
                };
                for i in tmp_vec {
                    let div_elem2 = document.create_element("div").unwrap();
                    let input_elem = document.create_element("input").unwrap();
                    let _ = input_elem.set_attribute("id", &format!("choice{:?}", i));
                    let _ = input_elem.set_attribute("type", type_str);
                    let _ = input_elem.set_attribute("name", "choices");
                    let _ = input_elem.set_attribute("value", &(i + 1).to_string());
                    let label_elem = document.create_element("label").unwrap();
                    let _ = label_elem.set_attribute("for", &format!("choice{:?}", i));
                    label_elem.set_text_content(Some(&format!("{}", tmp_question.choices[i])));
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
            let config = config_option.unwrap();
            let answer_log = answer_log_option.unwrap();
            let mut correction_counter = 0;
            let mut correction_vec: Vec<bool> = vec![];
            for i in 0..config.questions.len() {
                let correction = check_correction(config.questions[i].answers.to_vec(), answer_log[i].to_vec());
                if correction {
                    correction_counter += 1;
                }
                correction_vec.push(correction);
            }

            let main_elem = document.get_element_by_id("main").expect("no main-element");
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
                    (100f64 *correction_counter as f64 / config.questions.len() as f64).round() as u64,
                    correction_counter,
                    config.questions.len()
                )));
                let _ = span_elem.set_attribute("class", "result_score");
                result_score_container_elem.append_child(&span_elem).unwrap();
                main_elem.append_child(&result_score_container_elem).unwrap();
            }
            let div_elem = document.create_element("div").unwrap();
            let _ = div_elem.set_attribute("class", "container");
            let result_detail_elem = document.create_element("div").unwrap();
            let _ = result_detail_elem.set_attribute("id", "result_detail_container");
            for i in 0..config.questions.len() {
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
                    span_elem.set_text_content(Some(&format!("Q : {}", config.questions[i].question)));
                    div_elem2.append_child(&span_elem).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                {
                    let div_elem2 = document.create_element("div").unwrap();
                    let span_elem = document.create_element("span").unwrap();
                    span_elem.set_text_content(Some(&format!("A : {}", u32_vec_to_choice_string(config.questions[i].answers.to_vec(), config.questions[i].choices.to_vec()))));
                    div_elem2.append_child(&span_elem).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                {
                    let div_elem2 = document.create_element("div").unwrap();
                    let span_elem = document.create_element("span").unwrap();
                    span_elem.set_text_content(Some(&format!("choiced : {}", u32_vec_to_choice_string(answer_log[i].to_vec(), config.questions[i].choices.to_vec()))));
                    let _ = span_elem.set_attribute("class", "result_choiced");
                    div_elem2.append_child(&span_elem).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                {
                    let div_elem2 = document.create_element("div").unwrap();
                    let span_elem = document.create_element("span").unwrap();
                    span_elem.set_text_content(Some(config.questions[i].explanation.as_ref()));
                    div_elem2.append_child(&span_elem).unwrap();
                    result_detail_elem.append_child(&div_elem2).unwrap();
                }
                if i < config.questions.len() - 1 {
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
            {
                let span_elem = document.create_element("span").unwrap();
                span_elem.set_inner_html("&nbsp;");
                button_container_elem.append_child(&span_elem).unwrap();
            }
            {
                let button_elem = document.create_element("button").unwrap();
                button_elem.set_text_content(Some("Reset"));
                let _ = button_elem.set_attribute("id", "reset");
                let _ = button_elem.set_attribute("onclick", "Reset();");
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

fn check_json(s: &str) {
    let document = get_document();
    let tmp_elem = document.get_element_by_id("data").expect("no data_element");
    let data_elem = tmp_elem.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
    let message_elem = document.get_element_by_id("message").expect("no message_element");
    if s.is_empty() {
        message_elem.set_text_content(Some(&"No Data."));
    } else {
        let json = JSON::parse(s).unwrap_or(JsValue::NULL);
        match Object::try_from(&json) {
            None => {
                message_elem.set_text_content(Some(&"Syntax Error."));
            },
            Some(v) => {
                if Object::keys(v).length() == 0 {
                    message_elem.set_text_content(Some(&"Syntax Error."));
                } else {
                    match JSON::stringify_with_replacer_and_space(
                            &json,
                            &JsValue::NULL,
                            &JsValue::from_str("\t"),
                        ) {
                        Err(_) => {
                            message_elem.set_text_content(Some(&"Syntax Error."));
                        },
                        Ok(x) => {
                            match x.as_string() {
                                None => {
                                    message_elem.set_text_content(Some(&"Syntax Error."));
                                },
                                Some(y) => {
                                    message_elem.set_text_content(Some(&"Syntax OK."));
                                    data_elem.set_value(&y);
                                    set_body(State::Ready, None, None);
                                },
                            };
                        },
                    };
                }
            },
        };
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    set_body(State::Init, None, None);
    let document = get_document();
    let tmp_elem = document.get_element_by_id("data").expect("no data_element");
    let data_elem = tmp_elem.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
    data_elem.set_value(encode_text(DEFAULT_TOML_STR).to_string().as_ref());
}

#[wasm_bindgen]
pub fn decode() {
    let document = get_document();
    let tmp_elem = document.get_element_by_id("data").expect("no data_element");
    let data_elem = tmp_elem.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
    let message_elem = document.get_element_by_id("message").expect("no message_element");
    let config: Config = toml::from_str(data_elem.value().to_string().as_ref()).unwrap_or(
        Config{
            title: Some(String::new()),
            random: Some(false),
            questions: vec![],
        }
    );
    if config.questions.len() > 0 {
        let json_string = match config.random.as_ref().unwrap() {
            true => {
                let mut rng = rand::thread_rng();
                let mut new_questions: Vec<QuestionsConfig> = config.questions;
                new_questions.shuffle(&mut rng);
                let new_config: Config = Config{
                    title: config.title,
                    random: Some(true),
                    questions: new_questions,
                };
                serde_json::to_string(&new_config).unwrap_or(String::new())
            },
            _ => {
                serde_json::to_string(&config).unwrap_or(String::new())
            }
        };
        if json_string.is_empty() {
            message_elem.set_text_content(Some(&"Syntax Error."));
        } else {
            check_json(&json_string);
        }
    } else {
        message_elem.set_text_content(Some(&"Syntax Error."));
    }
}

#[wasm_bindgen]
pub fn next(s: &str, data: &str) {
    let answer_log: Vec<Vec<u32>> = serde_json::from_str(s).unwrap_or(vec![vec![0]]);
    let config: Config = serde_json::from_str(data).unwrap_or(
        Config{
            title: Some(String::new()),
            random: Some(false),
            questions: vec![],
        }
    );
    if config.questions.len() > 0 {
        if answer_log.len() < config.questions.len() {
            set_body(State::Main, Some(config), Some(answer_log));
        } else {
            set_body(State::Finish, Some(config), Some(answer_log));
        }
    } else {
        web_sys::console::log_1(&"Json Error".to_string().into());
    }
}
