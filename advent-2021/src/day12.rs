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
    connections: HashMap<String, Vec<String>>,
}

impl Caves {
    fn from(data: &str) -> Self {
        let mut connections = HashMap::new();
        for line in data.lines() {
            let caves = line.split("-").map(|s| format!("{}", s)).collect::<Vec<String>>();
            // Connect in both directions.
            let from = connections.entry(caves[0].clone()).or_insert(vec![]);
            from.push(caves[1].clone());
            let to = connections.entry(caves[1].clone()).or_insert(vec![]);
            to.push(caves[0].clone());
        }
        Self { connections }
    }

    fn inner_routes(&self, at: &str, mut seen: HashMap<String, u8>, limit: u8, route: &str) -> u64 {
        let mut routes = 0u64;
        for explore in &self.connections[at] {
            if explore == "start" {
               // Never go back to the start.
            } else if explore == "end" {
                //println!("{},end", route);
                routes += 1;
            } else if is_upper(explore) {
                // We can freely explore through 'large' caves.
                routes += self.inner_routes(explore, seen.clone(), limit, &format!("{},{}", route, explore));
            } else if *seen.entry(explore.clone()).or_insert(0) < limit {
                // This is a small cave we haven't seen.
                // Mark the cave and explore.
                let mut seen_copy = seen.clone();
                let seen_count = seen_copy.entry(explore.clone()).or_insert(0);
                let new_limit = limit - *seen_count;
                *seen_count += 1;
                routes += self.inner_routes(explore, seen_copy, new_limit, &format!("{},{}", route, explore));
            }
        }
        routes
    }

    fn routes(&self, limit: u8) -> u64 {
        self.inner_routes("start", HashMap::new(), limit, "start")
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

