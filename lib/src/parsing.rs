use crate::date::ApodDate;
use crate::ApodEntry;
use scraper::Html;

mod explanation;
pub mod quality_control;
mod title;
pub mod verbose;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParseError {
    #[error("Explanation not found")]
    ExplanationNotFound,
    #[error("Title not found")]
    TitleNotFound,
}

pub fn parse_html(date: ApodDate, html: &str) -> Result<ApodEntry, ParseError> {
    let doc = Html::parse_document(html);

    let title = title::parse_title(&doc)?;
    let explanation = explanation::parse_explanation(&doc)?;

    Ok(ApodEntry {
        date,
        title,
        explanation,
    })
}
