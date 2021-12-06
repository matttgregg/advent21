use crate::utils;
use std::time::SystemTime;

// Solve today.
pub fn solve() {
    utils::print_day(6);
    let data = include_str!("./data/day6.dat");
    let start = SystemTime::now();
    let after80 = evolve(data, 80);
    let after256 = evolve(data, 256);
    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("After 80 days there are {} fish.", utils::fmt_bright(&after80));
    println!("After 256 days there are {} fish.", utils::fmt_bright(&after256));
    utils::print_duration(timed);
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
    let mut fishes = vec![0;10];
    let mut total_fish = 0;
    // The input is a comma separated list of timers.
    for c in from.split(",") {
        let i = c.parse::<usize>().unwrap();
        fishes[i] = fishes[i] + 1;
        total_fish += 1;
    }

    // For sanity, we take gen 0 as our starting state.
    for _generation in 1..=generations {
        // We push all 0 fish to 9 so as to avoid overwriting other fish.
        fishes[9] = fishes[0];
        // Now all timers evolve. Note that 8, new born fish are pulled in automatically.
        for i in 0..=8 {
            fishes[i] = fishes[i + 1];
        }

        // All the birthing fish reset to 6.
        fishes[6] = fishes[6] + fishes[9];
        total_fish += fishes[9];

        //println!("Generation {} : {} fish.", generation, total_fish);
    }

    total_fish
}
