use crate::runtime::file_picker::{FilePicker, PickTarget};
use crate::widgets::enum_select::EnumSelect;
use crate::windows::{AppWindow, ToggleableWindowState};
use egui::{Id, Ui, Widget, WidgetText};
use std::fmt::Display;
use strum_macros::EnumIter;

pub struct ImportWindow<'a> {
    state: &'a mut ImportWindowState,
    file_picker: &'a mut FilePicker,
}

impl<'a> ImportWindow<'a> {
    pub fn new(state: &'a mut ImportWindowState, file_picker: &'a mut FilePicker) -> Self {
        Self { state, file_picker }
    }
}

impl AppWindow for ImportWindow<'_> {
    fn id() -> Id {
        Id::new("import_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Import"
    }

    fn is_open(&self) -> bool {
        self.state.is_open()
    }

    fn set_open(&mut self, open: bool) {
        self.state.set_open(open);
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            EnumSelect::new(&mut self.state.import_type, "import_type").ui(ui);
            if ui.button("Import").clicked() {
                match self.state.import_type {
                    ImportType::HtmlArchive => {
                        self.file_picker.open_single(PickTarget::LoadHtmlArchive)
                    }
                    ImportType::EntryArchive => {
                        self.file_picker.open_single(PickTarget::LoadEntryArchive)
                    }
                }
            }
        });
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct ImportWindowState {
    pub is_open: bool,
    pub import_type: ImportType,
}

impl ToggleableWindowState for ImportWindowState {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn toggle_label(&self) -> String {
        egui_phosphor::regular::TRAY_ARROW_DOWN.to_string()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, EnumIter)]
pub enum ImportType {
    EntryArchive,
    #[default]
    HtmlArchive,
}

impl Display for ImportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportType::EntryArchive => write!(f, "Entry Archive"),
            ImportType::HtmlArchive => write!(f, "HTML Archive"),
        }
    }
}
