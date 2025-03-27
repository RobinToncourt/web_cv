#[derive(Default)]
pub struct Skills {}

impl crate::Page for Skills {
    fn name(&self) -> String {
        "Compétences".to_string()
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

use crate::TEXT;

impl crate::View for Skills {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Langages");
        show_array_cell_by_line(&TEXT["fr"]["skills"]["PROG_LANG"], ui);
        ui.separator();
        ui.heading("Outils");
        show_array_cell_by_line(&TEXT["fr"]["skills"]["TOOLS"], ui);
        ui.separator();
        ui.heading("Environnement");
        show_array_cell_by_line(&TEXT["fr"]["skills"]["ENV"], ui);
        ui.separator();
        ui.heading("Langues");
        show_array_cell_by_line(&TEXT["fr"]["skills"]["LANG"], ui);
    }
}

use crate::json::Value;

fn show_array_cell_by_line(value: &Value, ui: &mut egui::Ui) {
    match value {
        Value::Array(array) => {
            for e in array {
                ui.label(e.to_string());
            }
        },
        _ => panic!("Not an array."),
    }
}
