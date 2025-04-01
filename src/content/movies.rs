use crate::t;

#[derive(Default)]
pub struct Movies {}

impl crate::Page for Movies {
    fn name(&self) -> String {
        "TODO: movies title".to_string()
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

impl crate::View for Movies {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO: content");

        ui.label(t!["about", "CONTENT"]);

        if ui.button("test lang").clicked() {
            println!("{:}", crate::LANG.lock().unwrap().get_code());
        }
    }
}

