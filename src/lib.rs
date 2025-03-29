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
        match parse_json(crate::constants::JSON) {
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct FontsSizer {
    pub fonts_size: f32,
}

impl Default for FontsSizer {
    fn default() -> Self {
        Self {
            fonts_size: 12.0,
        }
    }
}

impl emath::Numeric for FontsSizer {
    const INTEGRAL: bool = false;
    const MIN: Self = FontsSizer { fonts_size: f32::MIN };
    const MAX: Self = FontsSizer { fonts_size: f32::MAX };

    fn to_f64(self) -> f64 {
        println!("FontsSizer::to_f64({})", self.fonts_size);
        self.fonts_size.into()
    }

    fn from_f64(num: f64) -> Self {
        // Resize every fonts.
        println!("FontsSizer::from_f64({num})");

        // Unlikely that we exceed f32::MAX value for the font.
        #[allow(clippy::cast_possible_truncation)]
        FontsSizer { fonts_size: num as f32 }
    }
}
