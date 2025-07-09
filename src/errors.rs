use crate::style::{ ANSIStyle, };
use crate::help ::{ get_usage, get_try, };


pub struct ErrorMsg {
    msg: String,
}


impl ErrorMsg {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn raise(&self) {
        for part in self.gen_msg() {
            eprintln!("{}", part);
        }
    }

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