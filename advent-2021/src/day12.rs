use crate::{DaySolver, DayResult};
use std::time::SystemTime;
use std::collections::HashMap;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day12.dat");
        let start = SystemTime::now();
        let caves = Caves::from(data);
        let routes1 = caves.routes(1);
        let routes2 = caves.routes(2);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Cave system has {} routes, or if allowing double exploration {} routes", routes1, routes2);

        DayResult {
            description,
            part1: format!("{}", routes1),
            part2: format!("{}", routes2),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Debug)]
struct Caves {
    connections: Vec<Vec<usize>>,
    start: usize,
    end: usize,
    large: Vec<bool>,
    indices: HashMap<String, usize>,
}

impl Caves {
    fn from(data: &str) -> Self {
        let mut connections: Vec<Vec<usize>> = vec![];
        let start = 0;
        let end = 1;
        let mut indices: HashMap<String, usize> = HashMap::new();
        indices.insert(String::from("start"),start);
        indices.insert(String::from("end"),end);
        for line in data.lines() {
            let caves = line.split("-").map(|s| format!("{}", s)).collect::<Vec<String>>();
            let from = caves[0].clone();
            let to = caves[1].clone();

            // Set up the indices.
            let from_idx = *indices.get(&from).unwrap_or(&indices.len());
            indices.insert(from.clone(), from_idx);
            let to_idx = *indices.get(&to).unwrap_or(&indices.len());
            indices.insert(to.clone(), to_idx);

            // Connect in both directions.
            let largest_idx = std::cmp::max(from_idx, to_idx);
            if largest_idx >= connections.len() {
                connections.resize_with(largest_idx + 1, || vec![])
            }
            connections[from_idx].push(to_idx);
            connections[to_idx].push(from_idx);
        }

        let mut large = vec![false; indices.len()];

        for (k, i) in &indices {
            large[*i] = is_upper(k);
        }

        Self {
            connections,
            start,
            end,
            large,
            indices
        }
    }

    fn inner_routes(&self, at: usize, seen: &mut Vec<u8>, limit: u8, route: &str) -> u64 {
        let mut routes = 0u64;
        for explore in &self.connections[at] {
            if *explore == self.start {
               // Never go back to the start.
            } else if *explore == self.end {
                //println!("{},end", route);
                routes += 1;
            } else if self.large[*explore] {
                // We can freely explore through 'large' caves.
                routes += self.inner_routes(*explore, seen, limit, &format!("{},{}", route, explore));
            } else if seen[*explore] < limit {
                // This is a small cave we haven't seen.
                // Mark the cave and explore.
                let seen_count = seen[*explore];
                let new_limit = limit - seen_count;
                seen[*explore] += 1;
                routes += self.inner_routes(*explore, seen, new_limit, &format!("{},{}", route, explore));
                seen[*explore] -= 1;
            }
        }
        routes
    }

    fn routes(&self, limit: u8) -> u64 {
        self.inner_routes(0, &mut vec![0; self.indices.len()], limit, "0")
    }
}

fn is_upper(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day12.dat");
        let caves = Caves::from(data);
        let routes1 = caves.routes(1);
        let routes2 = caves.routes(2);
        assert_eq!(routes1, 10);
        assert_eq!(routes2, 36);
    }

    #[test]
    fn test_data_b() {
        let data = include_str!("data/test_day12b.dat");
        let caves = Caves::from(data);
        let routes1 = caves.routes(1);
        let routes2 = caves.routes(2);
        assert_eq!(routes1, 19);
        assert_eq!(routes2, 103);
    }

    #[test]
    fn test_data_c() {
        let data = include_str!("data/test_day12c.dat");
        let caves = Caves::from(data);
        let routes1 = caves.routes(1);
        let routes2 = caves.routes(2);
        assert_eq!(routes1, 226);
        assert_eq!(routes2, 3509);
    }
}

