#[derive(Default)]
pub struct About {}

impl crate::Page for About {
    fn name(&self) -> &'static str {
        "À propos de moi"
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

impl crate::View for About {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Robin Toncourt");
        ui.image(egui::include_image!("../../assets/Photo.jpeg"));
        ui.label(TEXT["fr"]["about"].to_string());

        ui.add_space(12.0);

        links(ui);
    }
}

fn links(ui: &mut egui::Ui) {
    use egui::special_emojis::GITHUB;
    ui.hyperlink_to(
        format!("{GITHUB} github.com/RobinToncourt"),
        "https://github.com/RobinToncourt"
    );
    ui.hyperlink_to(
        format!("linkedin.com/in/toncourt-robin/"),
        "https://www.linkedin.com/in/toncourt-robin/"
    );
}
