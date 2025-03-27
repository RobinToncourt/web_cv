#[derive(Default)]
pub struct Books {}

impl crate::Page for Books {
    fn name(&self) -> String {
        "TODO: book title".to_string()
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            .open(open)
            .resizable([false, false])
            .scroll(false)
            .show(ctx, |ui| {
                use crate::View as _;
                self.ui(ui);
            });
    }
}

impl crate::View for Books {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO: content");
    }
}
