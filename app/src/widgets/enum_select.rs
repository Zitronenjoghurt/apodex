use egui::{Ui, Widget};
use std::fmt::Display;
use strum::IntoEnumIterator;

pub struct EnumSelect<'a, T>
where
    T: IntoEnumIterator + PartialEq + Copy + Display,
{
    value: &'a mut T,
    label: Option<&'a str>,
    id: &'a str,
}

impl<'a, T> EnumSelect<'a, T>
where
    T: IntoEnumIterator + PartialEq + Copy + Display,
{
    pub fn new(value: &'a mut T, id: &'a str) -> Self {
        Self {
            value,
            label: None,
            id,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }
}

impl<T> Widget for EnumSelect<'_, T>
where
    T: IntoEnumIterator + PartialEq + Copy + Display,
{
    fn ui(self, ui: &mut Ui) -> egui::Response {
        egui::ComboBox::new(self.id, self.label.unwrap_or_default())
            .selected_text(self.value.to_string())
            .show_ui(ui, |ui| {
                for variant in T::iter() {
                    ui.selectable_value(self.value, variant, variant.to_string());
                }
            })
            .response
    }
}
