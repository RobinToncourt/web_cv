use egui::text::LayoutJob;
use egui::TextFormat;
use egui::TextStyle;
use egui::Color32;
use egui::FontId;

use crate::t;

#[derive(Default)]
pub struct Education {
    heading_font_id: FontId,
}

impl crate::Page for Education {
    fn name(&self) -> String {
        t!["edu", "edu"]
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        ctx.all_styles_mut(|styles| {
            self.heading_font_id = styles.text_styles.get(&TextStyle::Heading).unwrap().clone();
        });

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
        let color = if ui.visuals().dark_mode {
            Color32::LIGHT_BLUE
        } else {
            Color32::BLUE
        };

        ui.label(create_heading(&t!["edu", "ADA", "TITLE"], &color, &self.heading_font_id));
        ui.label(t!["edu", "ADA", "CONTENT"]);
        ui.add_space(12.0);
        ui.label(create_heading(&t!["edu", "EPSI", "TITLE"], &color, &self.heading_font_id));
        ui.label(t!["edu", "EPSI", "CONTENT"]);
        ui.add_space(12.0);
        ui.label(create_heading(&t!["edu", "BTS", "TITLE"], &color, &self.heading_font_id));
        ui.label(t!["edu", "BTS", "CONTENT"]);
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
