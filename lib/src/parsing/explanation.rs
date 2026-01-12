use crate::parsing::ParseError;
use scraper::{Html, Selector};

pub fn parse_explanation(doc: &Html) -> Result<String, ParseError> {
    let explanation = extract_explanation_from_td(doc)
        .or_else(|| extract_explanation_from_p(doc))
        .or_else(|| extract_explanation_from_text(doc))
        .map(|s| clean_explanation(&s))
        .ok_or(ParseError::ExplanationNotFound)?;
    Ok(explanation)
}

fn extract_explanation_from_td(document: &Html) -> Option<String> {
    let td_sel = Selector::parse("td").unwrap();

    let td = document
        .select(&td_sel)
        .find(|el| el.text().collect::<String>().contains("Explanation:"))?;

    let text: String = td.text().collect();
    let text = text.split("Explanation:").nth(1)?.trim();

    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn extract_explanation_from_p(document: &Html) -> Option<String> {
    let p_sel = Selector::parse("p").unwrap();

    let p = document
        .select(&p_sel)
        .find(|el| el.text().collect::<String>().contains("Explanation:"))?;

    let text: String = p.text().collect();
    let text = text.split("Explanation:").nth(1)?.trim();

    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn extract_explanation_from_text(document: &Html) -> Option<String> {
    let full_text = document.root_element().text().collect::<String>();
    let text = full_text.split("Explanation:").nth(1)?.trim();

    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn clean_explanation(text: &str) -> String {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");

    let delimiters = [
        "Tomorrow's picture",
        "Tomorrow's Picture",
        "Authors & editors",
        "Author:",
        "We keep an archive file.",
    ];

    let mut result = text.trim();
    for delim in delimiters {
        if let Some(pos) = result.find(delim) {
            result = &result[..pos];
            break;
        }
    }

    result.trim().to_string()
}
