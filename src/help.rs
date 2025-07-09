use crate::style::{ ANSIStyle, };


const VOID: usize = 25;
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct CLIOption<'a> {
    name: &'a str,
    desc: &'a str,
}


impl<'a> CLIOption<'a> {
    fn new(name: &'a str, desc: &'a str) -> Self {
        Self { name, desc }
    }
}


pub struct HelpMessage<'a> {
    desc: String,
    usage: String,
    options: Vec<CLIOption<'a>>,
}


impl<'a> HelpMessage<'a> {
    pub fn new(desc: String, usage: String, options: Vec<CLIOption<'a>>) -> Self {
        Self { desc, usage, options }
    }

    pub fn options(&self) -> Option<&Vec<CLIOption<'a>>> {
        if !self.options.is_empty() { Some(&self.options) } else { None }
    }

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


pub fn get_usage() -> String {
    format!(
        "{}Usage:{} lenz [QUERY] [FILEs]... [OPTION]",
        ANSIStyle::FGGreen.as_str(),
        ANSIStyle::Reset.as_str(),
    )
}


pub fn get_try() -> String {
    format!(
        "Try {}{}'lenz --help'{} for help.\n",
        ANSIStyle::FGWhite.as_str(),
        ANSIStyle::Italic.as_str(),
        ANSIStyle::Reset.as_str(),
    )
}


pub fn get_desc() -> String {
    String::from("\n  Lenz is a simple CLI for browsing files to find words.\n")
}


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


pub fn print_version() {
    println!("{}", VERSION);
}