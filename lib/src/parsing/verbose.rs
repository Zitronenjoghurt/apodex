use crate::date::ApodDate;
use crate::parsing::quality_control::{quality_control, QualityWarning};
use crate::parsing::{parse_html, ParseError};
use crate::ApodEntry;
use std::collections::HashSet;

#[derive(Debug)]
pub struct VerboseParseResult {
    pub entry: Option<ApodEntry>,
    pub warnings: HashSet<QualityWarning>,
    pub error: Option<ParseError>,
}

pub fn parse_html_verbose(date: ApodDate, html: &str) -> VerboseParseResult {
    match parse_html(date, html) {
        Ok(entry) => VerboseParseResult {
            warnings: quality_control(&entry),
            entry: Some(entry),
            error: None,
        },
        Err(error) => VerboseParseResult {
            entry: None,
            warnings: HashSet::new(),
            error: Some(error),
        },
    }
}
