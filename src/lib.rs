pub mod rf;
pub mod help;
pub mod cells;
pub mod style;
pub mod errors;

use std::fs::{ File, } ;
use std::path::{ Path, };
use std::io::{BufReader, Error};
use std::process::exit;

use crate::style::{ ANSIStyle, };
use crate::errors::{ ErrorMsg, };
use crate::rf::{ ReadableFile, };


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


pub fn read_file(path: &str) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    Ok(buffer)
}


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