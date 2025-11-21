use crate::sql_parser::Quote;
use regex::Regex;

/// Filters quotes for Rust-related content.
///
/// A quote is considered Rust-related if it has a #rust tag OR
/// contains "rust" (case-insensitive) but NOT as part of "trust",
/// "frustration", or "rusty".
pub fn filter_rust_quotes(quotes: &[Quote]) -> Vec<Quote> {
    let rust_pattern = Regex::new(r"(?i)\brust\b").unwrap();
    let false_positive_pattern = Regex::new(r"(?i)\b(trust|frustrat|rusty)\b").unwrap();

    quotes
        .iter()
        .filter(|quote| is_rust_related(quote, &rust_pattern, &false_positive_pattern))
        .cloned()
        .collect()
}

fn is_rust_related(quote: &Quote, rust_pattern: &Regex, false_positive_pattern: &Regex) -> bool {
    // Check for #rust tag
    if quote.tags.iter().any(|tag| tag.to_lowercase() == "rust") {
        return true;
    }

    // Check body for rust keyword
    if rust_pattern.is_match(&quote.body) {
        // Exclude false positives
        if !false_positive_pattern.is_match(&quote.body) {
            return true;
        }
    }

    // Check notes for rust keyword
    if let Some(notes) = &quote.notes {
        if rust_pattern.is_match(notes) && !false_positive_pattern.is_match(notes) {
            return true;
        }
    }

    false
}
