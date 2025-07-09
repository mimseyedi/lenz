//! # lenz
//!
//! `lenz` is a command-line utility for searching and Browse files.
//! It allows users to quickly find specific text patterns within files,
//! with options for case-insensitive matching, counting occurrences,
//! and different display modes.
//!
//! ## How to use:
//!
//! You can run `lenz` from your terminal:
//!
//! ```bash
//! lenz "query" /path/to/your/file.txt /path/to/another/file.txt ...
//! ```
//!
//! ## Options:
//!
//! `lenz` uses single flag options to display output in different formats:
//!
//! ### For ignoring case-sensitive search -> (-i, --ignore-case):
//!
//! ```bash
//! lenz "query" /path/to/your/file.txt -i
//! ```
//!
//! ### For Page-view -> (-p, --page-view):
//!
//! ```bash
//! lenz "query" /path/to/your/file.txt -p
//! ```
//!
//! ### For counting matches in files -> (-c, --count):
//!
//! ```bash
//! lenz "query" /path/to/your/file.txt -c
//! ```
//!
//! ### You can use (-h, --help) for getting help about `lenz`:
//!
//! ```bash
//! lenz --help
//! ```
//!
//! ### And also for get `lenz` version -> (-v, --version):
//!
//! ```bash
//! lenz --version
//! ```
//!
//! *GitHub repo: https://github.com/mimseyedi/lenz*


// Defining modules in the root
pub mod rf;
pub mod help;
pub mod cells;
pub mod style;
pub mod errors;

// Interiors
use std::fs::{ File, } ;
use std::path::{ Path, };
use std::io::{BufReader, Error};
use std::process::exit;
// crate
use crate::style::{ ANSIStyle, };
use crate::errors::{ ErrorMsg, };
use crate::rf::{ ReadableFile, };


/// The task of this function is to count a substring (query) in a string.
/// The search can be performed case-sensitively or case-insensitively.
///
/// # Arguments
///
/// * `query` - The string to search for.
/// * `text` - The string within which to search.
/// * `ignore_case` - A boolean indicating whether the search should be case-insensitive.
///
/// # Returns
///
/// An `Option<usize>` which is `Some(count)` if the query is found at least once,
/// where `count` is the total number of occurrences. Returns `None` if the query
/// is not found.
pub fn count_query(query: &str, text: &str, ignore_case: bool) -> Option<usize> {
    let mut t = text.to_string();
    let mut q = query.to_string();
    if ignore_case {
        t = text.to_lowercase();
        q = q.to_lowercase();
    }
    let c: usize = t.matches(&q).count();
    if c > 0 { Some(c) } else { None }
}


/// The task of this function is to find the index of the beginning and end
/// of each specified substring (query) in a string.
/// The search can be performed case-sensitively or case-insensitively.
///
/// # Arguments
///
/// * `query` - The string to search for.
/// * `text` - The string within which to search.
/// * `ignore_case` - A boolean indicating whether the search should be case-insensitive.
///
/// # Returns
///
/// An `Option<Vec<(usize, usize)`. If the substring exists in the string, a `Some(Vec)`
/// consisting of the start and end indices is returned, otherwise `None`.
pub fn find_all(query: &str, text: &str, ignore_case: bool) -> Option<Vec<(usize, usize)>> {
    let (t, q);
    let matches: Vec<(usize, &str)>;
    if ignore_case {
        t = text.to_lowercase();
        q = query.to_lowercase();
        matches = t.match_indices(&q).collect();
    } else {
        matches = text.match_indices(query).collect();
    }
    let vec: Vec<(usize, usize)> = matches.into_iter()
                                          .map(|(x, m)| (x, m.len()))
                                          .collect();
    if !vec.is_empty() { Some(vec) } else { None }
}


/// The task of this function is to open and read a file.
/// For greater efficiency, this function uses a `BufReader`
/// instead of returning the entire contents of a file.
///
/// # Arguments
///
/// * `path` - The path to the file to be opened.
///
/// # Returns
///
/// A `Result<BufReader<File>, Error>`, which returns an `Ok(BufReader)`
/// if the file had no problems opening. Otherwise, an `Err` will be returned.
///
/// # Errors
///
/// If there is a problem opening the file,
/// the `?` operator will return an `Err(e)` containing the relevant message.
pub fn read_file(path: &str) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    Ok(buffer)
}


/// The task of this function is to display the **contents(matches)** of each file
/// using the **methods** implemented for and accessed by `ReadableFiles`.
///
/// # Arguments
///
/// * `files` - A **vector** containing `ReadableFiles` to display.
/// * `mode` - Display mode -> `0` for counting, `1` for cell display, `2` for page-based display.
///
/// # Panics
///
/// If the display mode is set incorrectly,
/// an uncontrollable error (panic) will be thrown.
fn show(files: Vec<ReadableFile>, mode: usize) {
    for file in files {
        match mode {
            0 => file.print_count(),
            1 => file.print_cells(),
            2 => file.print_page(),
            _ => panic!("Invalid mode! You must select between (0, 1, 2)."),
        }
    }
}


