use egui::CollapsingHeader;

#[derive(Default)]
pub struct Experiences {}

impl crate::Page for Experiences {
    fn name(&self) -> String {
        "Expériences".to_string()
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

use crate::TEXT;
impl crate::View for Experiences {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // AKKODIS.
        ui.heading(TEXT["fr"]["exp"]["AKKODIS"].to_string());
        add_infos("KNDS_BOURGES", ui);
        add_infos("THALES_ORLEANS", ui);
        add_infos("THALES_ETRELLES", ui);
        add_infos("THALES_BREST_TBSL", ui);
        add_infos("THALES_BREST_SEA", ui);

        // SII.
        ui.separator();
        ui.heading(TEXT["fr"]["exp"]["SII"].to_string());
        add_infos("THALES_BREST_MENACE", ui);

        // Thales.
        ui.separator();
        ui.heading(TEXT["fr"]["exp"]["THALES_ALTERNANCE"].to_string());
        add_infos("THALES_BREST_SAMDIS", ui);
        add_infos("THALES_BREST_CPP_TO_JAVA", ui);

        // Kerpape.
        ui.separator();
        ui.heading(TEXT["fr"]["exp"]["KERPAPE"].to_string());
        add_infos("KERPAPE_LORIENT_I2", ui);
        add_infos("KERPAPE_LORIENT_SUMMER", ui);
        add_infos("KERPAPE_LORIENT_I1", ui);
    }
}

fn add_infos(of: &str, ui: &mut egui::Ui) {
    CollapsingHeader::new(TEXT["fr"]["exp"][format!("{of}_TITLE")].to_string())
        .id_salt(TEXT["fr"]["exp"][format!("{of}_ID")].to_string())
        .show(ui, |ui| {
            ui.label(TEXT["fr"]["exp"][format!("{of}_MISSION")].to_string());
            ui.label(TEXT["fr"]["exp"][format!("{of}_TECHNOS")].to_string());
        });
}
