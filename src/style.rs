//! This module is about styling and coloring the output for better clarity and readability.
//! This module uses `ANSI escape codes` for styling.
//! Naturally, this approach can only be used in terminals or shells that support it.
//! Like `bash`.


use crate::{ find_all, };


/// An enum, to define various colors and styles.
pub enum ANSIStyle {
    Reset,
    // Styles
    Bold,
    Italic,
    Underline,
    Blink,
    // Foreground colors
    FGBlack,
    FGRed,
    FGGreen,
    FGYellow,
    FGBlue,
    FGMagenta,
    FGCyan,
    FGWhite,
    FGDef,
    // Background colors
    BGBlack,
    BGRed,
    BGGreen,
    BGYellow,
    BGBlue,
    BGMagenta,
    BGCyan,
    BGWhite,
    BGDef,
}


impl ANSIStyle {
    /// The task of this method is to convert enum options
    /// into literal strings corresponding to `ANSI escape codes`.
    pub fn as_str(&self) -> &'static str {
        match *self {
            ANSIStyle::Reset     => "\x1b[0m",
            // Styles
            ANSIStyle::Bold      => "\x1b[1m",
            ANSIStyle::Italic    => "\x1b[3m",
            ANSIStyle::Underline => "\x1b[4m",
            ANSIStyle::Blink     => "\x1b[5m",
            // Foreground colors
            ANSIStyle::FGBlack   => "\x1b[30m",
            ANSIStyle::FGRed     => "\x1b[31m",
            ANSIStyle::FGGreen   => "\x1b[32m",
            ANSIStyle::FGYellow  => "\x1b[33m",
            ANSIStyle::FGBlue    => "\x1b[34m",
            ANSIStyle::FGMagenta => "\x1b[35m",
            ANSIStyle::FGCyan    => "\x1b[36m",
            ANSIStyle::FGWhite   => "\x1b[37m",
            ANSIStyle::FGDef     => "\x1b[39m",
            // Background colors
            ANSIStyle::BGBlack   => "\x1b[40m",
            ANSIStyle::BGRed     => "\x1b[41m",
            ANSIStyle::BGGreen   => "\x1b[42m",
            ANSIStyle::BGYellow  => "\x1b[43m",
            ANSIStyle::BGBlue    => "\x1b[44m",
            ANSIStyle::BGMagenta => "\x1b[45m",
            ANSIStyle::BGCyan    => "\x1b[46m",
            ANSIStyle::BGWhite   => "\x1b[47m",
            ANSIStyle::BGDef     => "\x1b[49m",
        }
    }
}


/// The task of this function is to `highlight` the query in a string.
/// This is done with the help of other functions in the main module.
///
/// # Arguments
///
/// * `query` - The specified substring.
/// * `text` - Text that can contain a substring.
/// * `ignore_case` - A boolean indicating whether the search should be case-insensitive.
///
/// # Returns
///
/// The output is a new `String` highlighted.
/// If there is no query, the `original` string itself will be returned.
pub fn hg_query(query: &str, text: &str, ignore_case: bool) -> String {
    match find_all(query, text, ignore_case) {
        Some(vec) => {
            let mut hg_str = String::new();
            let mut l = 0;
            for (x, n) in vec {
                hg_str.push_str(&text[l..x]);
                hg_str.push_str(ANSIStyle::BGRed.as_str());
                hg_str.push_str(&text[x..x+n]);
                hg_str.push_str(ANSIStyle::Reset.as_str());
                l = x + query.len();
            }
            hg_str.push_str(&text[l..]);
            hg_str
        }
        None => text.to_string(),
    }
}