use crate::app::ApodexApp;
use crate::widgets::toggle_button::ToggleButton;
use crate::windows::data::DataWindow;
use crate::windows::details::DetailsWindow;
use crate::windows::export::ExportWindow;
use crate::windows::import::ImportWindow;
use egui::{Context, Ui, Widget, WidgetText};
use serde::{Deserialize, Serialize};

mod data;
mod details;
mod export;
mod import;

#[derive(Default, Serialize, Deserialize)]
pub struct WindowState {
    pub data: data::DataWindowState,
    pub details: details::DetailsWindowState,
    pub export: export::ExportWindowState,
    pub import: import::ImportWindowState,
}

impl WindowState {
    pub fn update(&mut self, ctx: &Context, app: &mut ApodexApp) {
        DataWindow::new(&mut self.data, &app.actions, app.runtime.data()).show(ctx);
        DetailsWindow::new(&mut self.details, app.runtime.data()).show(ctx);
        ExportWindow::new(&mut self.export, &mut app.runtime).show(ctx);
        ImportWindow::new(&mut self.import, &mut app.runtime).show(ctx);
    }

    pub fn open_and_focus(&mut self, ctx: &Context, window_id: WindowId) {
        match window_id {
            WindowId::Data => self.data.set_open(true),
            WindowId::Details => self.details.set_open(true),
            WindowId::Export => self.export.set_open(true),
            WindowId::Import => self.import.set_open(true),
        }
        ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, window_id.egui_id()));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowId {
    Data,
    Details,
    Export,
    Import,
}

impl WindowId {
    pub fn egui_id(&self) -> egui::Id {
        egui::Id::new(format!("app_window_{self:?}"))
    }
}

pub trait AppWindow: Sized {
    fn id() -> WindowId;
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
            .id(Self::id().egui_id())
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
