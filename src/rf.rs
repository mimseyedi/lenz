use std::io::{ BufRead, };
use crate::{ read_file, count_query, };
use crate::cells::{ Cell, };
use crate::style::{ ANSIStyle, };


pub struct ReadableFile {
    path: String,
    query: String,
    ignore_case: bool,
}


impl ReadableFile {
    pub fn new(path: String, query: String, ignore_case: bool) -> Self {
        Self { path, query, ignore_case, }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn print_cells(&self) {
        println!("\n{}\n", self.get_head());
        if let Ok(buffer) = read_file(&self.path) {
            let mut n: usize = 1;
            let q = &self.query;
            for line in buffer.lines() {
                if let Ok(line) = line {
                    let cell = Cell::new(
                        q.to_string(),
                        line,
                        n,
                        1,
                    );
                    n += 1;
                    if cell.check_line(self.ignore_case) {
                        cell.print(None, self.ignore_case);
                    } else {
                        drop(cell);
                        continue;
                    }
                }
            }
        }
    }

    pub fn print_count(&self) {
        println!("{}", self.get_head());
    }

    pub fn print_page(&self) {
        println!("\n{}\n", self.get_head());
        if let Ok(buffer) = read_file(&self.path) {
            let mut n: usize = 1;
            let q = &self.query;
            for line in buffer.lines() {
                if let Ok(line) = line {
                    Cell::new(
                        q.to_string(),
                        line,
                        n,
                        1,
                    ).print(Some(true), self.ignore_case);
                }
                n += 1;
            }
        }
    }

    fn get_head(&self) -> String {
        format!(
            "{}- Filename: {}{}'{}'{} {}[{}]{}",
            ANSIStyle::Bold.as_str(),
            ANSIStyle::Italic.as_str(),
            ANSIStyle::FGWhite.as_str(),
            self.path,
            ANSIStyle::Reset.as_str(),
            ANSIStyle::FGGreen.as_str(),
            self.get_count(&self.query),
            ANSIStyle::Reset.as_str(),
        )
    }

    fn get_count(&self, query: &str) -> usize {
        let mut c: usize = 0;
        if let Ok(buffer) = read_file(&self.path) {
            for line in buffer.lines() {
                if let Ok(line) = line {
                    if let Some(count) = count_query(query, &line, self.ignore_case) {
                        c += count;
                    }
                }
            }
        }
        c
    }
}