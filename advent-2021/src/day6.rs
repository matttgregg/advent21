use std::time::SystemTime;
use crate::{DayResult, DaySolver};

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("./data/day6.dat");
        let start = SystemTime::now();
        let after80 = evolve(data, 80);
        let after256 = evolve(data, 256);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let desc1 = format!("After 80 days there are {} fish.", after80);
        let desc2 = format!("After 256 days there are {} fish.", after256);

        DayResult{
            description: format!("{}\n{}", desc1, desc2),
            part1: format!("{}", after80),
            part2: format!("{}", after256),
            timing_us: timed.as_micros(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("./data/test_day6.dat");
        let after80 = evolve(data, 80);
        let after256 = evolve(data, 256);
        assert_eq!(after80, 5934);
        assert_eq!(after256, 26984457539);
    }
}

pub fn evolve(from: &str, generations: usize) -> u64 {
    // We store the day state as a vec of timers 0-8, plus 9:Birthed (which reset to 6)
    let mut fishes = vec![0u64;9];
    let mut total_fish = 0;
    // The input is a comma separated list of timers.
    for i in from.split(',').map(|c| c.parse::<usize>().unwrap()) {
        fishes[i] += 1;
        total_fish += 1;
    }

    // For sanity, we take gen 0 as our starting state.
    for _generation in 1..=generations {
        // Use built in rotate for vector.
        fishes.rotate_left(1);
        // All the birthing fish reset to 6 - these are the same as the fish that just gave birth, 0 -> 8.
        fishes[6] += fishes[8];
        total_fish += fishes[8];
    }

    total_fish
}
