use crate::runtime::file_picker::PickTarget;
use crate::runtime::{Runtime, RuntimePending};
use crate::windows::{AppWindow, ToggleableWindowState};
use egui::{Button, Ui, WidgetText};

pub struct ExportWindow<'a> {
    state: &'a mut ExportWindowState,
    runtime: &'a mut Runtime,
}

impl<'a> ExportWindow<'a> {
    pub fn new(state: &'a mut ExportWindowState, runtime: &'a mut Runtime) -> Self {
        Self { state, runtime }
    }
}

impl AppWindow for ExportWindow<'_> {
    fn id() -> crate::windows::WindowId {
        crate::windows::WindowId::Export
    }

    fn title() -> impl Into<WidgetText> {
        "Export"
    }

    fn is_open(&self) -> bool {
        self.state.is_open()
    }

    fn set_open(&mut self, open: bool) {
        self.state.set_open(open);
    }

    fn render_content(&mut self, ui: &mut Ui) {
        let data_available = self.runtime.data().latest_html_date().is_some();
        let is_loading = self.runtime.is_pending(RuntimePending::SaveHtmlData);

        ui.horizontal(|ui| {
            let button_response = ui.add_enabled(
                !is_loading && data_available,
                Button::new("Export HTML archive"),
            );
            if button_response.clicked()
                && let Some(latest_date) = self.runtime.data().latest_entry_date()
            {
                let date = latest_date.format("%Y-%m-%d").to_string();
                let file_name = format!("apodex-html-archive-{}.bin", date);
                self.runtime
                    .file_picker()
                    .open_save(PickTarget::SaveHtmlArchive, file_name);
            }
        });

        if is_loading {
            ui.horizontal(|ui| {
                ui.spinner();
                if let Some(status) = self.runtime.pending_status(RuntimePending::SaveHtmlData) {
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
pub struct ExportWindowState {
    pub is_open: bool,
}

impl ToggleableWindowState for ExportWindowState {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn toggle_label(&self) -> String {
        egui_phosphor::regular::TRAY_ARROW_UP.to_string()
    }
}