/// The task of this function is to **validate** a file.
/// The entered path is acceptable if it **already exists** and is a **file**.
///
/// # Arguments
///
/// * `file` - The **path** to a file to be validated.
///
/// # Returns
///
/// A `Result<String, ErrorMsg>`, which returns an `Ok(file)`
/// for confirmation and an `Err(ErrorMsg)` for an error.
fn check_file(file: &str) -> Result<String, ErrorMsg> {
    let f_path = Path::new(file);
    if !f_path.exists() {
        return Err(
            ErrorMsg::new(
                format!(
                    "File {}{}'{}'{} does not exist.",
                    ANSIStyle::Italic.as_str(),
                    ANSIStyle::FGWhite.as_str(),
                    file,
                    ANSIStyle::Reset.as_str(),
                )
            )
        )
    }
    if !f_path.is_file() {
        return Err(
            ErrorMsg::new(
                format!(
                    "Path {}{}'{}'{} is not a file.",
                    ANSIStyle::Italic.as_str(),
                    ANSIStyle::FGWhite.as_str(),
                    file,
                    ANSIStyle::Reset.as_str(),
                )
            )
        )
    }
    Ok(file.to_string())
}


/// The task of this function is to create a ReadableFile if it qualifies.
/// The file is first checked and if there is a **match**, it will be created.
///
/// # Arguments
///
/// * `file` - The path to the file to be searched.
/// * `query` - The string to search for.
/// * `ignore_case` - A boolean indicating whether the search should be case-insensitive.
///
/// # Returns
///
/// A `Result<ReadableFile, ErrorMsg>`, which returns an `Ok(ReadableFile)`
/// for confirmation and an `Err(ErrorMsg)` for an error.
fn create_rf<'a>(file: &'a str, query: &'a str, ignore_case: bool) -> Result<ReadableFile, ErrorMsg> {
    match check_file(file) {
        Ok(f) => {
            let rf = ReadableFile::new(
                f,
                query.to_string(),
                ignore_case,
            );
            Ok(rf)
        },
        Err(e) => Err(e)
    }
}


/// This function is responsible for reporting files that had problems
/// and could not be opened and were thrown into the trash.
/// At the end of each output, a report of the corrupted files will be given.
///
/// # Arguments
///
/// * `gr` - A referenced to a vector of errors (`&Vec<ErrorMsg>`) that should be checked and displayed.
fn garbage_report(gr: &Vec<ErrorMsg>) {
    let gr_len = gr.len();
    println!("{}",
        format!(
            "\n{}{}* Errors Report: {}",
            ANSIStyle::Bold.as_str(),
            ANSIStyle::FGRed.as_str(),
            ANSIStyle::Reset.as_str(),
        )
    );
    let mut br;
    for (i, e) in gr.iter().enumerate() {
        if gr_len == 1 {
            br = "└──";
        } else {
            if i == gr_len - 1 {
                br = "└──";
            } else {
                br = "├──";
            }
        }
        let e_msg = e.msg();
        println!(
            "{}{}[{}]{} {}",
            ANSIStyle::FGRed.as_str(),
            br,
            i + 1,
            ANSIStyle::Reset.as_str(),
            e_msg,
        );
    }
}


/// The task of this function, which is considered the **main** execution function,
/// is to manage arguments, build the required structures,
/// manage options, manage corrupted files,
/// and finally display the output based on the arguments entered by the user.
///
/// # Arguments
///
/// * `args` - Environment arguments read from the terminal.
///
/// # Returns
///
/// This function does not return anything and is just an executor.
pub fn run(args: Vec<String>) {
    let mut mode = 1;
    let alen = args.len();
    if alen == 1 && args[0].starts_with("-") {
        let opt = &args[0];
        match opt.to_string().as_str() {
            "-h" | "--help" => {
                help::get_help().show();
                exit(0);
            },
            "-v" | "--version" => {
                help::print_version();
                exit(0);
            },
            _ => {
                let msg = String::from(
                    "Only two options '-h, --help' and '-v, --version' are accepted without any arguments."
                );
                ErrorMsg::new(msg).raise();
                exit(1);
            }
        }
    }
    if alen < 2 {
        let msg = String::from(
            "At least two arguments are expected."
        );
        ErrorMsg::new(msg).raise();
        exit(1);
    }
    let mut gr: Vec<ErrorMsg> = Vec::new();
    let query = &args[0];
    let files;
    let mut opt = "";
    if args[alen - 1].starts_with("-") {
        opt = &args[alen - 1];
        files = &args[1..alen - 1];
    } else {
        files = &args[1..];
    }

    let mut output: Vec<ReadableFile> = Vec::new();

    if opt.is_empty() {
        for file in files {
            match create_rf(file, query, false) {
                Ok(f) => output.push(f),
                Err(e) => gr.push(e),
            }
        }
    } else {
        match opt {
            "-c" | "--count" => {
                for file in files {
                    match create_rf(file, query, false) {
                        Ok(f) => output.push(f),
                        Err(e) => gr.push(e),
                    }
                }
                mode = 0;
            },
            "-p" | "--page-view" => {
                for file in files {
                    match create_rf(file, query, false) {
                        Ok(f) => output.push(f),
                        Err(e) => gr.push(e),
                    }
                }
                mode = 2;
            },
            "-i" | "--ignore-case" => {
                for file in files {
                    match create_rf(file, query, true) {
                        Ok(f) => output.push(f),
                        Err(e) => gr.push(e),
                    }
                }
                mode = 1;
            },
            _ => {
                ErrorMsg::new(
                    format!(
                        "This option is not valid: {}{}'{}'{}.",
                        ANSIStyle::Italic.as_str(),
                        ANSIStyle::FGWhite.as_str(),
                        opt,
                        ANSIStyle::Reset.as_str(),
                    )
                ).raise();
                exit(1);
            }
        }
    }
    show(output, mode);
    if !gr.is_empty() {
        garbage_report(&gr);
    }
    exit(0);
}