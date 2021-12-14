use crate::{DaySolver, DayResult};
use std::time::SystemTime;
use std::collections::HashMap;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day14.dat");
        let start = SystemTime::now();
        let mut system = PolymerSystem::new(data);
        let strength10 = system.evolve_caching(10);
        let strength40 = system.evolve_caching(40);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Strength after 10 steps: {}, strength after 40: {}",
            strength10, strength40);

        DayResult {
            description,
            part1: format!("{}", strength10),
            part2: format!("{}", strength40),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CharPair(u32, u32);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CharPairTo(CharPair, usize);

type CharFrequency = [u64;26];

#[derive(Debug)]
struct PolymerSystem {
    start: Vec<u32>,
    rules: HashMap<CharPair, u32>,
    cache: HashMap<CharPairTo, CharFrequency>,
}

impl PolymerSystem {

    fn evolve_caching(&mut self, steps: usize) -> u64 {
        // We get the required frequencies for the starting string.
        let mut freq = [0u64;26];

        for i in 1..self.start.len() {
            let pair_freq =
                self.evolve_caching_pair(&CharPairTo(
                    CharPair(self.start[i - 1], self.start[i]), steps));

            // Accumulate the frequencies
            for j in 0..26usize {
                freq[j] += pair_freq[j];
            }

            // Need to add the final char.
            if i + 1 == self.start.len() {
                freq[self.start[i] as usize] += 1;
            }
        }

        // Now find min, max and strength values.
        // Find most and least common.
        let mut most = 1;
        let mut least = u64::max_value();
        for c in freq.iter() {
            most = std::cmp::max(most, *c);
            if *c > 0 {
                least = std::cmp::min(least, *c);
            }
        }

        most - least
    }

    fn evolve_caching_pair(&mut self, from: &CharPairTo) -> CharFrequency {
        // We've finished. We just get the final char.
        if let Some(freq) = self.cache.get(from) {
            *freq
        } else if from.1 == 0 {
            let mut freq = [0u64;26];
            freq[from.0.0 as usize] = 1;
            self.cache.insert(from.clone(), freq.clone());
            freq
        } else if let Some(insertion) = self.rules.get(&from.0) {
            // We have a rule, so build the sum from the inner parts.
            let first_pair = CharPairTo(CharPair(from.0.0, *insertion), from.1 - 1);
            let second_pair = CharPairTo(CharPair(*insertion, from.0.1), from.1 - 1);
            let mut freq = self.evolve_caching_pair(&first_pair);
            let freq_second = self.evolve_caching_pair(&second_pair);

            for i in 0..26 {
                freq[i] += freq_second[i];
            }
            self.cache.insert(from.clone(), freq.clone());
            freq.clone()
        } else {
            // No rule, so this pair remains untouched.
            let mut freq = [0u64;26];
            freq[from.0.0 as usize] = 1;
            self.cache.insert(from.clone(), freq);
            freq.clone()
        }
    }

    fn new(data: &str) -> Self {
        let mut start = vec![];
        let mut rules = HashMap::new();
        for (i, line) in data.lines().enumerate() {
            if i == 0 {
                start = line.chars().map(|c| c.to_digit(36).unwrap() - 10).collect::<Vec<u32>>();
            } else if line.contains("->") {
                let chars = line.chars().collect::<Vec<char>>();
                let c1 = chars[0].to_digit(36).unwrap() - 10;
                let c2 = chars[1].to_digit(36).unwrap() - 10;
                let insertion = chars[6].to_digit(36).unwrap() - 10;
                rules.insert(CharPair(c1, c2), insertion);
            }
        }

        PolymerSystem {
            start,
            rules,
            cache: Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day14.dat");
        let mut system = PolymerSystem::new(data);
        let strength10 = system.evolve_caching(10);
        let strength40 = system.evolve_caching(40);
        assert_eq!(strength10, 1588);
        assert_eq!(strength40, 2188189693529)
    }
}

