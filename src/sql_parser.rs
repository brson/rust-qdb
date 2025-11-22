use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: u32,
    pub body: String,
    pub notes: Option<String>,
    pub rating: i32,
    pub votes: u32,
    pub submitted: String,
    pub approved: bool,
    pub flagged: bool,
    pub score: f64,
    pub tags: Vec<String>,
}

/// Parses the SQL dump and extracts all quotes.
pub fn parse_sql_dump(path: &str) -> Result<Vec<Quote>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read SQL file: {}", path))?;

    let mut quotes = Vec::new();
    let mut tags_map: HashMap<u32, Vec<String>> = HashMap::new();

    // Parse quotes table
    let quotes_data = extract_table_data(&content, "quotes")?;
    for row in quotes_data {
        if let Some(quote) = parse_quote_row(&row) {
            quotes.push(quote);
        }
    }

    // Parse tags table to get tag_id -> tag_name mapping
    let mut tag_names: HashMap<u32, String> = HashMap::new();
    let tags_data = extract_table_data(&content, "tags")?;
    for row in tags_data {
        if let Some((tag_id, tag_name)) = parse_tag_name_row(&row) {
            tag_names.insert(tag_id, tag_name);
        }
    }

    // Parse quote_tag junction table to get quote_id -> tag_id mapping
    let quote_tag_data = extract_table_data(&content, "quote_tag")?;
    for row in quote_tag_data {
        if let Some((quote_id, tag_id)) = parse_quote_tag_row(&row) {
            if let Some(tag_name) = tag_names.get(&tag_id) {
                tags_map.entry(quote_id).or_default().push(tag_name.clone());
            }
        }
    }

    // Associate tags with quotes
    for quote in &mut quotes {
        if let Some(tags) = tags_map.get(&quote.id) {
            quote.tags = tags.clone();
        }
    }

    Ok(quotes)
}

/// Extracts INSERT statement data for a specific table.
fn extract_table_data(content: &str, table_name: &str) -> Result<Vec<String>> {
    let mut rows = Vec::new();
    let insert_pattern = format!("INSERT INTO `{}`", table_name);
    let mut in_insert = false;

    for line in content.lines() {
        if line.starts_with(&insert_pattern) && line.contains("VALUES") {
            in_insert = true;
            // Extract the VALUES part
            if let Some(values_start) = line.find("VALUES") {
                let values_part = &line[values_start + 6..].trim();
                if !values_part.is_empty() {
                    let parsed_rows = parse_value_tuples(values_part);
                    rows.extend(parsed_rows);
                }
            }
        } else if in_insert && line.starts_with('(') {
            // This is a continuation line with quote data
            let parsed_rows = parse_value_tuples(line);
            rows.extend(parsed_rows);
        } else if in_insert && line.starts_with("--") {
            // End of INSERT statements (comment or new section)
            in_insert = false;
        } else if in_insert && line.trim().ends_with(';') {
            // End of INSERT statement
            let parsed_rows = parse_value_tuples(line);
            rows.extend(parsed_rows);
            in_insert = false;
        } else if in_insert && !line.trim().is_empty() && !line.starts_with("--") {
            // Another data row
            let parsed_rows = parse_value_tuples(line);
            rows.extend(parsed_rows);
        }
    }

    Ok(rows)
}

/// Parses comma-separated value tuples from SQL INSERT statement.
fn parse_value_tuples(values: &str) -> Vec<String> {
    let mut rows = Vec::new();
    let mut current_row = String::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut paren_depth = 0;

    for ch in values.chars() {
        if escaped {
            current_row.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' if in_string => {
                escaped = true;
                current_row.push(ch);
            }
            '\'' => {
                in_string = !in_string;
                current_row.push(ch);
            }
            '(' if !in_string => {
                paren_depth += 1;
                if paren_depth == 1 {
                    current_row.clear();
                } else {
                    current_row.push(ch);
                }
            }
            ')' if !in_string => {
                paren_depth -= 1;
                if paren_depth == 0 {
                    rows.push(current_row.clone());
                    current_row.clear();
                } else {
                    current_row.push(ch);
                }
            }
            _ => {
                current_row.push(ch);
            }
        }
    }

    rows
}

/// Parses a quote row tuple.
fn parse_quote_row(row: &str) -> Option<Quote> {
    let fields = parse_sql_fields(row);
    if fields.len() < 9 {
        return None;
    }

    Some(Quote {
        id: fields[0].parse().ok()?,
        body: unescape_sql_string(&fields[1]),
        notes: if fields[2] == "NULL" {
            None
        } else {
            Some(unescape_sql_string(&fields[2]))
        },
        rating: fields[3].parse().ok()?,
        votes: fields[4].parse().ok()?,
        submitted: fields[5].trim_matches('\'').to_string(),
        approved: fields[6] != "0",
        flagged: fields[7] != "0",
        score: fields[8].parse().ok()?,
        tags: Vec::new(),
    })
}

/// Parses a tag name row tuple from the tags table (tag_id, tag_name).
fn parse_tag_name_row(row: &str) -> Option<(u32, String)> {
    let fields = parse_sql_fields(row);
    if fields.len() < 2 {
        return None;
    }

    let tag_id = fields[0].parse().ok()?;
    let tag_name = unescape_sql_string(&fields[1]);

    Some((tag_id, tag_name))
}

/// Parses a quote_tag junction row tuple (quote_id, tag_id).
fn parse_quote_tag_row(row: &str) -> Option<(u32, u32)> {
    let fields = parse_sql_fields(row);
    if fields.len() < 2 {
        return None;
    }

    let quote_id = fields[0].parse().ok()?;
    let tag_id = fields[1].parse().ok()?;

    Some((quote_id, tag_id))
}

/// Parses comma-separated fields from a SQL row tuple.
fn parse_sql_fields(row: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_string = false;
    let mut escaped = false;

    for ch in row.chars() {
        if escaped {
            current_field.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' if in_string => {
                escaped = true;
                current_field.push(ch);
            }
            '\'' => {
                in_string = !in_string;
                current_field.push(ch);
            }
            ',' if !in_string => {
                fields.push(current_field.trim().to_string());
                current_field.clear();
            }
            _ => {
                current_field.push(ch);
            }
        }
    }

    if !current_field.is_empty() {
        fields.push(current_field.trim().to_string());
    }

    fields
}

/// Unescapes SQL string literals.
fn unescape_sql_string(s: &str) -> String {
    let trimmed = s.trim_matches('\'');
    trimmed
        .replace("\\'", "'")
        .replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\\\", "\\")
}
