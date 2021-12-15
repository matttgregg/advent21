use crate::{DaySolver, DayResult};
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day15.dat");
        let start = SystemTime::now();

        let cave_map = CaveMap::from(data);
        let path_risk = cave_map.find_path();

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Least risk path has risk {}", path_risk);

        DayResult {
            description,
            part1: format!("{}", path_risk),
            part2: format!("{}", 0),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Debug)]
struct CaveMap {
    risks: Vec<Vec<u8>>,
    square_size: usize,
}

impl CaveMap {
    fn from(data: &str) -> Self {
        let mut risks = vec![];
        for line in data.lines() {
            let risk_line = line.chars()
                .map(|c| String::from(c).parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            risks.push(risk_line);
        }
        let square_size = risks.len();

        Self {
            risks,
            square_size,
        }
    }

    fn find_path(&self) -> u64 {
        // We basically follow Djikstra
        let mut costs_to = vec![vec![u64::max_value(); self.square_size]; self.square_size];
        self.paths_update(0, 0, 0, &mut costs_to);
        costs_to[self.square_size - 1][self.square_size - 1]
    }

    fn paths_update(&self, i: usize, j: usize, cost: u64, costs: &mut Vec<Vec<u64>>) {
        // Note that we don't 'enter' the origin, so no cost is added.
        let new_cost = if  i == 0 && j == 0 { cost } else { cost + self.risks[i][j] as u64};
        let old_cost = costs[i][j];

        if new_cost >= old_cost {
            // This is no better than we've already seen. Don't bother progressing.
            return;
        }

        // Otherwise, it is better! Update, and propagate to neighbours.else
        costs[i][j] = new_cost;

        for di in -1..=1 {
            for dj in -1..=1 {
                // We ignore not moving at all, and diagonal moves.
                if di == 0 && dj == 0 || (di * dj != 0) {
                    continue;
                }

                // We ignore falling off the low end.
                if (di < 0 && i == 0) || (dj < 0 && j == 0) {
                    continue;
                }

                let new_i = (i as i64 + di) as usize;
                let new_j = (j as i64 + dj) as usize;

                // We ignor falling off the high end.
                if (new_i >= self.square_size) ||
                    (new_j >= self.square_size) {
                    continue;
                }

                self.paths_update(new_i, new_j, new_cost, costs);
            }
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

