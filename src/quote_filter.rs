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

    // Blacklist of quote IDs that are incorrectly tagged.
    let blacklist = [3109];

    quotes
        .iter()
        .filter(|quote| !blacklist.contains(&quote.id))
        .filter(|quote| is_rust_related(quote, &rust_pattern, &false_positive_pattern))
        .cloned()
        .collect()
}

fn is_rust_related(quote: &Quote, rust_pattern: &Regex, false_positive_pattern: &Regex) -> bool {
    // Check for #rust tag
    if quote.tags.iter().any(|tag| tag.to_lowercase() == "rust") {
        return true;
    }

    // Check body for rust keyword that's not part of a false positive.
    // We need to check if ANY match of "rust" is standalone, not just if there are no false positives.
    if has_standalone_rust(&quote.body, rust_pattern, false_positive_pattern) {
        return true;
    }

    // Check notes for rust keyword
    if let Some(notes) = &quote.notes {
        if has_standalone_rust(notes, rust_pattern, false_positive_pattern) {
            return true;
        }
    }

    false
}

/// Checks if text contains "rust" that is not part of "trust", "frustrat", or "rusty".
fn has_standalone_rust(text: &str, rust_pattern: &Regex, false_positive_pattern: &Regex) -> bool {
    for rust_match in rust_pattern.find_iter(text) {
        let start = rust_match.start();
        let end = rust_match.end();

        // Check if this "rust" match is part of a false positive by looking at surrounding context
        let context_start = start.saturating_sub(5);
        let context_end = (end + 5).min(text.len());
        let context = &text[context_start..context_end];

        // If the context around this specific match doesn't contain false positives, it's valid
        if !false_positive_pattern.is_match(context) {
            return true;
        }
    }
    false
}
