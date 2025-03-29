use crate::content::menu::Menu;

use egui::Slider;

use crate::FontsSizer;

const MIN_FONTS_SIZE: FontsSizer = FontsSizer{ fonts_size: 4.0 };
const MAX_FONTS_SIZE: FontsSizer = FontsSizer{ fonts_size: 40.0 };
const GLOBAL_FONTS_SIZE_RANGE: std::ops::RangeInclusive<FontsSizer> = MIN_FONTS_SIZE..=MAX_FONTS_SIZE;

#[derive(Default)]
pub struct TemplateApp {
    menu: Menu,
    global_fonts_size: f32,
}

impl TemplateApp {
    /// Called once before the first frame.
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        configure_text_styles(&cc.egui_ctx);

        Self {
            menu: Menu::default(),
            global_fonts_size: 12.0
        }
    }
}

/// Text sizes.
fn configure_text_styles(ctx: &egui::Context) {
    use std::collections::BTreeMap;
    use egui::FontId;
    use egui::TextStyle;

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::proportional(25.0)),
        (TextStyle::Body, FontId::proportional(12.0)),
        (TextStyle::Monospace, FontId::monospace(12.0)),
        (TextStyle::Button, FontId::proportional(12.0)),
        (TextStyle::Small, FontId::proportional(8.0)),
    ].into();

    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            let mut fonts_sizer = FontsSizer{ fonts_size: self.global_fonts_size };

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
                ui.add(
                    Slider::new(
                        &mut fonts_sizer,
                        GLOBAL_FONTS_SIZE_RANGE
                    )
                    .max_decimals(1)
                );

            });

            self.global_fonts_size = fonts_sizer.fonts_size;
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.menu.ui(ctx);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
