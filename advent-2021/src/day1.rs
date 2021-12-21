use crate::{DaySolver, DayResult};
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day1.dat");
        let values: Vec<i32> = data.lines().map(|c| c.parse::<i32>().unwrap()).collect();
        let start = SystemTime::now();
        let increases = increases(&values);
        let smoothed_increases = smoothed(&values);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Increases {} . Smoothed increases {} .", increases, smoothed_increases);

        DayResult {
            description,
            part1: format!("{}", increases),
            part2: format!("{}", smoothed_increases),
            timing_us: timed.as_micros(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("./data/test_day1.dat");
        let vals: Vec<i32> = data.lines().map(|c| c.parse::<i32>().unwrap()).collect();
        let increases = increases(&vals);
        let smoothed_increases = smoothed(&vals);
        assert_eq!(increases, 7);
        assert_eq!(smoothed_increases, 5);
    }
}

fn increases(data: &[i32]) -> i32 {
    let mut last = data[0];
    let mut increases = 0;
    for (i, v) in data.iter().enumerate() {
        if i > 0 && *v > last {
            increases += 1;
        }
        last = *v;
    }
    increases
}

fn smoothed(data: &[i32]) -> i32 {
    let mut a;
    let mut b = data[0];
    let mut c = data[0];
    let mut last = 0;
    let mut increases = 0;

    for (i, v) in data.iter().enumerate() {
        a = b;
        b = c;
        c = *v;
        let total = a + b + c;
        if i > 2 && total > last {
            increases += 1;
        }
        last = total;
    }

    increases
}