use crate::{DaySolver, DayResult};
use std::time::SystemTime;
use std::collections::HashMap;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day14.dat");
        let start = SystemTime::now();
        let system = PolymerSystem::new(data);
        let (_after10, strength10) = system.evolve(10);
        let (_after40, strength40) = system.evolve(40);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Strength aster 10 steps: {}, strength after 40: {}",
            strength10, strength40);

        DayResult {
            description,
            part1: format!("{}", strength10),
            part2: format!("{}", strength40),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CharPair(char, char);

#[derive(Debug)]
struct PolymerSystem {
    start: String,
    rules: HashMap<CharPair, char>,
}

impl PolymerSystem {

    fn evolve(&self, steps: usize) -> (String, u64) {
        //let mut res = self.start.clone();
        let mut res = String::from("CV");
        let mut strength = 0;
        for i in 0..steps {
            let (new_res, new_strength) = self.evolve_one(&res);
            res = new_res;
            strength = new_strength;
            println!("Step: {}, Strength {}", i, strength);
        }
        (res, strength)
    }

    fn evolve_one(&self, from: &str) -> (String, u64) {
        let mut result: Vec<char> = vec![];
        let mut counts: HashMap<&char, u64> = HashMap::new();
        let chars = from.chars().collect::<Vec<char>>();
        for i in 1..chars.len() {
            result.push(chars[i - 1]);

            let upd = if let Some(c) = counts.get(&chars[i - 1]) { *c + 1 } else { 1 };
            counts.insert(&chars[i - 1], upd);

            let char_pair = CharPair(chars[i - 1], chars[i]);
            if let Some(insertion) = self.rules.get(&char_pair) {
                result.push(*insertion);

                let upd = if let Some(c) = counts.get(insertion) { *c + 1 } else { 1 };
                counts.insert(insertion, upd);
            }
            if i == chars.len() - 1 {
                result.push(chars[i]);
                let upd = if let Some(c) = counts.get(&chars[i]) { *c + 1 } else { 1 };
                counts.insert(&chars[i], upd);
            }
        }
        // Find most and least common.
        let mut most = 1;
        let mut least = chars.len() as u64;
        for c in counts.keys() {
            let count = counts[c];
            most = std::cmp::max(most, count);
            least = std::cmp::min(least, count);
        }

        println!("Strength = {} - {} = {}", most, least, most - least);

        let strength = most - least;
        (result.iter().map(|c| format!("{}", c)).collect::<Vec<String>>().join(""), strength)
    }

    fn new(data: &str) -> Self {
        let mut start = String::from("");
        let mut rules = HashMap::new();
        for (i, line) in data.lines().enumerate() {
            if i == 0 {
                start = String::from(line);
            } else if line.contains("->") {
                let chars = line.chars().collect::<Vec<char>>();
                rules.insert(CharPair(chars[0], chars[1]), chars[6]);
            }
        }

        PolymerSystem {
            start,
            rules
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
    }
}

