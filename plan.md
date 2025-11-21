# Plan brief

The raw mozilla QDB is in qdb-src.

We need to extract the contents to a more useful JSON format,
stored in qdb-src/full-qdb.json;
then filter out only the Rust content,
store it in www/rust-qdb.json,
then write the Rust quotes to rust-qdb.md;
then create a www/index.html with various filtering and searching options.

A brief description of the dump is in qdb-src/readme.txt.
I don't remember the dump format offhand and a big part of the project
will be writing the conversion tool.
