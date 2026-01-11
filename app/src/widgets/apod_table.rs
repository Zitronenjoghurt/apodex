use crate::app::apod_data::ApodData;
use crate::widgets::option_enum_select::OptionEnumSelect;
use apodex::date::ApodDate;
use egui::{CursorIcon, Hyperlink, Popup, RectAlign, Response, RichText, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use std::fmt::{Display, Formatter};
use std::time::Instant;
use strum_macros::EnumIter;

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
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Selected Entries: {}/{}",
                        self.state.entry_count(),
                        ApodDate::total_apod_days()
                    ))
                });
            });

            ui.separator();

            TableBuilder::new(ui)
                .striped(true)
                .column(Column::exact(80.0))
                .column(Column::exact(85.0))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        self.state
                            .render_simple_column_sort(ui, ApodTableColumn::Date);
                    });
                    header.col(|ui| {
                        self.state.render_status_column(ui);
                    });
                    header.col(|ui| {
                        self.state.render_title_column(ui);
                    });
                })
                .body(|body| {
                    body.rows(18.0, self.state.entry_count(), |mut row| {
                        let Some(date) = self.state.get_date(row.index()) else {
                            return;
                        };

                        let date_link = if let Some(entry) = self.data.get_entry(date) {
                            entry.link()
                        } else {
                            None
                        };

                        let title = if let Some(entry) = self.data.get_entry(date) {
                            entry.title.clone()
                        } else {
                            "".to_string()
                        };

                        let status = if self.data.get_error(date).is_some() {
                            RichText::new("Failed").color(egui::Color32::RED)
                        } else if let Some(warnings) = self.data.get_warnings(date) {
                            if warnings.len() == 1 {
                                RichText::new("1 warning")
                            } else {
                                RichText::new(format!("{} warnings", warnings.len()))
                            }
                            .color(egui::Color32::YELLOW)
                        } else if self.data.get_entry(date).is_some() {
                            RichText::new("OK").color(egui::Color32::GREEN)
                        } else {
                            RichText::new("Missing").color(egui::Color32::YELLOW)
                        };

                        row.col(|ui| {
                            if let Some(link) = date_link {
                                ui.add(Hyperlink::from_label_and_url(date.to_string(), &link))
                                    .on_hover_cursor(CursorIcon::PointingHand)
                                    .on_hover_text(format!("Open {link}"));
                            } else {
                                ui.label(date.to_string());
                            }
                        });
                        row.col(|ui| {
                            ui.label(status);
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

impl Display for ApodTableColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApodTableColumn::Date => write!(f, "Date"),
            ApodTableColumn::Title => write!(f, "Title"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, serde::Serialize, serde::Deserialize)]
pub enum StatusFilter {
    Ok,
    Failed,
    Warnings,
    Missing,
}

impl Display for StatusFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusFilter::Ok => write!(f, "OK"),
            StatusFilter::Failed => write!(f, "Failed"),
            StatusFilter::Warnings => write!(f, "Warnings"),
            StatusFilter::Missing => write!(f, "Missing"),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApodTableState {
    sort_column: Option<ApodTableColumn>,
    sort_ascending: bool,
    status_filter: Option<StatusFilter>,
    title_filter: String,
    #[serde(default, skip)]
    status_filter_popup_open: bool,
    #[serde(default, skip)]
    title_filter_popup_open: bool,
    #[serde(default, skip)]
    cached_sorted_dates: Vec<ApodDate>,
    #[serde(skip, default = "Instant::now")]
    data_last_update: Instant,
    #[serde(default, skip)]
    sort_clean: bool,
}

impl ApodTableState {
    pub fn sort(&mut self, data: &ApodData) {
        if self.data_last_update <= data.last_update() {
            self.sort_clean = false;
        }

        if self.sort_clean {
            return;
        } else {
            self.sort_clean = true;
        }

        self.cached_sorted_dates = ApodDate::iter_till_today().collect();

        if let Some(column) = self.sort_column {
            match column {
                ApodTableColumn::Date => {
                    self.cached_sorted_dates.sort_by(|a, b| {
                        let ord = a.cmp(b);
                        if self.sort_ascending {
                            ord
                        } else {
                            ord.reverse()
                        }
                    });
                }
                ApodTableColumn::Title => {
                    self.cached_sorted_dates.sort_by(|a, b| {
                        let a_title = data
                            .get_entry(*a)
                            .map(|entry| entry.title.as_str())
                            .unwrap_or_default();
                        let b_title = data
                            .get_entry(*b)
                            .map(|entry| entry.title.as_str())
                            .unwrap_or_default();
                        let ord = a_title.cmp(b_title);
                        if self.sort_ascending {
                            ord
                        } else {
                            ord.reverse()
                        }
                    });
                }
            }
        }

        if let Some(filter) = &self.status_filter {
            self.cached_sorted_dates.retain(|date| match filter {
                StatusFilter::Ok => data.get_entry(*date).is_some(),
                StatusFilter::Failed => data.get_error(*date).is_some(),
                StatusFilter::Warnings => data.get_warnings(*date).is_some(),
                StatusFilter::Missing => data.get_entry(*date).is_none(),
            })
        }

        if !self.title_filter.is_empty() {
            self.cached_sorted_dates.retain(|date| {
                data.get_entry(*date)
                    .map(|entry| entry.title.contains(&self.title_filter))
                    .unwrap_or(false)
            });
        }

        self.data_last_update = Instant::now();
    }

    pub fn entry_count(&self) -> usize {
        self.cached_sorted_dates.len()
    }

    pub fn get_date(&self, index: usize) -> Option<ApodDate> {
        self.cached_sorted_dates.get(index).copied()
    }

    pub fn sort_arrow(&self) -> &'static str {
        if self.sort_ascending {
            egui_phosphor::regular::SORT_ASCENDING
        } else {
            egui_phosphor::regular::SORT_DESCENDING
        }
    }

    pub fn render_simple_column_sort(&mut self, ui: &mut Ui, column: ApodTableColumn) {
        let label = column.to_string();
        let text = if self.sort_column == Some(column) {
            format!("{} {}", label, self.sort_arrow())
        } else {
            label.to_string()
        };

        let response = ui.add(
            egui::Label::new(RichText::new(text))
                .selectable(false)
                .sense(egui::Sense::click()),
        );

        if response.clicked() {
            if self.sort_column == Some(column) {
                self.sort_ascending = !self.sort_ascending;
            } else {
                self.sort_column = Some(column);
                self.sort_ascending = true;
            }
            self.sort_clean = false;
        }

        response.on_hover_cursor(CursorIcon::PointingHand);
    }

    pub fn render_status_column(&mut self, ui: &mut Ui) {
        let label = if let Some(filter) = &self.status_filter {
            match filter {
                StatusFilter::Ok => format!("Status {}", egui_phosphor::regular::CHECK_CIRCLE),
                StatusFilter::Failed => format!("Status {}", egui_phosphor::regular::X_CIRCLE),
                StatusFilter::Warnings => format!("Status {}", egui_phosphor::regular::WARNING),
                StatusFilter::Missing => format!("Status {}", egui_phosphor::regular::QUESTION),
            }
        } else {
            format!("Status {}", egui_phosphor::regular::FUNNEL)
        };

        let response = ui
            .add(
                egui::Label::new(RichText::new(label))
                    .selectable(false)
                    .sense(egui::Sense::click()),
            )
            .on_hover_cursor(CursorIcon::PointingHand);
        if response.clicked() {
            self.status_filter_popup_open = true;
        }

        let popup_id = ui.make_persistent_id("status_filter_popup");

        let mut filter = self.status_filter;
        Popup::from_response(&response)
            .open_bool(&mut self.status_filter_popup_open)
            .id(popup_id)
            .align(RectAlign::TOP_START)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                OptionEnumSelect::new(&mut filter, "status_filter_select").ui(ui);
            });
        if self.status_filter != filter {
            self.status_filter = filter;
            self.sort_clean = false;
            Popup::close_id(ui.ctx(), popup_id);
        }
    }

    pub fn render_title_column(&mut self, ui: &mut Ui) {
        let is_sorted_by_title = self.sort_column == Some(ApodTableColumn::Title);
        let label = if is_sorted_by_title {
            format!("Title {}", self.sort_arrow())
        } else {
            format!("Title {}", egui_phosphor::regular::SLIDERS)
        };

        let response = ui
            .add(
                egui::Label::new(RichText::new(label))
                    .selectable(false)
                    .sense(egui::Sense::click()),
            )
            .on_hover_cursor(CursorIcon::PointingHand);
        if response.clicked() {
            self.title_filter_popup_open = true;
        }

        let popup_id = ui.make_persistent_id("title_filter_popup");

        let mut sort_column = self.sort_column;
        let mut sort_ascending = self.sort_ascending;
        let mut title_filter = self.title_filter.clone();
        let mut changed = false;

        Popup::from_response(&response)
            .open_bool(&mut self.title_filter_popup_open)
            .id(popup_id)
            .align(RectAlign::TOP_START)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.horizontal(|ui| {
                    let sort_icon = if sort_column == Some(ApodTableColumn::Title) {
                        if sort_ascending {
                            egui_phosphor::regular::SORT_ASCENDING
                        } else {
                            egui_phosphor::regular::SORT_DESCENDING
                        }
                    } else {
                        egui_phosphor::regular::FUNNEL
                    };

                    if ui.button(sort_icon).clicked() {
                        if sort_column == Some(ApodTableColumn::Title) {
                            sort_ascending = !sort_ascending;
                        } else {
                            sort_column = Some(ApodTableColumn::Title);
                            sort_ascending = true;
                        }
                        changed = true;
                    }

                    if ui.text_edit_singleline(&mut title_filter).changed() {
                        changed = true;
                    }
                });
            });

        if self.sort_column != sort_column || self.sort_ascending != sort_ascending {
            self.sort_column = sort_column;
            self.sort_ascending = sort_ascending;
            self.sort_clean = false;
        }
        if self.title_filter != title_filter {
            self.title_filter = title_filter;
            self.sort_clean = false;
        }
    }
}

impl Default for ApodTableState {
    fn default() -> Self {
        Self {
            data_last_update: Instant::now(),
            sort_column: None,
            sort_ascending: true,
            status_filter: None,
            status_filter_popup_open: false,
            title_filter: String::new(),
            title_filter_popup_open: false,
            cached_sorted_dates: ApodDate::iter_till_today().collect(),
            sort_clean: false,
        }
    }
}
