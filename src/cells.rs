//! This module is related to matching cells.
//! Each search in a file can consist of one or more matching cells,
//! each cell containing line content, line number, etc.
//! for better display of the output.


use crate::{ count_query, };
use crate::style::{hg_query, ANSIStyle};


/// The space between the **line number** and the **formatted content** of the line.
const VOID: usize = 7;


/// A `Cell` is a **structure** for displaying **line-by-line** matches in a file.
pub struct Cell {
    query: String,
    line: String,
    lineno: usize,
    start_at: usize,
}


impl Cell {
    /// `Cell` Constructor.
    pub fn new(query: String, line: String, lineno: usize, start_at: usize) -> Self {
        Self { query, line, lineno , start_at}
    }

    /// The task of this method is to display and print the contents of the matching `Cell`.
    ///
    /// # Arguments
    ///
    /// * `lineno_color` - Dynamic mode to color matching line numbers.
    /// * `ignore-case` - A boolean indicating whether the search should be case-insensitive.
    pub fn print(&self, lineno_color: Option<bool>, ignore_case: bool) {
        let s = " ".repeat(self.start_at);
        let void = " ".repeat(VOID - self.lineno.to_string().len());
        let lineno;
        let ig;
        if ignore_case {
            ig = true;
        } else { ig = false;}
        if let Some(true) = lineno_color {
            lineno = self.get_formatted_lineno(true, ig);
        } else {
            lineno = self.get_formatted_lineno(false, ig);
        }
        println!(
            "{}{} {}{}",
            s,
            lineno,
            void,
            self.get_formatted_line(ig),
        )
    }

    /// This method returns the status of a row relative to the query.
    /// `true` if it exists, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `ignore-case` - A boolean indicating whether the search should be case-insensitive.
    ///
    /// # Returns
    ///
    /// A boolean value, associated with a match or not match.
    pub fn check_line(&self, ignore_case: bool) -> bool {
        if let Some(_) = count_query(&self.query, &self.line, ignore_case) {
            return true;
        }
        false
    }

    /// This method formats the line corresponding to `Cell`
    /// and returns it as a new string.
    ///
    /// # Arguments
    ///
    /// * `ignore-case` - A boolean indicating whether the search should be case-insensitive.
    ///
    /// # Returns
    ///
    /// A formatted `String`.
    fn get_formatted_line(&self, ignore_case: bool) -> String {
        hg_query(&self.query, &self.line, ignore_case)
    }

    /// This method formats and colors the line number
    /// of the `Cell` and returns it as a new `String`.
    ///
    ///
    /// # Arguments
    ///
    /// * `dyn_color` - A boolean to dynamically change the line number color.
    /// * `ignore-case` - A boolean indicating whether the search should be case-insensitive.
    ///
    /// # Returns
    ///
    /// A `String`, containing the formatted line number.
    fn get_formatted_lineno(&self, dyn_color: bool, ignore_case: bool) -> String {
        let color: &str;
        if dyn_color {
            if self.check_line(ignore_case) == true {
                color = ANSIStyle::FGRed.as_str();
            } else {
                color = ANSIStyle::FGGreen.as_str();
            }
        } else {
            color = ANSIStyle::FGGreen.as_str();
        }
        String::from(
            format!(
                "{}{}{}",
                color,
                self.lineno,
                ANSIStyle::Reset.as_str(),
            )
        )
    }
}