#![warn(clippy::all)]

mod app;
pub use app::TemplateApp;

mod constants;
pub mod content;

mod json;

use std::sync::Mutex;

use json::{parse_json, Null, Value};
use lazy_static::lazy_static;

lazy_static! {
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
