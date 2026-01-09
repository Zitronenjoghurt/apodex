use crate::parsing::ParseError;
use scraper::{Html, Selector};

pub fn parse_title(doc: &Html) -> Result<String, ParseError> {
    let title = extract_from_center(doc)
        .or_else(|| extract_from_title_tag(doc))
        .map(|t| t.trim().to_string())
        .ok_or(ParseError::TitleNotFound)?;
    Ok(clean_title(&title))
}

fn extract_from_center(document: &Html) -> Option<String> {
    let center_sel = Selector::parse("center").unwrap();
    let bold_sel = Selector::parse("b").unwrap();

    let centers: Vec<_> = document.select(&center_sel).collect();
    let idx = if centers.len() == 2 { 0 } else { 1 };

    let center = centers.get(idx)?;
    let bold = center.select(&bold_sel).next()?;

    Some(bold.text().collect::<String>())
}

fn extract_from_title_tag(document: &Html) -> Option<String> {
    let title_sel = Selector::parse("title").unwrap();

    let title = document.select(&title_sel).next()?;
    let text: String = title.text().collect();

    text.split(" - ").last().map(String::from)
}

fn clean_title(raw: &str) -> String {
    raw.lines()
        .next()
        .unwrap_or("")
        .split("Credit:")
        .next()
        .unwrap_or("")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
