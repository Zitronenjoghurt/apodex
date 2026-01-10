use crate::runtime::{file_picker, Runtime, RuntimeEvent};
use crate::windows::{ToggleableWindowState, WindowState};
use eframe::{App, Frame};
use egui::{CentralPanel, Context, FontDefinitions, TopBottomPanel, Ui};
use egui_notify::Toasts;

pub mod apod_data;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct ApodexApp {
    windows: WindowState,
    #[serde(default, skip)]
    apod_data: apod_data::ApodData,
    #[serde(default, skip)]
    runtime: Runtime,
    #[serde(default, skip)]
    toasts: Toasts,
}

impl ApodexApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self::setup_fonts(&cc.egui_ctx);
        cc.storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }

    fn setup_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }
}

impl App for ApodexApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let runtime_events = self.runtime.update(ctx);
        for event in runtime_events {
            if let Err(err) = self.handle_runtime_event(ctx, event) {
                self.toasts.error(err.to_string());
            }
        }
        TopBottomPanel::top("top_panel").show(ctx, |ui| self.show_top_panel(ui));
        CentralPanel::default().show(ctx, |ui| self.show_central_panel(ui));
        self.windows.update(ctx, &mut self.runtime, &self.apod_data);
        self.toasts.show(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

// Rendering
impl ApodexApp {
    fn show_top_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Apodex");
            ui.separator();
            self.windows.import.toggle_button(ui);
            self.windows.data.toggle_button(ui);
        });
    }

    fn show_central_panel(&mut self, _ui: &mut Ui) {}
}

// Runtime events
impl ApodexApp {
    fn handle_runtime_event(&mut self, _ctx: &Context, event: RuntimeEvent) -> anyhow::Result<()> {
        match event {
            RuntimeEvent::FilePicker(event) => self.handle_file_picker_event(event),
        }
    }

    fn handle_file_picker_event(
        &mut self,
        event: file_picker::FilePickerEvent,
    ) -> anyhow::Result<()> {
        match event.target() {
            file_picker::PickTarget::LoadEntryArchive => {
                if let Some(path) = event.single_path() {
                    self.apod_data.load_entry_archive(path)?;
                    self.toasts.success("Loaded Entry Archive");
                }
            }
            file_picker::PickTarget::LoadHtmlArchive => {
                if let Some(path) = event.single_path() {
                    self.apod_data.load_html_archive(path)?;
                    self.toasts.success("Loaded HTML Archive");
                }
            }
        }

        Ok(())
    }
}
