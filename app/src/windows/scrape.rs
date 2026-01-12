use crate::runtime::Runtime;
use crate::windows::{AppWindow, ToggleableWindowState, WindowId};
use egui::{Button, Grid, Ui, WidgetText};
use std::time::Duration;

pub struct ScrapeWindow<'a> {
    state: &'a mut ScrapeWindowState,
    runtime: &'a mut Runtime,
}

impl<'a> ScrapeWindow<'a> {
    pub fn new(state: &'a mut ScrapeWindowState, runtime: &'a mut Runtime) -> Self {
        Self { state, runtime }
    }
}

impl AppWindow for ScrapeWindow<'_> {
    fn id() -> WindowId {
        WindowId::Scrape
    }

    fn title() -> impl Into<WidgetText> {
        "Scrape"
    }

    fn is_open(&self) -> bool {
        self.state.is_open()
    }

    fn set_open(&mut self, open: bool) {
        self.state.set_open(open);
    }

    fn render_content(&mut self, ui: &mut Ui) {
        Grid::new("scrape_window_gri")
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Status");
                if let Some(status) = self.runtime.scraper.status() {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(status);
                    });
                } else {
                    ui.label("Idle");
                };
                ui.end_row();

                if self.runtime.scraper.is_busy() {
                    let eta = Duration::from_secs(self.runtime.scraper.queue_len() as u64 * 2);
                    ui.label("ETA");
                    ui.label(humantime::format_duration(eta).to_string());
                    ui.end_row();
                }
            });

        ui.separator();

        ui.horizontal(|ui| {
            let can_scrape = self.runtime.data.has_missing()
                && !self.runtime.scraper.is_busy()
                && !self.runtime.data.load_busy();
            let can_abort = self.runtime.scraper.is_busy();

            let scrape_button = ui.add_enabled(
                can_scrape,
                Button::new(format!(
                    "Download {} missing",
                    self.runtime.data.missing_count()
                )),
            );
            let abort_button = ui.add_enabled(can_abort, Button::new("Abort"));

            if scrape_button.clicked() {
                let missing = self.runtime.data.iter_missing().collect::<Vec<_>>();
                missing.into_iter().for_each(|date| {
                    self.runtime.scraper.enqueue(date);
                });
            }

            if abort_button.clicked() {
                self.runtime.scraper.abort();
            }
        });
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct ScrapeWindowState {
    pub is_open: bool,
}

impl ToggleableWindowState for ScrapeWindowState {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn toggle_label(&self) -> String {
        egui_phosphor::regular::CLOUD_ARROW_DOWN.into()
    }
}
