use crate::app::actions::AppActions;
use std::path::Path;

pub mod apod_data;
pub mod file_picker;
mod task;

pub struct Runtime {
    tokio: tokio::runtime::Runtime,
    data: apod_data::ApodData,
    file_picker: file_picker::FilePicker,
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
        }
    }
}

impl Runtime {
    pub fn update(&mut self, ctx: &egui::Context, actions: &AppActions) {
        self.file_picker.update(ctx, actions);
        match self.data.poll_load_html() {
            Some(Ok(())) => actions.toast_success("Data loaded successfully!"),
            Some(Err(err)) => actions.toast_error(format!("Error loading data: {}", err)),
            None => {}
        }
        match self.data.poll_save_html() {
            Some(Ok(())) => actions.toast_success("Data saved successfully!"),
            Some(Err(err)) => actions.toast_error(format!("Error saving data: {}", err)),
            None => {}
        }
    }

    pub fn data(&mut self) -> &mut apod_data::ApodData {
        &mut self.data
    }

    pub fn file_picker(&mut self) -> &mut file_picker::FilePicker {
        &mut self.file_picker
    }

    pub fn is_pending(&self, pending: RuntimePending) -> bool {
        match pending {
            RuntimePending::LoadHtmlData => self.data.load_busy(),
            RuntimePending::SaveHtmlData => self.data.save_busy(),
        }
    }

    pub fn pending_status(&self, pending: RuntimePending) -> Option<String> {
        match pending {
            RuntimePending::LoadHtmlData => self.data.load_status(),
            RuntimePending::SaveHtmlData => self.data.save_status(),
        }
    }
}

// Convenience tokio helpers
impl Runtime {
    pub fn data_load_html(&mut self, path: impl AsRef<Path>) {
        self.data.start_load_html(self.tokio.handle(), path);
    }

    pub fn data_save_html(&mut self, path: impl AsRef<Path>) {
        self.data.start_save_html(self.tokio.handle(), path);
    }
}

#[derive(Debug)]
pub enum RuntimeEvent {
    FilePicker(file_picker::FilePickerEvent),
}

pub trait RuntimeSystem {
    fn update(&mut self, ctx: &egui::Context, actions: &AppActions);
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum RuntimePending {
    LoadHtmlData,
    SaveHtmlData,
}
