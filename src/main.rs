mod sql_parser;
mod quote_filter;
mod markdown_gen;
mod html_gen;

use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    println!("Rust QDB Builder");
    println!("================");

    // Stage 1: Parse SQL dump
    println!("\n[1/5] Parsing SQL dump...");
    let sql_path = "qdb-src/chirpy.sql";
    let quotes = sql_parser::parse_sql_dump(sql_path)?;
    println!("  Found {} quotes", quotes.len());

    // Save full QDB to JSON
    let full_json_path = "qdb-src/full-qdb.json";
    let json_str = serde_json::to_string_pretty(&quotes)?;
    fs::write(full_json_path, json_str)?;
    println!("  Saved to {}", full_json_path);

    // Stage 2: Filter for Rust quotes
    println!("\n[2/5] Filtering Rust-related quotes...");
    let rust_quotes = quote_filter::filter_rust_quotes(&quotes);
    println!("  Found {} Rust quotes", rust_quotes.len());

    // Stage 3: Save Rust quotes JSON
    println!("\n[3/5] Saving Rust quotes JSON...");
    fs::create_dir_all("www")?;
    let rust_json_path = "www/rust-qdb.json";
    let rust_json_str = serde_json::to_string_pretty(&rust_quotes)?;
    fs::write(rust_json_path, rust_json_str)?;
    println!("  Saved to {}", rust_json_path);

    // Stage 4: Generate markdown
    println!("\n[4/5] Generating markdown...");
    let markdown = markdown_gen::generate_markdown(&rust_quotes);
    fs::write("rust-qdb.md", markdown)?;
    println!("  Saved to rust-qdb.md");

    // Stage 5: Generate HTML
    println!("\n[5/5] Generating HTML interface...");
    let html = html_gen::generate_html();
    fs::write("www/index.html", html)?;
    println!("  Saved to www/index.html");

    println!("\nDone! Generated:");
    println!("  - {} (all quotes)", full_json_path);
    println!("  - {} (Rust quotes)", rust_json_path);
    println!("  - rust-qdb.md (markdown)");
    println!("  - www/index.html (web interface)");

    Ok(())
}
