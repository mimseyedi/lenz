use crate::{ count_query, };
use crate::style::{hg_query, ANSIStyle};


const VOID: usize = 7;


pub struct Cell {
    query: String,
    line: String,
    lineno: usize,
    start_at: usize,
}


impl Cell {
    pub fn new(query: String, line: String, lineno: usize, start_at: usize) -> Self {
        Self { query, line, lineno , start_at}
    }

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

    pub fn check_line(&self, ignore_case: bool) -> bool {
        if let Some(_) = count_query(&self.query, &self.line, ignore_case) {
            return true;
        }
        false
    }

    fn get_formatted_line(&self, ignore_case: bool) -> String {
        hg_query(&self.query, &self.line, ignore_case)
    }

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