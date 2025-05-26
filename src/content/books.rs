use egui_extras::{Size, StripBuilder};

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
        let ltdlm_img = egui::Image::new(egui::include_image!(
            "../../assets/Les travailleurs de la mer - Victor Hugo - Couverture BD.jpg"
        ))
        .max_width(200.0);

        let mnqvq_img = egui::Image::new(egui::include_image!(
            "../../assets/Mil neuf cent quatre-vingt-quatre.jpg"
        ))
        .max_width(200.0);

        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .size(Size::exact(1.0))
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| {
                strip.strip(|builder| {
                    builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.add(ltdlm_img);
                        });
                        strip.cell(|ui| {
                            ui.label(t!["books", "LTDLM_DESCR"]);
                        });
                    });
                });

                strip.strip(|builder| {
                    builder.size(Size::remainder()).vertical(|mut strip| {
                        strip.cell(|ui| {
                            ui.separator();
                        });
                    });
                });

                strip.strip(|builder| {
                    builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.label(t!["books", "1984_DESCR"]);
                        });
                        strip.cell(|ui| {
                            ui.add(mnqvq_img);
                        });
                    });
                });
            });
    }
}
