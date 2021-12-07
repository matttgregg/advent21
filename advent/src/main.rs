mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

use std::env;
use std::process;

fn main() {
    match day_arg(env::args()) {
        Some(1) => day1::solve(),
        Some(2) => day2::solve(),
        Some(3) => day3::solve(),
        Some(4) => day4::solve(),
        Some(5) => day5::solve(),
        Some(6) => day6::solve(),
        Some(7) => day7::solve(),
        Some(0) | None => {
            day1::solve();
            day2::solve();
            day3::solve();
            day4::solve();
            day5::solve();
            day6::solve();
            day7::solve();
        },
        Some(x) => { eprintln!("Unimplemented day {}", x); process::exit(1);},
    }
}

fn day_arg(mut args: env::Args) -> Option<i32> {
    args.next();

    let day = match args.next() {
        Some(day) => day.parse::<i32>(),
        None => return None,
    };

    if let Ok(i) = day {
        return Some(i);
    }

    None
}
