use egui::CollapsingHeader;

use crate::{search_array, show_array_cell_by_line};
use crate::t;

#[derive(Default)]
pub struct Experiences {}

impl crate::Page for Experiences {
    fn name(&self) -> String {
        t!["exp", "exp"]
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(520.0)
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

impl crate::View for Experiences {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // AKKODIS.
        ui.heading(t!["exp", "AKKODIS"].to_string());
        add_infos("KNDS_BOURGES", ui);
        add_infos("THALES_ORLEANS", ui);
        add_infos("THALES_ETRELLES", ui);
        add_infos("THALES_BREST_TBSL", ui);
        add_infos("THALES_BREST_SEA", ui);

        // SII.
        ui.separator();
        ui.heading(t!["exp", "SII"].to_string());
        add_infos("THALES_BREST_MENACE", ui);

        // Thales.
        ui.separator();
        ui.heading(t!["exp", "THALES_ALTERNANCE"].to_string());
        add_infos("THALES_BREST_SAMDIS", ui);
        add_infos("THALES_BREST_CPP_TO_JAVA", ui);

        // Kerpape.
        ui.separator();
        ui.heading(t!["exp", "KERPAPE"].to_string());
        add_infos("KERPAPE_LORIENT_I2", ui);
        add_infos("KERPAPE_LORIENT_SUMMER", ui);
        add_infos("KERPAPE_LORIENT_I1", ui);
    }
}

fn add_infos(of: &str, ui: &mut egui::Ui) {
    let title: &str = &format!("{of}_TITLE");
    let id: &str = &format!("{of}_ID");
    let mission: &str = &format!("{of}_MISSION");
    let env: &str = &format!("{of}_ENV");
    CollapsingHeader::new(t!["exp", title])
        .id_salt(t!["exp", id])
        .show(ui, |ui| {
            ui.label(t!["exp", mission]);
            ui.add_space(12.0);
            ui.label(t!["env"]);
            show_array_cell_by_line(search_array(&["exp", env]), Some("> "), ui);
        });
}
