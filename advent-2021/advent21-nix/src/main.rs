mod utils;

use advent_2021;
use regex::Regex;
use std::env;
use std::process;
use advent_2021::DayResult;

fn main() {
    match day_arg(env::args()) {
        Some(0) | None => {
            for (i, day) in advent_2021::days().iter().enumerate() {
                pretty_print_day(i, &day.solve());
            }
        },
        Some(i) => {
            let days = advent_2021::days();
            if i as usize > days.len() {
                eprintln!("Unimplemented day {}/{}", i, days.len());
                process::exit(1);
            } else {
                pretty_print_day((i - 1) as usize, &days[i as usize - 1].solve());
            }
        }
    }
}

fn pretty_print_day(day_index: usize, solution: &DayResult) {
    utils::print_day(day_index + 1);
    println!("{}\n\t[{}]\n",
             brighten(&solution.description, &solution.part1, &solution.part2),
             pretty_us(solution.timing_us));

}

fn brighten(desc: &String, p1: &String, p2: &String) -> String {
    let re1 = Regex::new(p1).unwrap();
    let re2 = Regex::new(p2).unwrap();
    let replaced = re1.replace_all(desc, utils::fmt_bright(p1)).to_string();
    re2.replace_all(&replaced, utils::fmt_bright(p2)).to_string()
}

fn pretty_us(micros: u128) -> String {
    if micros > 1000_000 {
        format!("{}s", micros / 1000_000)
    } else if micros > 1000 {
        format!("{}ms", micros / 1000)
    } else {
        format!("{}us", micros)
    }
}

fn day_arg(mut args: env::Args) -> Option<u8> {
    args.next();

    let day = match args.next() {
        Some(day) => day.parse::<u8>(),
        None => return None,
    };

    if let Ok(i) = day {
        return Some(i);
    }

    None
}
