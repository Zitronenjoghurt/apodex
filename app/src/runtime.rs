use crate::app::actions::AppActions;
use apodex::date::ApodDate;
use egui::Ui;
use std::path::Path;

pub mod apod_data;
mod apod_media;
pub mod file_picker;
mod scraper;
mod task;

pub struct Runtime {
    tokio: tokio::runtime::Runtime,
    pub data: apod_data::ApodData,
    pub file_picker: file_picker::FilePicker,
    pub media: apod_media::ApodMedia,
    pub scraper: scraper::Scraper,
}

impl Default for Runtime {
    fn default() -> Self {
        let tokio = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");

        Self {
            tokio,
            data: Default::default(),
            file_picker: Default::default(),
            media: Default::default(),
            scraper: Default::default(),
        }
    }
}

impl Runtime {
    pub fn update(&mut self, ctx: &egui::Context, actions: &AppActions) {
        self.file_picker.update(ctx, self.tokio.handle(), actions);
        self.data.update(ctx, self.tokio.handle(), actions);
        self.scraper.update(ctx, self.tokio.handle(), actions);
        self.media.update(ctx, self.tokio.handle(), actions);
    }
}

// Convenience tokio helpers
impl Runtime {
    pub fn data_load_included_html(&mut self) {
        self.data.start_load_included_html(self.tokio.handle());
    }

    pub fn data_load_html(&mut self, path: impl AsRef<Path>) {
        self.data.start_load_html(self.tokio.handle(), path);
    }

    pub fn data_save_html(&mut self, path: impl AsRef<Path>) {
        self.data.start_save_html(self.tokio.handle(), path);
    }

    pub fn show_image(&mut self, ui: &mut Ui, date: ApodDate) {
        if let Some(entry) = self.data.get_entry(date) {
            self.media.show_image(ui, entry);
        } else {
            ui.small("No entry found");
        }
    }
}

pub trait RuntimeSystem {
    fn update(
        &mut self,
        ctx: &egui::Context,
        handle: &tokio::runtime::Handle,
        actions: &AppActions,
    );
}
