use crate::sql_parser::Quote;

/// Generates markdown document with all Rust quotes.
pub fn generate_markdown(quotes: &[Quote]) -> String {
    let mut md = String::new();

    md.push_str("# Rust Quotes from Mozilla QDB\n\n");
    md.push_str(&format!("Total quotes: {}\n\n", quotes.len()));
    md.push_str("---\n\n");

    for quote in quotes {
        md.push_str(&format_quote(quote));
        md.push_str("\n---\n\n");
    }

    md
}

fn format_quote(quote: &Quote) -> String {
    let mut output = String::new();

    // Header with ID and metadata
    output.push_str(&format!("## Quote #{}\n\n", quote.id));

    // Quote body
    output.push_str("```\n");
    output.push_str(&quote.body);
    if !quote.body.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```\n\n");

    // Metadata
    output.push_str(&format!("- **Rating:** {}", quote.rating));
    if quote.votes > 0 {
        output.push_str(&format!(" ({} votes)", quote.votes));
    }
    output.push('\n');

    output.push_str(&format!("- **Score:** {:.2}\n", quote.score));
    output.push_str(&format!("- **Submitted:** {}\n", quote.submitted));
    output.push_str(&format!("- **Approved:** {}\n", quote.approved));

    // Tags
    if !quote.tags.is_empty() {
        output.push_str("- **Tags:** ");
        output.push_str(&quote.tags.join(", "));
        output.push('\n');
    }

    // Notes
    if let Some(notes) = &quote.notes {
        if !notes.is_empty() {
            output.push_str(&format!("\n**Notes:** {}\n", notes));
        }
    }

    output
}
