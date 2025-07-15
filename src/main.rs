//! # lenz
//!
//! `lenz` is a command-line utility for searching and Browse files.
//! It allows users to quickly find specific text patterns within files,
//! with options for case-insensitive matching, counting occurrences,
//! and different display modes.
//!
//! *GitHub repo: <https://github.com/mimseyedi/lenz>*


use std::env;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    lenz::run(args);
}