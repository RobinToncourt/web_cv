#[derive(Default)]
pub struct Games {}

impl crate::Page for Games {
    fn name(&self) -> String {
        "TODO: games title".to_string()
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

impl crate::View for Games {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO: content");
    }
}
