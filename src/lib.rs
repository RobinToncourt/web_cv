#![warn(clippy::all)]

mod app;
pub use app::TemplateApp;

pub mod content;

mod json;
use crate::json::Getter;

use std::sync::{Arc, Mutex};

use json::{parse_json, Null, Value};
use lazy_static::lazy_static;

lazy_static! {
    static ref LANG: Arc<Mutex<Lang>> = Arc::new(Mutex::new(Lang::Français));
    static ref ERROR: Mutex<String> = Mutex::new(String::new());
    static ref TEXT: Value = {
        match parse_json(std::str::from_utf8(include_bytes!("../assets/content.json")).unwrap()) {
            Ok(v) => v,
            Err(e) => {
                ERROR.lock().unwrap().push_str(&e.to_string());
                Value::Null(Null)
            }
        }
    };
}

const NOT_FOUND_INDEX: &str = "not_found";
/// # Panics
///
/// Can't panic.
#[must_use]
pub fn search_content(indexes: &[&str]) -> String {
    let lang_code = crate::LANG.lock().unwrap().get_code();
    let not_found_text = crate::TEXT[lang_code.as_str()][NOT_FOUND_INDEX].to_string();

    let not_found = crate::json::Value::String(not_found_text);
    let mut result = &crate::TEXT[lang_code];

    for index in indexes {
        if let Some(res) = result.get(*index) {
            result = res;
        } else {
            return not_found.to_string();
        }
    }

    result.to_string()
}

#[macro_export]
macro_rules! t {
    ($($args:tt),*) => {
        $crate::search_content(&[$($args),+])
    };
}


pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Page {
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }
    fn name(&self) -> String;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

#[derive(Debug, PartialEq)]
pub enum Lang {
    Français,
    English,
}

impl Lang {
    fn get_code(&self) -> String {
        match self {
            Lang::Français => "fr".to_string(),
            Lang::English => "en".to_string(),
        }
    }
}
