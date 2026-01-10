use crate::app::apod_data::ApodData;
use crate::runtime::Runtime;
use crate::widgets::toggle_button::ToggleButton;
use crate::windows::data::DataWindow;
use crate::windows::import::ImportWindow;
use egui::{Context, Id, Ui, Widget, WidgetText};
use serde::{Deserialize, Serialize};

mod data;
mod import;

#[derive(Default, Serialize, Deserialize)]
pub struct WindowState {
    pub data: data::DataWindowState,
    pub import: import::ImportWindowState,
}

impl WindowState {
    pub fn update(&mut self, ctx: &Context, runtime: &mut Runtime, apod_data: &ApodData) {
        ImportWindow::new(&mut self.import, runtime.file_picker()).show(ctx);
        DataWindow::new(&mut self.data, apod_data).show(ctx);
    }
}

pub trait AppWindow: Sized {
    fn id() -> Id;
    fn title() -> impl Into<WidgetText>;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn render_content(&mut self, ui: &mut Ui);

    fn resizable(&self) -> bool {
        true
    }

    fn movable(&self) -> bool {
        true
    }

    fn collapsible(&self) -> bool {
        false
    }

    fn show(mut self, ctx: &Context) {
        let mut is_open = self.is_open();
        egui::Window::new(Self::title())
            .id(Self::id())
            .open(&mut is_open)
            .fade_in(true)
            .fade_out(true)
            .resizable(self.resizable())
            .movable(self.movable())
            .collapsible(self.collapsible())
            .show(ctx, |ui| self.render_content(ui));
        self.set_open(is_open && self.is_open());
    }
}

pub trait ToggleableWindowState: Sized {
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn toggle_label(&self) -> String;

    fn toggle_button(&mut self, ui: &mut Ui) {
        let mut is_open = self.is_open();
        ToggleButton::new(&mut is_open, &self.toggle_label()).ui(ui);
        self.set_open(is_open);
    }
}
