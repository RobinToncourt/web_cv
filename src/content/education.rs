#[derive(Default)]
pub struct Education {}

impl crate::Page for Education {
    fn name(&self) -> String {
        "Formation".to_string()
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(500.0)
            .default_height(480.0)
            .open(open)
            .resizable([true, true])
            .scroll(true)
            .show(ctx, |ui| {
                use crate::View as _;
                self.ui(ui);
            });
    }
}

use crate::TEXT;

impl crate::View for Education {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading(TEXT["fr"]["edu"]["ADA"].to_string());
        ui.heading(TEXT["fr"]["edu"]["EPSI"].to_string());
        ui.heading(TEXT["fr"]["edu"]["BTS"].to_string());
    }
}
