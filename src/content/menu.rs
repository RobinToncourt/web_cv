use std::collections::BTreeSet;

use crate::content::{
    about::About, books::Books, education::Education, experiences::Experiences, games::Games,
    movies::Movies, projects::Projects, skills::Skills, sport::Sport,
};
use crate::Page;

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
        let mut open = BTreeSet::new();

        set_open(&mut open, &about.name(), true);
        //set_open(&mut open, &hobby_pages[0].name(), true);

        Self {
            open,
            about,
            experience_pages,
            hobby_pages,
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
                // ui.add_space(12.0);
                // ui.heading("Passe temps");
                // ui.separator();
                // self.toggle_hobby(ui);
                // ui.add_space(12.0);
                // ui.heading("Questions");
                // ui.separator();
            });

        {
            let about_name = self.about.name();
            let mut is_open = self.open.contains(&about_name);
            self.about.show(ctx, &mut is_open);
            set_open(&mut self.open, &about_name, is_open);
        }

        for page in &mut self.experience_pages {
            show(ctx, &mut self.open, page);
        }

        for page in &mut self.hobby_pages {
            show(ctx, &mut self.open, page);
        }
    }

    fn toggle_about(&mut self, ui: &mut egui::Ui) {
        let about_name = self.about.name();
        let mut is_open = self.open.contains(&about_name);
        ui.toggle_value(&mut is_open, &about_name);
        set_open(&mut self.open, &about_name, is_open);
    }

    fn toggle_experience(&mut self, ui: &mut egui::Ui) {
        for page in &self.experience_pages {
            let page_name = page.name();
            let mut is_open = self.open.contains(&page_name);
            ui.toggle_value(&mut is_open, &page_name);
            set_open(&mut self.open, &page_name, is_open);
        }
    }

    fn toggle_hobby(&mut self, ui: &mut egui::Ui) {
        for page in &self.hobby_pages {
            let page_name = page.name();
            let mut is_open = self.open.contains(&page_name);
            ui.toggle_value(&mut is_open, &page_name);
            set_open(&mut self.open, &page_name, is_open);
        }
    }
}

fn show(ctx: &egui::Context, open: &mut BTreeSet<String>, page: &mut Box<dyn Page>) {
    let page_name = page.name();
    let mut is_open = open.contains(&page_name);
    page.show(ctx, &mut is_open);
    set_open(open, &page_name, is_open);
}

fn set_open(open: &mut BTreeSet<String>, key: &str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}
