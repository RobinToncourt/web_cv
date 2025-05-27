use crate::t;

#[derive(Default)]
pub struct Education {}

impl crate::Page for Education {
    fn name(&self) -> String {
        t!["edu", "edu"]
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

impl crate::View for Education {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading(t!["edu", "ADA", "TITLE"]);
        ui.label(t!["edu", "ADA", "CONTENT"]);
        ui.add_space(12.0);
        ui.heading(t!["edu", "EPSI", "TITLE"]);
        ui.label(t!["edu", "EPSI", "CONTENT"]);
        ui.add_space(12.0);
        ui.heading(t!["edu", "BTS", "TITLE"]);
        ui.label(t!["edu", "BTS", "CONTENT"]);
    }
}
