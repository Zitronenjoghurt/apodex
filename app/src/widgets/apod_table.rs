use crate::app::apod_data::ApodData;
use apodex::chrono::NaiveDate;
use egui::ahash::HashSet;
use egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};

pub struct ApodTable<'a> {
    data: &'a ApodData,
    state: &'a mut ApodTableState,
}

impl<'a> ApodTable<'a> {
    pub fn new(data: &'a ApodData, state: &'a mut ApodTableState) -> Self {
        state.sort(data);
        Self { data, state }
    }
}

impl Widget for ApodTable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .column(Column::auto())
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.label("Date");
                    });
                    header.col(|ui| {
                        ui.label("Title");
                    });
                })
                .body(|body| {
                    body.rows(18.0, self.state.entry_count(), |mut row| {
                        let Some(date) = self.state.get_date(row.index()) else {
                            return;
                        };

                        let title = if let Some(entry) = self.data.get_entry(date) {
                            entry.title.clone()
                        } else {
                            "".to_string()
                        };

                        row.col(|ui| {
                            ui.label(date.to_string());
                        });
                        row.col(|ui| {
                            ui.label(title);
                        });
                    })
                });
        })
        .response
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ApodTableColumn {
    Date,
    Title,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApodTableState {
    sort_column: Option<ApodTableColumn>,
    sort_ascending: bool,
    #[serde(default, skip)]
    cached_sort_direction: Vec<NaiveDate>,
    #[serde(default, skip)]
    cached_blacklist: HashSet<NaiveDate>,
    #[serde(default, skip)]
    sort_clean: bool,
}

impl ApodTableState {
    pub fn sort(&mut self, _data: &ApodData) {
        if self.sort_clean {
            return;
        } else {
            self.sort_clean = true;
        }

        if self.cached_sort_direction.is_empty() {
            self.cached_sort_direction = apodex::iter_apod_dates().collect();
        }
    }

    pub fn entry_count(&self) -> usize {
        self.cached_sort_direction.len()
    }

    pub fn get_date(&self, index: usize) -> Option<NaiveDate> {
        self.cached_sort_direction.get(index).copied()
    }
}

impl Default for ApodTableState {
    fn default() -> Self {
        Self {
            sort_column: None,
            sort_ascending: true,
            cached_sort_direction: apodex::iter_apod_dates().collect(),
            cached_blacklist: HashSet::default(),
            sort_clean: false,
        }
    }
}
