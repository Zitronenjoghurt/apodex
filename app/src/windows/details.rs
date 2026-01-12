use crate::runtime::Runtime;
use crate::windows::{AppWindow, ToggleableWindowState, WindowId};
use apodex::date::ApodDate;
use egui::{Ui, WidgetText};

pub struct DetailsWindow<'a> {
    state: &'a mut DetailsWindowState,
    runtime: &'a mut Runtime,
}

impl<'a> DetailsWindow<'a> {
    pub fn new(state: &'a mut DetailsWindowState, runtime: &'a mut Runtime) -> Self {
        Self { state, runtime }
    }
}

impl AppWindow for DetailsWindow<'_> {
    fn id() -> WindowId {
        WindowId::Details
    }

    fn title() -> impl Into<WidgetText> {
        "Details"
    }

    fn is_open(&self) -> bool {
        self.state.is_open()
    }

    fn set_open(&mut self, open: bool) {
        self.state.set_open(open);
    }

    fn render_content(&mut self, ui: &mut Ui) {
        let Some(entry) = self
            .runtime
            .data
            .get_entry(self.state.current_date)
            .cloned()
        else {
            ui.vertical_centered(|ui| {
                ui.small("No data for selected date.");
            });
            return;
        };

        ui.vertical_centered(|ui| {
            ui.heading(&entry.title);
        });

        ui.separator();

        self.runtime.show_image(ui, self.state.current_date);

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&entry.explanation);
        });
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct DetailsWindowState {
    pub is_open: bool,
    pub current_date: ApodDate,
}

impl ToggleableWindowState for DetailsWindowState {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn toggle_label(&self) -> String {
        egui_phosphor::regular::MAGNIFYING_GLASS.to_string()
    }
}
