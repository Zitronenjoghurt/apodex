use crate::date::ApodDate;
use crate::ApodEntry;
use chrono::Datelike;
use scraper::Html;

pub mod quality_control;
mod title;
pub mod verbose;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Title not found")]
    TitleNotFound,
}

pub fn parse_html(date: ApodDate, html: &str) -> Result<ApodEntry, ParseError> {
    let doc = Html::parse_document(html);

    let title = title::parse_title(&doc)?;

    Ok(ApodEntry { date, title })
}
