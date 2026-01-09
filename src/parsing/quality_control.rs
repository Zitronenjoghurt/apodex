use crate::ApodEntry;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QualityWarning {
    ContainsHtml,
    EmptyField,
    LeadingWhitespace,
    MultiWhitespace,
    TrailingWhitespace,
    TitleMultiline,
}

pub fn quality_control(entry: &ApodEntry) -> HashSet<QualityWarning> {
    let mut warnings = HashSet::new();
    quality_control_title(&entry.title, &mut warnings);
    warnings
}

fn quality_control_string(string: &str, warnings: &mut HashSet<QualityWarning>) {
    if string.contains('<') || string.contains('>') {
        warnings.insert(QualityWarning::ContainsHtml);
    }

    if string.is_empty() {
        warnings.insert(QualityWarning::EmptyField);
    }

    if string.starts_with(char::is_whitespace) {
        warnings.insert(QualityWarning::LeadingWhitespace);
    }

    if string.ends_with(char::is_whitespace) {
        warnings.insert(QualityWarning::TrailingWhitespace);
    }

    if has_multiple_whitespaces(string) {
        warnings.insert(QualityWarning::MultiWhitespace);
    }
}

fn quality_control_title(title: &str, warnings: &mut HashSet<QualityWarning>) {
    quality_control_string(title, warnings);

    if title.lines().count() > 1 {
        warnings.insert(QualityWarning::TitleMultiline);
    }
}

fn has_multiple_whitespaces(s: &str) -> bool {
    let mut prev_whitespace = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if prev_whitespace {
                return true;
            }
            prev_whitespace = true;
        } else {
            prev_whitespace = false;
        }
    }
    false
}
