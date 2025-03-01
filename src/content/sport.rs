#[derive(Default)]
pub struct Sport {}

impl crate::Page for Sport {
    fn name(&self) -> &'static str {
        "Activité physique"
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

impl crate::View for Sport {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Robin Toncourt");
        ui.image(egui::include_image!("../../assets/Photo.jpeg"));
        ui.label(format!(
            "Déterminé, sérieux, autonome et conscient du
            travail qui m'attend, je suis persuadé que je serais
            un élément moteur au sein de votre structure !"
        ));

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
