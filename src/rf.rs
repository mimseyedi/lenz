//! This module is related to the structure of the files that can be searched.
//! Any file that is `readable` can be searched and the output is displayed using matching `Cells`.


use std::io::{ BufRead, };
use crate::{ read_file, count_query, };
use crate::cells::{ Cell, };
use crate::style::{ ANSIStyle, };


/// This structure includes any `readable` file that holds search-related information.
pub struct ReadableFile {
    path: String,
    query: String,
    ignore_case: bool,
}


impl ReadableFile {
    /// `ReadableFile` Constructor.
    pub fn new(path: String, query: String, ignore_case: bool) -> Self {
        Self { path, query, ignore_case, }
    }

    /// `path` attr getter.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// This method displays only the lines that are matched by `Cells`.
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

    /// This method only displays the `number` of matches in the file along with the `file path`.
    pub fn print_count(&self) {
        println!("{}", self.get_head());
    }

    /// This method uses `cells` to display all lines in the file alongside the `matched lines`.
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

    /// This method returns a constant `head` consisting of the
    /// `file name` and the `number` of matching occurrences.
    ///
    /// # Returns
    ///
    /// The output is a new `String`.
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

    /// The task of this method is to `count` the matches in the file.
    /// This is done with the help of functions in the main module.
    ///
    /// # Arguments
    ///
    /// * `query` - The string to search for.
    ///
    /// # Returns
    ///
    /// The output is a `usize`, which is the number of `matches`.
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