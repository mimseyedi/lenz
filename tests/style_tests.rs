use lenz::style::{hg_query, ANSIStyle};


#[test]
fn test_styles() {
    let o = format!(
        "{}BOLD{}Italic{}Underline{}",
        ANSIStyle::Bold.as_str(),
        ANSIStyle::Italic.as_str(),
        ANSIStyle::Underline.as_str(),
        ANSIStyle::Reset.as_str(),
    );
    let e = String::from(
        "\x1b[1mBOLD\x1b[3mItalic\x1b[4mUnderline\x1b[0m",
    );
    assert_eq!(o, e);
}


#[test]
fn test_style_fg_colors() {
    let o = format!(
        "{}FGRed{}FGGreen{}FGMagenta{}",
        ANSIStyle::FGRed.as_str(),
        ANSIStyle::FGGreen.as_str(),
        ANSIStyle::FGMagenta.as_str(),
        ANSIStyle::Reset.as_str(),
    );
    let e = String::from(
        "\x1b[31mFGRed\x1b[32mFGGreen\x1b[35mFGMagenta\x1b[0m",
    );
    assert_eq!(o, e);
}


#[test]
fn test_style_bg_colors() {
    let o = format!(
        "{}BGYellow{}BGWhite{}BGBlue{}",
        ANSIStyle::BGYellow.as_str(),
        ANSIStyle::BGWhite.as_str(),
        ANSIStyle::BGBlue.as_str(),
        ANSIStyle::Reset.as_str(),
    );
    let e = String::from(
        "\x1b[43mBGYellow\x1b[47mBGWhite\x1b[44mBGBlue\x1b[0m",
    );
    assert_eq!(o, e);
}


#[test]
fn test_hg_query_case_sensitivity_one_match() {
    let query = "lenz";
    let text = "Just for testing ... lenz is cool.";
    let o = hg_query(query, text, false);
    let e = format!(
        "Just for testing ... {}lenz{} is cool.",
        ANSIStyle::BGRed.as_str(),
        ANSIStyle::Reset.as_str(),
    );
    assert_eq!(o, e);
}


#[test]
fn test_hg_query_case_sensitivity_more_than_one_match() {
    let query = "lenz";
    let text = "Just for testing ... lenz is cool. YES! lenz is super cool!";
    let o = hg_query(query, text, false);
    let e = format!(
        "Just for testing ... {}lenz{} is cool. YES! {}lenz{} is super cool!",
        ANSIStyle::BGRed.as_str(),
        ANSIStyle::Reset.as_str(),
        ANSIStyle::BGRed.as_str(),
        ANSIStyle::Reset.as_str(),
    );
    assert_eq!(o, e);
}


#[test]
fn test_hg_query_case_sensitivity_no_match() {
    let query = "lenz";
    let text = "Just for testing ...";
    let o = hg_query(query, text, false);
    let e = "Just for testing ...";
    assert_eq!(o, e);
}


#[test]
fn test_hg_query_case_insensitivity_one_match() {
    let query = "lenz";
    let text = "Just for testing ... LENz is cool.";
    let o = hg_query(query, text, true);
    let e = format!(
        "Just for testing ... {}LENz{} is cool.",
        ANSIStyle::BGRed.as_str(),
        ANSIStyle::Reset.as_str(),
    );
    assert_eq!(o, e);
}


#[test]
fn test_hg_query_case_insensitivity_more_than_one_match() {
    let query = "LENZ";
    let text = "Just for testing ... lEnz is cool. YES! lenZ is super cool.";
    let o = hg_query(query, text, true);
    let e = format!(
        "Just for testing ... {}lEnz{} is cool. YES! {}lenZ{} is super cool.",
        ANSIStyle::BGRed.as_str(),
        ANSIStyle::Reset.as_str(),
        ANSIStyle::BGRed.as_str(),
        ANSIStyle::Reset.as_str(),
    );
    assert_eq!(o, e);
}


#[test]
fn test_hg_query_case_insensitivity_no_match() {
    let query = "Lenz";
    let text = "Just for testing ...";
    let o = hg_query(query, text, true);
    let e = "Just for testing ...";
    assert_eq!(o, e);
}