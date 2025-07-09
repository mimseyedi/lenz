//! This module is related to the `help` section of the program.
//! In this module, the **help message**, **options**,
//! and how to display the help text are implemented and written.


use crate::style::{ ANSIStyle, };


/// The space between the **options name** and their **descriptions**.
const VOID: usize = 25;
/// The **version** of the application that is read directly from `Cargo.toml`.
const VERSION: &str = env!("CARGO_PKG_VERSION");


/// A structure for systematically defining program **options**.
/// This structure can help generate better program help messages
/// by receiving the option `name` and `description`.
pub struct CLIOption<'a> {
    name: &'a str,
    desc: &'a str,
}


impl<'a> CLIOption<'a> {
    /// `CLIOption` Constructor.
    fn new(name: &'a str, desc: &'a str) -> Self {
        Self { name, desc }
    }
}


/// This structure is primarily responsible for formatting and displaying
/// the program's `help message`, by receiving fixed,
/// `unchangeable messages` and `options` for display.
pub struct HelpMessage<'a> {
    desc: String,
    usage: String,
    options: Vec<CLIOption<'a>>,
}


impl<'a> HelpMessage<'a> {
    /// `HelpMessage` Constructor.
    pub fn new(desc: String, usage: String, options: Vec<CLIOption<'a>>) -> Self {
        Self { desc, usage, options }
    }

    /// The task of this method is to return the help message `options`.
    ///
    /// # Returns
    ///
    /// If there is an option, it is `Some(&Vec<CLIOption>)` and otherwise `None`.
    pub fn options(&self) -> Option<&Vec<CLIOption<'a>>> {
        if !self.options.is_empty() { Some(&self.options) } else { None }
    }

    /// The task of this method is to display the program's `help` message.
    /// This means displaying **static** messages and program **options**.
    pub fn show(&self) {
        println!("{}", self.usage);
        println!("{}", self.desc);
        println!("{}",
            format!(
                "{}Options:{}",
                ANSIStyle::FGGreen.as_str(),
                ANSIStyle::Reset.as_str(),
            ),
        );
        for option in &self.options {
            let space = " ".repeat(VOID - option.name.len());
            println!("  {}{}{}", option.name, space, option.desc);
        }
    }
}


/// This function generates a constant string describing how to use the program.
///
/// # Returns
///
/// A `String` containing the usage text.
pub fn get_usage() -> String {
    format!(
        "{}Usage:{} lenz [QUERY] [FILEs]... [OPTION]",
        ANSIStyle::FGGreen.as_str(),
        ANSIStyle::Reset.as_str(),
    )
}


/// This function produces a static message regarding the use of the help option.
///
/// # Returns
///
/// A `String` containing the `try lenz --help` text.
pub fn get_try() -> String {
    format!(
        "Try {}{}'lenz --help'{} for help.\n",
        ANSIStyle::FGWhite.as_str(),
        ANSIStyle::Italic.as_str(),
        ANSIStyle::Reset.as_str(),
    )
}


/// This function generates a static message related to the program **description**.
///
/// # Returns
///
/// A `String` containing the program description.
pub fn get_desc() -> String {
    String::from("\n  Lenz is a simple CLI for browsing files to find words.\n")
}


/// This function returns an instance of `HelpMessage` by setting its attributes.
///
/// # Returns
///
/// An instance of `HelpMessage`.
pub fn get_help() -> HelpMessage<'static> {
    HelpMessage::new(
        get_desc(),
        get_usage(),
        vec![
            CLIOption::new(
                "-i, --ignore-case",
                "Perform case insensitive matching.",
            ),
            CLIOption::new(
                "-c, --count",
                "Counting matches in files.",
            ),
            CLIOption::new(
                "-p, --page-view",
                "Page view.",
            ),
            CLIOption::new(
                "-h, --help",
                "Show this message and exit.",
            ),
            CLIOption::new(
                "-v, --version",
                "Display version and exit.",
            ),
        ]
    )
}


/// The task of this function is to display the program version
/// by referring to the `VERSION` constant.
pub fn print_version() {
    println!("{}", VERSION);
}