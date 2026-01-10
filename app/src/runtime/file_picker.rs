use crate::runtime::{RuntimeEvent, RuntimeSystem};
use egui::Context;
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

#[derive(Default)]
pub struct FilePicker {
    dialog: FileDialog,
    target: Option<PickTarget>,
}

#[derive(Debug)]
pub enum FilePickerEvent {
    FilePicked {
        path: PathBuf,
        target: PickTarget,
    },
    FilesPicked {
        paths: Vec<PathBuf>,
        target: PickTarget,
    },
}

impl FilePickerEvent {
    pub fn target(&self) -> PickTarget {
        match self {
            Self::FilePicked { target, .. } => *target,
            Self::FilesPicked { target, .. } => *target,
        }
    }

    pub fn single_path(&self) -> Option<&PathBuf> {
        match self {
            Self::FilePicked { path, .. } => Some(path),
            Self::FilesPicked { paths, .. } => paths.first(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PickMode {
    Single,
    Multiple,
    Directory,
    Save { default_name: String },
}

#[derive(Debug, Copy, Clone)]
pub enum PickTarget {
    LoadEntryArchive,
    LoadHtmlArchive,
}

impl FilePicker {
    pub fn open(&mut self, target: PickTarget, mode: PickMode) {
        self.target = Some(target);
        match mode {
            PickMode::Single => self.dialog.pick_file(),
            PickMode::Multiple => self.dialog.pick_multiple(),
            PickMode::Directory => self.dialog.pick_directory(),
            PickMode::Save { default_name } => {
                self.dialog.config_mut().default_file_name = default_name;
                self.dialog.save_file();
            }
        }
    }

    pub fn open_single(&mut self, target: PickTarget) {
        self.open(target, PickMode::Single);
    }
}

impl RuntimeSystem for FilePicker {
    fn update(&mut self, ctx: &Context) -> Vec<RuntimeEvent> {
        let Some(target) = self.target else {
            return vec![];
        };

        self.dialog.update(ctx);

        if let Some(path) = self.dialog.take_picked() {
            self.target = None;
            return vec![RuntimeEvent::FilePicker(FilePickerEvent::FilePicked {
                path,
                target,
            })];
        }

        if let Some(paths) = self.dialog.take_picked_multiple() {
            self.target = None;
            return vec![RuntimeEvent::FilePicker(FilePickerEvent::FilesPicked {
                paths,
                target,
            })];
        }

        vec![]
    }
}
