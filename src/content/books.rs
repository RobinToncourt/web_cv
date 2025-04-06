use crate::t;

#[derive(Default)]
pub struct Books {}

impl crate::Page for Books {
    fn name(&self) -> String {
        t!["books", "TITLE"]
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(750.0)
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

impl crate::View for Books {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading(t!["books", "LTDLM_TITLE"]);
        ui.horizontal(|ui| {
            ui.image(egui::include_image!(
                "../../assets/Les travailleurs de la mer - Victor Hugo - Couverture BD.jpg"
            ));
            ui.label(t!["books", "LTDLM_DESCR"]);
        });
        ui.separator();
    }
}
