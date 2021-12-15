use std::cmp::Ordering;
use crate::{DaySolver, DayResult};
use std::time::SystemTime;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day15.dat");
        let start = SystemTime::now();

        let cave_map1 = CaveMap::from(data, 1);
        let path_risk1 = cave_map1.find_path();

        let cave_map5 = CaveMap::from(data, 5);
        let path_risk5 = cave_map5.find_path();

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Least risk path has risk {}, or on the bigger map {}", path_risk1, path_risk5);

        DayResult {
            description,
            part1: format!("{}", path_risk1),
            part2: format!("{}", path_risk5),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Debug)]
struct CaveMap {
    risks_minus: Vec<Vec<u8>>,
    risks_full: Vec<Vec<u8>>,
    grid_size: usize,
    full_grid_size: usize,
}

struct TryMove {
    i: usize,
    j: usize,
    cost: u64,
    best_attainable: u64,
}

impl Eq for TryMove {}

impl Ord for TryMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.best_attainable.cmp(&other.best_attainable)
    }
}

impl PartialOrd for TryMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TryMove {
    fn eq(&self, other: &Self) -> bool {
        self.best_attainable == other.best_attainable
    }
}

impl CaveMap {
    fn from(data: &str, multiplier: usize) -> Self {
        let mut risks_minus = vec![];
        for line in data.lines() {
            let risk_line = line.chars()
                .map(|c| String::from(c).parse::<u8>().unwrap() - 1)
                .collect::<Vec<u8>>();
            risks_minus.push(risk_line);
        }
        let grid_size = risks_minus.len();
        let full_grid_size = multiplier * grid_size;

        // Expand to write the full grid, to avoid recalculating.
        let mut risks_full = vec![vec![0; full_grid_size]; full_grid_size];
        for i in 0..full_grid_size {
            for j in 0..full_grid_size {
                let incr = (i / grid_size) + (j / grid_size);
                let base_val = risks_minus[i % grid_size][j % grid_size];
                risks_full[i][j] = ((base_val as usize + incr) % 9) as u8 + 1;
            }
        }

        Self {
            risks_minus,
            risks_full,
            grid_size,
            full_grid_size,
        }
    }

    fn find_path(&self) -> u64 {
        let mut costs_to = vec![vec![u64::max_value(); self.full_grid_size]; self.full_grid_size];
        let mut worklist: BinaryHeap<Reverse<TryMove>> = BinaryHeap::new();

        worklist.push(Reverse(TryMove{
            i: 0,
            j: 0,
            cost: 0,
            best_attainable: 2 * self.full_grid_size as u64,
        }));

        costs_to[0][0] = 0;
        while worklist.len() > 0 {
            self.paths_update(&mut worklist, &mut costs_to);
        }

        // Print out the final grid
        /*
        for l in &costs_to {
            let strs = l.iter().map(|r| if *r > 10000 { String::from("|****|") } else { format!("|{:04}|", r)}).collect::<Vec<String>>();
            println!("{}", strs.join("."));
        }
        println!("{}x{}", self.full_grid_size, self.full_grid_size);
        */
        costs_to[self.full_grid_size - 1][self.full_grid_size - 1]
    }

    // Check whether we can improve.
    fn could_improve(&self, try_move: &TryMove, costs: &Vec<Vec<u64>>) -> bool {
        if costs[try_move.i][try_move.j] <= try_move.cost {
            false
        } else {
            let current_best = costs[self.full_grid_size - 1][self.full_grid_size - 1];
            self.best_case(try_move) < current_best
        }
    }

    fn could_beat(&self, try_move: &TryMove, costs: &Vec<Vec<u64>>) -> bool {
        let current_best = costs[self.full_grid_size - 1][self.full_grid_size - 1];
        self.best_case(try_move) < current_best
    }

    fn best_case(&self, try_move: &TryMove) -> u64 {
        try_move.cost + (self.full_grid_size - try_move.i) as u64
            + (self.full_grid_size - try_move.j) as u64
    }

    fn risk_at(&self, i: usize, j: usize) -> u64 {
        self.risks_full[i][j] as u64
            /*
        // The grid repeats, but for each repeat increments, wrapping to 1-8.
        let incr = (i / self.grid_size) + (j / self.grid_size);
        let base_val = self.risks_minus[i % self.grid_size][j % self.grid_size];
        ((base_val as usize + incr) % 9) as u64 + 1

             */
    }

    fn paths_update(&self, worklist: &mut BinaryHeap<Reverse<TryMove>>, costs: &mut Vec<Vec<u64>>) {
        let try_move = worklist.pop().unwrap();
        let TryMove { i, j, cost, .. } = try_move.0;
        if !self.could_beat(&try_move.0, costs) {
            return
        }

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

                // We ignore falling off the high end.
                if (new_i >= self.full_grid_size) ||
                    (new_j >= self.full_grid_size) {
                    continue;
                }

                let new_cost = cost + self.risk_at(new_i, new_j);
                let best_attainable =
                    new_cost + (self.full_grid_size - new_i) as u64
                        + (self.full_grid_size - new_j) as u64;

                let new_move = TryMove {
                    i: new_i,
                    j: new_j,
                    cost: new_cost,
                    best_attainable,
                };

                // Check whether this does lead to an improvement.
                if self.could_improve(&new_move, &costs) {
                    costs[new_i][new_j] = new_cost;
                    worklist.push(Reverse(new_move))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day15.dat");

        let cave_map1 = CaveMap::from(data, 1);
        let path_risk1 = cave_map1.find_path();
        assert_eq!(path_risk1, 40);

        let cave_map5 = CaveMap::from(data, 5);
        let path_risk5 = cave_map5.find_path();
        assert_eq!(path_risk5, 315);
    }
}

