use crate::ApodEntry;
use chrono::Datelike;
use scraper::Html;

pub mod quality_control;
mod title;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Title not found")]
    TitleNotFound,
}

pub fn parse_html(date: chrono::NaiveDate, html: &str) -> Result<ApodEntry, ParseError> {
    let doc = Html::parse_document(html);

    let title = title::parse_title(&doc)?;

    Ok(ApodEntry {
        year: date.year() as u16,
        month: date.month() as u8,
        day: date.day() as u8,
        title,
    })
}
