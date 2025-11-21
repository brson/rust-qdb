mod sql_parser;
mod quote_filter;
mod markdown_gen;

use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    println!("Rust QDB Builder");
    println!("================");

    // Stage 1: Parse SQL dump
    println!("\n[1/4] Parsing SQL dump...");
    let sql_path = "qdb-src/chirpy.sql";
    let quotes = sql_parser::parse_sql_dump(sql_path)?;
    println!("  Found {} quotes", quotes.len());

    // Save full QDB to JSON
    let full_json_path = "qdb-src/full-qdb.json";
    let json_str = serde_json::to_string_pretty(&quotes)?;
    fs::write(full_json_path, json_str)?;
    println!("  Saved to {}", full_json_path);

    // Stage 2: Filter for Rust quotes
    println!("\n[2/4] Filtering Rust-related quotes...");
    let rust_quotes = quote_filter::filter_rust_quotes(&quotes);
    println!("  Found {} Rust quotes", rust_quotes.len());

    // Stage 3: Save Rust quotes JSON
    println!("\n[3/4] Saving Rust quotes JSON...");
    fs::create_dir_all("docs")?;
    let rust_json_path = "docs/rust-qdb.json";
    let rust_json_str = serde_json::to_string_pretty(&rust_quotes)?;
    fs::write(rust_json_path, rust_json_str)?;
    println!("  Saved to {}", rust_json_path);

    // Stage 4: Generate markdown
    println!("\n[4/4] Generating markdown...");
    let markdown = markdown_gen::generate_markdown(&rust_quotes);
    fs::write("rust-qdb.md", markdown)?;
    println!("  Saved to rust-qdb.md");

    println!("\nDone! Generated:");
    println!("  - {} (all quotes)", full_json_path);
    println!("  - {} (Rust quotes)", rust_json_path);
    println!("  - rust-qdb.md (markdown)");
    println!("\nView the web interface at docs/index.html");

    Ok(())
}
