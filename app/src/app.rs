use crate::app::actions::AppAction;
use crate::runtime::{file_picker, Runtime};
use crate::windows::{ToggleableWindowState, WindowState};
use eframe::{App, Frame};
use egui::{CentralPanel, Context, FontDefinitions, TopBottomPanel, Ui};
use egui_notify::Toasts;

pub mod actions;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct ApodexApp {
    windows: WindowState,
    #[serde(default, skip)]
    pub actions: actions::AppActions,
    #[serde(default, skip)]
    pub runtime: Runtime,
    #[serde(default, skip)]
    pub toasts: Toasts,
}

impl ApodexApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self::setup_fonts(&cc.egui_ctx);
        let mut app = cc
            .storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default();
        app.runtime.data_load_included_html();
        app
    }

    fn setup_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }

    fn update_windows(&mut self, ctx: &Context) {
        let mut windows = std::mem::take(&mut self.windows);
        windows.update(ctx, self);
        self.windows = windows;
    }
}

impl App for ApodexApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| self.show_top_panel(ui));
        CentralPanel::default().show(ctx, |ui| self.show_central_panel(ui));
        self.update_windows(ctx);
        self.toasts.show(ctx);

        self.runtime.update(ctx, &self.actions);
        for action in self.actions.take_actions() {
            if let Err(err) = self.handle_action(ctx, action) {
                self.toasts.error(err.to_string());
            }
        }
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
            self.windows.export.toggle_button(ui);
            self.windows.data.toggle_button(ui);
            self.windows.details.toggle_button(ui);
            self.windows.scrape.toggle_button(ui);
        });
    }

    fn show_central_panel(&mut self, _ui: &mut Ui) {}
}

// Actions
impl ApodexApp {
    fn handle_action(&mut self, ctx: &Context, action: AppAction) -> anyhow::Result<()> {
        match action {
            AppAction::DetailsSelectDate(date) => self.windows.details.current_date = date,
            AppAction::FilePickerAction(action) => self.handle_file_picker_action(action)?,
            AppAction::OpenAndFocusWindow(window_id) => self.windows.open_and_focus(ctx, window_id),
            AppAction::ToastError(message) => {
                self.toasts.error(message);
            }
            AppAction::ToastSuccess(message) => {
                self.toasts.success(message);
            }
            AppAction::ToastWarning(message) => {
                self.toasts.warning(message);
            }
            AppAction::InsertHtml { date, html } => self.runtime.data().insert_html(date, html),
        };
        Ok(())
    }

    fn handle_file_picker_action(
        &mut self,
        action: file_picker::FilePickerAction,
    ) -> anyhow::Result<()> {
        match action.target() {
            file_picker::PickTarget::LoadHtmlArchive => {
                if let Some(path) = action.single_path() {
                    self.runtime.data_load_html(path);
                }
            }
            file_picker::PickTarget::SaveHtmlArchive => {
                if let Some(path) = action.single_path() {
                    self.runtime.data_save_html(path);
                }
            }
        }

        Ok(())
    }
}
