//! This module is related to structuring and displaying **errors** in `lenz`.
//! In this module, errors are structured and can be displayed in the form
//! of a specific message with a specific format so that the
//! program can use this specific structure whenever it needs to **handle errors**.


use crate::style::{ ANSIStyle, };
use crate::help ::{ get_usage, get_try, };


/// The `ErrorMsg` structure, upon receiving a message,
/// formats the error in a structured way and can display it if needed.
/// This structure can be used to **control**, **handle**, and **display** errors in the program.
pub struct ErrorMsg {
    msg: String,
}


impl ErrorMsg {
    /// `ErrorMsg` Constructor.
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    /// `msg` attr getter.
    pub fn msg(&self) -> &str {
        &self.msg
    }

    /// This method can `raise` an error and **display** the error
    /// in the form of a **standard CLI** message.
    pub fn raise(&self) {
        for part in self.gen_msg() {
            eprintln!("{}", part);
        }
    }

    /// The task of this method is to **generate** the **format** and error message.
    /// Each error message consists of **fixed instructions** and a **message** that can be set.
    ///
    /// # Returns
    ///
    /// A **vector** (`Vec<String>`) containing messages (`String`)s generated in order.
    fn gen_msg(&self) -> Vec<String> {
        // Error message
        let error: String = format!(
            "{}{}Error:{} {}",
            ANSIStyle::FGRed.as_str(),
            ANSIStyle::Bold.as_str(),
            ANSIStyle::Reset.as_str(),
            self.msg,
        );
        // Each will be placed into a vector
        // for printing, respectively.
        vec![get_usage(), get_try(), error]
    }
}