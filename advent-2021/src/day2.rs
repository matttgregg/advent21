use std::time::SystemTime;
use crate::{DayResult, DaySolver};

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let input = include_str!("data/day2.dat");
        let start = SystemTime::now();
        let (h1, d1) = horizontal_depth(input);
        let (h2, d2) = aiming_horizontal_depth(input);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let desc1 = format!("Reached distance {} and depth {} -> {}",
                            h1, d1,
                            h1 * d1);
        let desc2 = format!("Using corrected steering, reached distance {} and depth {} -> {}",
                            h2, d2,
                            h2 * d2);

        DayResult{
            part1: format!("{}", h1 * d1),
            part2: format!("{}", h2 * d2),
            description: format!("{}\n{}", desc1, desc2),
            timing_us: timed.as_micros(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = include_str!("./data/test_day2.dat");
        let (h1, d1) = horizontal_depth(input);
        let (h2, d2) = aiming_horizontal_depth(input);
        assert_eq!(h1 * d1, 150);
        assert_eq!(h2 * d2, 900);
    }
}

pub fn horizontal_depth(data :&str) -> (i32, i32) {
    let lines = data.lines();
    let mut depth = 0;
    let mut horizontal = 0;
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let val: i32 = split[1].parse().unwrap();
        if split[0] == "forward" {
            horizontal += val;
        } else if split[0] == "down" {
            depth += val;
        } else if split[0] == "up" {
            depth -= val;
        } else {
            panic!("Could not read {}", line);
        }
    }
    (horizontal, depth)
}

pub fn aiming_horizontal_depth(data :&str) -> (i32, i32) {
    let lines = data.lines();
    let mut depth = 0;
    let mut aim = 0;
    let mut horizontal = 0;
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let val: i32 = split[1].parse().unwrap();
        if split[0] == "forward" {
            horizontal += val;
            depth += aim * val
        } else if split[0] == "down" {
            aim += val;
        } else if split[0] == "up" {
            aim -= val;
        } else {
            panic!("Could not read {}", line);
        }
    }
    (horizontal, depth)
}