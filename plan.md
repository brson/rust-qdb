# Plan brief

The raw mozilla QDB is in qdb-src.

We need to extract the contents to a more useful JSON format,
stored in qdb-src/full-qdb.json;
then filter out only the Rust content,
store it in docs/rust-qdb.json,
then write the Rust quotes to rust-qdb.md;
then create a docs/index.html with various filtering and searching options.

A brief description of the dump is in qdb-src/readme.txt.
I don't remember the dump format offhand and a big part of the project
will be writing the conversion tool.

# Detailed Plan

## Data Source

SQL dump at qdb-src/chirpy.sql contains quotes table with structure:

```sql
CREATE TABLE `quotes` (
  `id` int UNSIGNED NOT NULL,
  `body` text CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL,
  `notes` text CHARACTER SET utf8 COLLATE utf8_general_ci,
  `rating` int NOT NULL DEFAULT '0',
  `votes` int UNSIGNED NOT NULL DEFAULT '0',
  `submitted` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `approved` tinyint UNSIGNED NOT NULL DEFAULT '0',
  `flagged` tinyint UNSIGNED NOT NULL DEFAULT '0',
  `score` double UNSIGNED NOT NULL DEFAULT '0'
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb3 COLLATE=utf8_unicode_ci;
```

Additional tables: tags (stored separately, referenced by quote ID).

## Pipeline Stages

### 1. SQL Parser
Extract all quotes from INSERT statements into qdb-src/full-qdb.json.
Handle escaped quotes, newlines, multi-line INSERT statements.

### 2. Rust Filter
Filter quotes containing Rust programming language references:
- Must have `#rust` tag OR
- Body contains "rust" (case-insensitive) but NOT as part of "trust", "frustration", "rusty"
- Context-aware filtering to avoid false positives

Estimated ~30-50 Rust quotes from initial scan.

### 3. Markdown Generator
Create rust-qdb.md with readable format:
- Quote ID and metadata
- Formatted IRC conversation
- Rating and vote count
- Tags

### 4. HTML Interface
Build docs/index.html with:
- Load quotes from docs/rust-qdb.json
- Client-side search/filter
- Sort by: rating, date, votes
- Display metadata and tags
- Responsive design

## Implementation

Build Rust CLI tool with modules:
- sql_parser: Parse SQL INSERT statements
- quote_filter: Identify Rust-related quotes
- markdown_gen: Generate markdown output
- html_gen: Generate HTML template with embedded JS

Dependencies needed:
- serde/serde_json for JSON handling
- regex for SQL parsing
- chrono for date handling
