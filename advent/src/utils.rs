use std::time;
use termion::{color, style};

pub fn print_day(d: i32) {
    println!(" \u{1F389} {}Day {} !{}", style::Underline, d, style::Reset);
}

pub fn print_duration(d: time::Duration) {
    if d.as_micros() < 1000 {
        println!(" \u{1F44D} {}Timed: {}us {}", style::Invert, d.as_micros(), style::Reset);
    } else {
        println!(" \u{1F44D} {}Timed: {} ms {}us {}", style::Invert, d.as_millis(), d.as_micros() % 1000, style::Reset);
    }

    println!{};
}

pub fn fmt_bright<T: std::fmt::Display>(t: &T) -> String {
    format!("{}{}{}", color::Fg(color::LightWhite), t, color::Fg(color::Reset))
}