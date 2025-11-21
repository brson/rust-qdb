# Plan brief

The raw mozilla QDB is in qdb-src.

We need to extract the contents to a more useful JSON format,
stored in qdb-src/full-qdb.json;
then filter out only the Rust content,
store it in www/rust-qdb.json,
then write the Rust quotes to rust-qdb.md;
then create a www/index.html with various filtering and searching options.