use crate::runtime::file_picker::PickTarget;
use crate::runtime::{Runtime, RuntimePending};
use crate::windows::{AppWindow, ToggleableWindowState, WindowId};
use egui::{Button, Ui, WidgetText};

pub struct ImportWindow<'a> {
    state: &'a mut ImportWindowState,
    runtime: &'a mut Runtime,
}

impl<'a> ImportWindow<'a> {
    pub fn new(state: &'a mut ImportWindowState, runtime: &'a mut Runtime) -> Self {
        Self { state, runtime }
    }
}

impl AppWindow for ImportWindow<'_> {
    fn id() -> WindowId {
        WindowId::Import
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
        let is_loading = self.runtime.is_pending(RuntimePending::LoadHtmlData);

        ui.horizontal(|ui| {
            let button_response = ui.add_enabled(!is_loading, Button::new("Import HTML archive"));
            if button_response.clicked() {
                self.runtime
                    .file_picker()
                    .open_single(PickTarget::LoadHtmlArchive);
            }
        });

        if is_loading {
            ui.horizontal(|ui| {
                ui.spinner();
                if let Some(status) = self.runtime.pending_status(RuntimePending::LoadHtmlData) {
                    ui.label(status);
                }
            });
        }
    }

    fn resizable(&self) -> bool {
        false
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct ImportWindowState {
    pub is_open: bool,
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
