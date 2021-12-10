use termion::{color, style};

pub fn print_day(d: usize) {
    println!(" \u{1F389} {}Day {} !{}", style::Underline, d, style::Reset);
}

pub fn fmt_bright<T: std::fmt::Display>(t: &T) -> String {
    format!("{}{}{}", color::Fg(color::LightWhite), t, color::Fg(color::Reset))
}