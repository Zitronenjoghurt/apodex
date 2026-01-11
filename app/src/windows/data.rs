use crate::app::actions::AppActions;
use crate::runtime::apod_data::ApodData;
use crate::widgets::apod_table::{ApodTable, ApodTableState};
use crate::windows::{AppWindow, ToggleableWindowState, WindowId};
use egui::{Ui, Widget, WidgetText};

pub struct DataWindow<'a> {
    state: &'a mut DataWindowState,
    actions: &'a AppActions,
    apod_data: &'a ApodData,
}

impl<'a> DataWindow<'a> {
    pub fn new(
        state: &'a mut DataWindowState,
        actions: &'a AppActions,
        apod_data: &'a ApodData,
    ) -> Self {
        Self {
            state,
            actions,
            apod_data,
        }
    }
}

impl AppWindow for DataWindow<'_> {
    fn id() -> WindowId {
        WindowId::Data
    }

    fn title() -> impl Into<WidgetText> {
        "Data"
    }

    fn is_open(&self) -> bool {
        self.state.is_open()
    }

    fn set_open(&mut self, open: bool) {
        self.state.set_open(open);
    }

    fn render_content(&mut self, ui: &mut Ui) {
        ApodTable::new(&mut self.state.table_state, self.actions, self.apod_data).ui(ui);
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct DataWindowState {
    pub is_open: bool,
    pub table_state: ApodTableState,
}

impl ToggleableWindowState for DataWindowState {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn toggle_label(&self) -> String {
        egui_phosphor::regular::ARCHIVE.to_string()
    }
}
