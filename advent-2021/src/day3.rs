use std::cmp::Ordering;
use std::time::SystemTime;
use crate::{DayResult, DaySolver};

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day3.dat");
        let start = SystemTime::now();
        let (epsilon, gamma) = epsilon_gamma(data, 12);
        let o2 = o2_generator(data, 12);
        let co2 = co2_scrubber(data, 12);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let desc1 = format!("Analyzed logs. Epsilon = {}, Gamma = {} -> Power {}",
                            epsilon, gamma,
                            epsilon * gamma);
        let desc2 = format!("O2 Generator {}, CO2 Scrubber {} -> Life Support Rating {}",
                            o2, co2,
                            o2 * co2);

        DayResult{
            description: format!("{}\n{}", desc1, desc2),
            part1: format!("{}", epsilon * gamma),
            part2: format!("{}", co2 * o2),
            timing_us: timed.as_micros(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day3.dat");
        let (epsilon, gamma) = epsilon_gamma(data, 5);
        let o2 = o2_generator(data, 5);
        let co2 = co2_scrubber(data, 5);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma, 22);
        assert_eq!(o2, 23);
        assert_eq!(co2, 10);
    }
}

pub fn epsilon_gamma(data :&str, length: usize) -> (i32, i32) {
    let lines = data.lines();
    let mut line_count = 0;
    let mut counts = vec![0;length];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
        line_count += 1;
    }

    // Not build the two numbers
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut power = 1;

    for bit in counts.iter().rev() {
        match (2*bit).cmp(&line_count) {
            Ordering::Equal => panic!("Exactly balanced chars."),
            // 1 is most common, so gets added to the gamma
            Ordering::Greater => gamma += power,
            // 1 is least common, so gets added to epsilon
            Ordering::Less => epsilon += power,
        }

        power *= 2;
    }

    (epsilon, gamma)
}

pub fn co2_scrubber(data: &str, length: usize) -> i32 {
    let mut lines: Vec<&str> = data.lines().collect();

    for i in 0..length {
        let mut upper = vec![];
        let mut lower = vec![];

        for line in lines {
            if line.chars().nth(i).unwrap() == '1' {
                upper.push(line);
            } else {
                lower.push(line);
            }
        }

        if lower.len() <= upper.len() {
            // Take least common, preferring lower in ties.
            lines = lower;
        } else {
            lines = upper;
        }

        if lines.len() == 1 {
            return int_of_str(lines[0]);
        }
    }

    panic!("did not find a single value");
}

pub fn o2_generator(data: &str, length: usize) -> i32 {
    let mut lines: Vec<&str> = data.lines().collect();

    for i in 0..length {
        let mut upper = vec![];
        let mut lower = vec![];

        for line in lines {
            if line.chars().nth(i).unwrap() == '1' {
                upper.push(line);
            } else {
                lower.push(line);
            }
        }

        if upper.len() >= lower.len() {
            // Take most common, preferring upper in ties.
            lines = upper;
        } else {
            lines = lower;
        }


        if lines.len() == 1 {
            let val = int_of_str(lines[0]);
            return val;
        }
    }

    panic!("did not find a single value");
}

fn int_of_str(s: &str) -> i32 {
    let mut power = 1;
    let mut res = 0;
    for c in s.chars().rev() {
        if c == '1' {
           res += power;
        }
        power *= 2;
    }

    res
}

