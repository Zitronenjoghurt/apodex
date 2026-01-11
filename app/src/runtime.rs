use crate::app::actions::AppActions;
use bitflags::bitflags;

pub mod apod_data;
pub mod file_picker;
mod task;

#[derive(Default)]
pub struct Runtime {
    data: apod_data::ApodData,
    file_picker: file_picker::FilePicker,
}

impl Runtime {
    pub fn update(&mut self, ctx: &egui::Context, actions: &AppActions) {
        self.file_picker.update(ctx, actions);
        match self.data.poll() {
            Some(Ok(())) => actions.toast_success("Data loaded successfully!"),
            Some(Err(err)) => actions.toast_error(format!("Error loading data: {}", err)),
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
            RuntimePending::LOAD_DATA => self.data.is_loading(),
            _ => false,
        }
    }

    pub fn pending_status(&self, pending: RuntimePending) -> Option<String> {
        match pending {
            RuntimePending::LOAD_DATA => self.data.pending_status(),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum RuntimeEvent {
    FilePicker(file_picker::FilePickerEvent),
}

pub trait RuntimeSystem {
    fn update(&mut self, ctx: &egui::Context, actions: &AppActions);
}

bitflags! {
    #[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct RuntimePending: u32 {
        const LOAD_DATA = 0b1;
    }
}
