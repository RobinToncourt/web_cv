use std::collections::BTreeSet;

use crate::Cv;
use crate::content::about::About;

pub struct Menu {
    pages: Vec<Box<dyn Cv>>,
    open: BTreeSet<String>,
}

impl Default for Menu {
    fn default() -> Self {
        let pages: Vec<Box<dyn Cv>> = vec![
            Box::<About>::default(),
        ];
        let open = BTreeSet::new();

        Self {
            pages,
            open,
        }
    }
}

impl Menu {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Menu")
            .default_width(320.0)
            .default_height(620.0)
            .resizable([false, false])
            .scroll(false)
            .show(ctx, |ui| {
                ui.heading("Menu");
                ui.separator();
                for page in &self.pages {
                    let mut is_open = self.open.contains(page.name());
                    ui.toggle_value(&mut is_open, page.name());
                    set_open(&mut self.open, page.name(), is_open)
                }
            });

        for page in &mut self.pages {
            let mut is_open = self.open.contains(page.name());
            page.show(ctx, &mut is_open);
            set_open(&mut self.open, page.name(), is_open);
        }
    }
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
