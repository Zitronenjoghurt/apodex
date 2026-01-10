use egui::{Button, Ui, Widget};

pub struct ToggleButton<'a> {
    value: &'a mut bool,
    label: &'a str,
    tooltip: Option<&'a str>,
    enabled: bool,
}

impl<'a> ToggleButton<'a> {
    pub fn new(value: &'a mut bool, label: &'a str) -> Self {
        Self {
            value,
            label,
            tooltip: None,
            enabled: true,
        }
    }

    pub fn tooltip(mut self, tooltip: &'a str) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for ToggleButton<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let response = ui.add_enabled(self.enabled, Button::selectable(*self.value, self.label));
        if response.clicked() {
            *self.value = !*self.value;
        };

        if let Some(tooltip) = self.tooltip {
            response.on_hover_text(tooltip)
        } else {
            response
        }
    }
}
