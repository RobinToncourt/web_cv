use std::collections::BTreeSet;

use crate::Page;
use crate::content::{
    about::About,
    experiences::Experiences,
    skills::Skills,
    education::Education,
    games::Games,
    movies::Movies,
    books::Books,
    sport::Sport,
    projects::Projects,
};

pub struct Menu {
    open: BTreeSet<String>,

    about: About,
    experience_pages: Vec<Box<dyn Page>>,
    hobby_pages: Vec<Box<dyn Page>>,
}

impl Default for Menu {
    fn default() -> Self {
        let about = About::default();
        let experience_pages: Vec<Box<dyn Page>> = vec![
            Box::<Experiences>::default(),
            Box::<Skills>::default(),
            Box::<Education>::default(),
        ];
        let hobby_pages: Vec<Box<dyn Page>> = vec![
            Box::<Books>::default(),
            Box::<Sport>::default(),
            Box::<Movies>::default(),
            Box::<Projects>::default(),
            Box::<Games>::default(),
        ];
        let open = BTreeSet::new();

        Self {
            about,
            experience_pages,
            hobby_pages,
            open,
        }
    }
}

impl Menu {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Menu")
            .default_width(180.0)
            .default_height(820.0)
            .resizable([false, false])
            .scroll(false)
            .show(ctx, |ui| {
                self.toggle_about(ui);
                ui.add_space(12.0);
                ui.heading("Expériences");
                ui.separator();
                self.toggle_experience(ui);
                ui.add_space(12.0);
                ui.heading("Passe temps");
                ui.separator();
                self.toggle_hobby(ui);
                ui.add_space(12.0);
                ui.heading("Questions");
                ui.separator();
            });

        {
            let mut is_open = self.open.contains(self.about.name());
            self.about.show(ctx, &mut is_open);
            set_open(&mut self.open, self.about.name(), is_open);
        }

        for page in &mut self.experience_pages {
            show(ctx, &mut self.open, page);
        }

        for page in &mut self.hobby_pages {
            show(ctx, &mut self.open, page);
        }

    }

    fn toggle_about(&mut self, ui: &mut egui::Ui) {
        let mut is_open = self.open.contains(self.about.name());
        ui.toggle_value(&mut is_open, self.about.name());
        set_open(&mut self.open, self.about.name(), is_open);
    }

    fn toggle_experience(&mut self, ui: &mut egui::Ui) {
        for page in &self.experience_pages {
            let mut is_open = self.open.contains(page.name());
            ui.toggle_value(&mut is_open, page.name());
            set_open(&mut self.open, page.name(), is_open);
        }
    }

    fn toggle_hobby(&mut self, ui: &mut egui::Ui) {
        for page in &self.hobby_pages {
            let mut is_open = self.open.contains(page.name());
            ui.toggle_value(&mut is_open, page.name());
            set_open(&mut self.open, page.name(), is_open);
        }
    }
}

fn show(ctx: &egui::Context, open: &mut BTreeSet<String>, page: &mut Box<dyn Page>) {
    let mut is_open = open.contains(page.name());
    page.show(ctx, &mut is_open);
    set_open(open, page.name(), is_open);
}

fn toggle() {

}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
