use egui::{CollapsingHeader, TextStyle, FontId};

use crate::search_array;
use crate::t;

#[derive(Default)]
pub struct Experiences {
    font_id: FontId,
}

impl crate::Page for Experiences {
    fn name(&self) -> String {
        t!["exp", "exp"]
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        ctx.all_styles_mut(|styles| {
            self.font_id = styles.text_styles.get(&TextStyle::Body).unwrap().clone();
        });

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
        add_infos("KNDS_BOURGES", ui, &self.font_id, true);
        add_infos("THALES_ORLEANS", ui, &self.font_id, true);
        add_infos("THALES_ETRELLES", ui, &self.font_id, true);
        add_infos("THALES_BREST_TBSL", ui, &self.font_id, true);
        add_infos("THALES_BREST_SEA", ui, &self.font_id, true);

        // SII.
        ui.separator();
        ui.heading(t!["exp", "SII"].to_string());
        add_infos("THALES_BREST_MENACE", ui, &self.font_id, true);

        // Thales.
        ui.separator();
        ui.heading(t!["exp", "THALES_ALTERNANCE"].to_string());
        add_infos("THALES_BREST_SAMDIS", ui, &self.font_id, false);
        add_infos("THALES_BREST_CPP_TO_JAVA", ui, &self.font_id, false);

        // Kerpape.
        ui.separator();
        ui.heading(t!["exp", "KERPAPE"].to_string());
        add_infos("KERPAPE_LORIENT_I2", ui, &self.font_id, false);
        add_infos("KERPAPE_LORIENT_SUMMER", ui, &self.font_id, false);
        add_infos("KERPAPE_LORIENT_I1", ui, &self.font_id, false);
    }
}

use egui::text::LayoutJob;
use egui::TextFormat;
use egui::Color32;
fn add_infos(of: &str, ui: &mut egui::Ui, font_id: &FontId, has_client: bool) {
    let (title_color, client_date_color, mission_color, env_color) = if ui.visuals().dark_mode {
        (Color32::LIGHT_BLUE, Color32::LIGHT_GRAY, Color32::WHITE, Color32::DARK_GREEN)
    } else {
        (Color32::BLUE, Color32::DARK_GRAY, Color32::BLACK, Color32::LIGHT_GREEN)
    };

    let id: &str = &format!("{of}_ID");

    CollapsingHeader::new(create_title(of, title_color, font_id))
        .id_salt(t!["exp", id])
        .show(ui, |ui| {
            ui.label(create_client_date(of, client_date_color, font_id, has_client));
            ui.add_space(12.0);
            ui.label(create_mission(of, mission_color, font_id));
            ui.add_space(12.0);
            ui.label(create_env(of, env_color, font_id));
        });
}

fn create_title(of: &str, color: Color32, font_id: &FontId) -> LayoutJob {
    let title: &str = &format!("{of}_TITLE");
    let mut job = LayoutJob::default();

    job.append(
        &t!["exp", title],
        0.0,
        TextFormat {
            font_id: font_id.clone(),
            color,
            ..Default::default()
        },
    );

    job
}

fn create_client_date(of: &str, color: Color32, font_id: &FontId, has_client: bool) -> LayoutJob {
    let dates: &str = &format!("{of}_DATES");
    let mut job = LayoutJob::default();

    let text_format = TextFormat {
        font_id: font_id.clone(),
        color,
        ..Default::default()
    };

    if has_client {
        let client: &str = &format!("{of}_CLIENT");
        job.append(
            &t!["exp", client],
            0.0,
            text_format.clone(),
        );
        job.append(
            "\n",
            0.0,
            TextFormat::default(),
        );
    }
    job.append(
        &t!["exp", dates],
        0.0,
        text_format.clone(),
    );

    job
}

fn create_mission(of: &str, color: Color32, font_id: &FontId) -> LayoutJob {
    let mission: &str = &format!("{of}_MISSION");
    let mut job = LayoutJob::default();

    job.append(
        &t!["exp", mission],
        10.0,
        TextFormat {
            font_id: font_id.clone(),
            color,
            ..Default::default()
        },
    );

    job
}

use crate::Value;
fn create_env(of: &str, color: Color32, font_id: &FontId) -> LayoutJob {
    let env: &str = &format!("{of}_ENV");
    let mut job = LayoutJob::default();

    job.append(
        &t!["env"],
        0.0,
        TextFormat {
            font_id: font_id.clone(),
            color,
            ..Default::default()
        },
    );

    let bind = ["exp", env];
    let Value::Array(array) = search_array(&bind) else {
        panic!("Not an array.");
    };

    for e in array {
        job.append(
            "\n",
            0.0,
            TextFormat::default(),
        );
        job.append(
            &format!("> {e}"),
            0.0,
            TextFormat {
                font_id: font_id.clone(),
                color,
                ..Default::default()
            },
        );
    }

    job
}
