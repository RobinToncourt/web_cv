#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

pub mod content;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Cv {
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
