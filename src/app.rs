use crate::content::menu::Menu;

use std::collections::BTreeMap;

use egui::FontId;
use egui::TextStyle;

use crate::Lang;
use crate::t;

pub struct TemplateApp {
    menu: Menu,
    global_fonts_size: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            menu: Menu::default(),
            global_fonts_size: 12.0,
        }
    }
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

        Self::default()
    }
}

/// Text sizes.
fn configure_text_styles(ctx: &egui::Context) {
    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::proportional(25.0)),
        (TextStyle::Body, FontId::proportional(12.0)),
        (TextStyle::Monospace, FontId::monospace(12.0)),
        (TextStyle::Button, FontId::proportional(12.0)),
        (TextStyle::Small, FontId::proportional(8.0)),
    ]
    .into();

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

        {
            ctx.all_styles_mut(|style| {
                for (text_style, font_id) in &mut style.text_styles {
                    match text_style {
                        TextStyle::Heading =>
                        font_id.size = self.global_fonts_size + 13.0,
                        TextStyle::Small =>
                        font_id.size = self.global_fonts_size - 4.0,
                        _ =>font_id.size = self.global_fonts_size,
                    }
                }
            });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            let lang_label = t!["lang_label"];
            let font_size_label = t!["fonts_size"];

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
                ui.label(lang_label);
                let lang: &mut Lang = &mut crate::LANG.lock().unwrap();
                egui::ComboBox::from_id_salt("lang-combobox")
                    .selected_text(format!("{lang:?}"))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(lang, Lang::Français, "Français");
                        ui.selectable_value(lang, Lang::English, "English");
                    });

                ui.label(font_size_label);
                ui.add(egui::DragValue::new(&mut self.global_fonts_size)
                    .speed(1.0)
                    .range(8.0..=20.0)
                );
            });
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
