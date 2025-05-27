use crate::{search_array, show_array_cell_by_line};
use crate::t;

#[derive(Default)]
pub struct Skills {}

impl crate::Page for Skills {
    fn name(&self) -> String {
        t!["skills", "skills"]
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

impl crate::View for Skills {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Langages");
        show_array_cell_by_line(search_array(&["skills", "PROG_LANG"]), None, ui);
        ui.separator();
        ui.heading("Outils");
        show_array_cell_by_line(search_array(&["skills", "TOOLS"]), None, ui);
        ui.separator();
        ui.heading("Environnement");
        show_array_cell_by_line(search_array(&["skills", "ENV"]), None, ui);
        ui.separator();
        ui.heading("Langues");
        show_array_cell_by_line(search_array(&["skills", "LANG"]), None, ui);
    }
}
