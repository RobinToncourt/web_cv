#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

pub mod content;
mod constants;

mod json;

use std::sync::Mutex;

use lazy_static::lazy_static;
use json::{parse_json, Value, Null};

lazy_static! {
    static ref ERROR: Mutex<String> = Mutex::new(String::new());

    static ref TEXT: Value = {
        match parse_json(crate::constants::JSON) {
            Ok(v) => v,
            Err(e) => {
                ERROR.lock().unwrap().push_str(&e.to_string());
                Value::Null(Null)
            },
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
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
