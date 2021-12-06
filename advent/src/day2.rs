use crate::utils;
use std::time::SystemTime;

pub fn solve() {
    utils::print_day(2);
    let input = include_str!("data/day2.dat");
    let start = SystemTime::now();
    let (h1, d1) = horizontal_depth(input);
    let (h2, d2) = aiming_horizontal_depth(input);
    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Reached distance {} and depth {} -> {}", h1, d1, utils::fmt_bright(&(h1 * d1)));
    println!("Using corrected steering, reached distance {} and depth {} -> {}", h2, d2, utils::fmt_bright(&(h2 * d2)));
    utils::print_duration(timed);
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
    let lines = data.split("\n");
    let mut depth = 0;
    let mut horizontal = 0;
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let val: i32 = split[1].parse().unwrap();
        if split[0] == "forward" {
            horizontal = horizontal + val;
        } else if split[0] == "down" {
            depth = depth + val;
        } else if split[0] == "up" {
            depth = depth - val;
        } else {
            panic!("Could not read {}", line);
        }
    }
    (horizontal, depth)
}

pub fn aiming_horizontal_depth(data :&str) -> (i32, i32) {
    let lines = data.split("\n");
    let mut depth = 0;
    let mut aim = 0;
    let mut horizontal = 0;
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let val: i32 = split[1].parse().unwrap();
        if split[0] == "forward" {
            horizontal = horizontal + val;
            depth = depth + aim * val
        } else if split[0] == "down" {
            aim = aim + val;
        } else if split[0] == "up" {
            aim = aim - val;
        } else {
            panic!("Could not read {}", line);
        }
    }
    (horizontal, depth)
}