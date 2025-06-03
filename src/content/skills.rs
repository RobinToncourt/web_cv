use egui::text::LayoutJob;
use egui::TextFormat;
use egui::TextStyle;
use egui::Color32;
use egui::FontId;

use crate::{search_array, show_array_cell_by_line};
use crate::t;

#[derive(Default)]
pub struct Skills {
    heading_font_id: FontId,
}

impl crate::Page for Skills {
    fn name(&self) -> String {
        t!["skills", "skills"]
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        ctx.all_styles_mut(|styles| {
            self.heading_font_id = styles.text_styles.get(&TextStyle::Heading).unwrap().clone();
        });

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
        let color = if ui.visuals().dark_mode {
            Color32::LIGHT_BLUE
        } else {
            Color32::BLUE
        };

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

fn create_heading(text: &str, color: &Color32, heading_font_id: &FontId) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.append(
        text,
        0.0,
        TextFormat {
            font_id: heading_font_id.clone(),
               color: color.clone(),
               ..Default::default()
        },
    );
    job
}
