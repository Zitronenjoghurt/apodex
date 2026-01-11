use crate::app::actions::AppActions;

pub mod file_picker;

#[derive(Default)]
pub struct Runtime {
    file_picker: file_picker::FilePicker,
}

impl Runtime {
    pub fn update(&mut self, ctx: &egui::Context, actions: &AppActions) {
        self.file_picker.update(ctx, actions);
    }

    pub fn file_picker(&mut self) -> &mut file_picker::FilePicker {
        &mut self.file_picker
    }
}

#[derive(Debug)]
pub enum RuntimeEvent {
    FilePicker(file_picker::FilePickerEvent),
}

pub trait RuntimeSystem {
    fn update(&mut self, ctx: &egui::Context, actions: &AppActions);
}
