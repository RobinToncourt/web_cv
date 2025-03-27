#[derive(Default)]
pub struct About {}

use crate::TEXT;

impl crate::Page for About {
    fn name(&self) -> String {
        TEXT["fr"]["about"]["TITLE"].to_string()
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

impl crate::View for About {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.image(egui::include_image!("../../assets/Photo.jpeg"));
        ui.label(TEXT["fr"]["about"]["CONTENT"].to_string());

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
        "linkedin.com/in/toncourt-robin/",
        "https://www.linkedin.com/in/toncourt-robin/"
    );
}
