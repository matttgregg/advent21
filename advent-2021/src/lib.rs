mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub struct DayResult {
    pub part1: String,
    pub part2: String,
    pub description: String,
    pub timing_us: u128,
}

pub trait DaySolver {
    fn solve(&self) -> DayResult;
}

pub fn days() -> Vec<Box<dyn DaySolver>> {
    let mut days: Vec<Box<dyn DaySolver>> = vec![Box::new(day1::Day {})];
    days.push(Box::new(day2::Day {}));
    days.push(Box::new(day3::Day {}));
    days.push(Box::new(day4::Day {}));
    days.push(Box::new(day5::Day {}));
    days.push(Box::new(day6::Day {}));
    days.push(Box::new(day7::Day {}));
    days.push(Box::new(day8::Day {}));
    days.push(Box::new(day9::Day {}));
    days.push(Box::new(day10::Day {}));
    days.push(Box::new(day11::Day {}));
    days.push(Box::new(day12::Day {}));
    days.push(Box::new(day13::Day {}));
    days.push(Box::new(day14::Day {}));
    days.push(Box::new(day15::Day {}));
    days.push(Box::new(day16::Day {}));
    days.push(Box::new(day17::Day {}));
    days.push(Box::new(day18::Day {}));
    days.push(Box::new(day19::Day {}));
    days.push(Box::new(day20::Day {}));
    days
}
