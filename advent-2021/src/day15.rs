use std::cmp::Ordering;
use crate::{DaySolver, DayResult};
use std::time::SystemTime;

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
    grid_size: usize,
    full_grid_size: usize,
}

struct TryMove {
    i: usize,
    j: usize,
    cost: u64,
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

        Self {
            risks_minus,
            grid_size,
            full_grid_size,
        }
    }

    fn find_path(&self) -> u64 {
        let mut costs_to = vec![vec![u64::max_value(); self.full_grid_size]; self.full_grid_size];
        let mut worklist: Vec<TryMove> = vec![TryMove{
            i: 0,
            j: 0,
            cost: 0,
        }];

        while worklist.len() > 0 {
            self.paths_update(&mut worklist, &mut costs_to);
        }

        /*
        // Print out the final grid
        for l in &costs_to {
            let strs = l.iter().map(|r| if *r > 1000 { String::from("|****|") } else { format!("|{:04}|", r)}).collect::<Vec<String>>();
            println!("{}", strs.join("."));
        }
         */
        costs_to[self.full_grid_size - 1][self.full_grid_size - 1]
    }

    fn could_improve(&self, try_move: &TryMove, costs: &Vec<Vec<u64>>) -> bool {
        let current_best = costs[self.full_grid_size - 1][self.full_grid_size - 1];
        return self.best_case(try_move) < current_best;
    }

    fn best_case(&self, try_move: &TryMove) -> u64 {
        try_move.cost + (self.full_grid_size - try_move.i) as u64
            + (self.full_grid_size - try_move.j) as u64
    }

    fn risk_at(&self, i: usize, j: usize) -> u64 {
        // The grid repeats, but for each repeat increments, wrapping to 1-8.
        let incr = (i / self.grid_size) + (j / self.grid_size);
        let base_val = self.risks_minus[i % self.grid_size][j % self.grid_size];
        ((base_val as usize + incr) % 9) as u64 + 1
    }

    fn paths_update(&self, worklist: &mut Vec<TryMove>, costs: &mut Vec<Vec<u64>>) {
        let TryMove { i, j, cost} = worklist.pop().unwrap();

        // Note that we don't 'enter' the origin, so no cost is added.
        let new_cost = if  i == 0 && j == 0 { cost } else { cost
            + self.risk_at(i, j) };
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

                // We ignore falling off the high end.
                if (new_i >= self.full_grid_size) ||
                    (new_j >= self.full_grid_size) {
                    continue;
                }

                let new_move = TryMove {
                    i: (i as i64 + di) as usize,
                    j: (j as i64 + dj) as usize,
                    cost: new_cost,
                };

                if self.could_improve(&new_move, &costs) {
                    worklist.push(new_move)
                }
            }
        }

        worklist.sort_unstable_by(|tma, tmb| self.order_moves(tma, tmb));
    }

    fn order_moves(&self, mva: &TryMove, mvb: &TryMove) -> Ordering {
        let costa = self.best_case(mva);
        let costb = self.best_case(mvb);
        costb.cmp(&costa)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
    }
}

