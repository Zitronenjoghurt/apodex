use crate::app::ApodexApp;

mod app;
mod runtime;
mod widgets;
mod windows;

fn main() {
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("Apodex")
            .with_app_id("io.github.zitronenjoghurt.apodex"),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Apodex",
        native_options,
        Box::new(|cc| Ok(Box::new(ApodexApp::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
