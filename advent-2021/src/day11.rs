use std::fmt::{Display, Formatter};
use crate::{DaySolver, DayResult};
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day11.dat");
        let start = SystemTime::now();
        let mut grid = OctoGrid::new(data);
        let flashed_100 = grid.step_n(100);
        let mut grid2 = OctoGrid::new(data);
        let synchronized_flash = grid2.first_synchronized(100);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Octopus flashes after 100 steps : {} .\n\
        All octopuses flashed at {}.", flashed_100, synchronized_flash);

        DayResult {
            description,
            part1: format!("{}", flashed_100),
            part2: format!("{}", synchronized_flash),
            timing_us: timed.as_micros(),
        }
    }
}

struct OctoGrid {
    octopuses: Vec<Vec<u8>>,
}

impl Display for OctoGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.octopuses.iter()
            .map(|r| format!("{:?}", r))
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

impl OctoGrid {
    fn new(data: &str) -> Self {
        let mut octopuses = vec![];
        for row in data.lines() {
            let octo_row = row.chars().map(|c| format!("{}", c).parse::<u8>().unwrap()).collect();
            octopuses.push(octo_row);
        }

        OctoGrid {
            octopuses,
        }
    }

    fn step_n(&mut self, n: usize) -> u64 {
        let mut flashes = 0u64;
        for _ in 0..n {
            let flashed = self.step();
            //println!("{}", self);
            //println!("[{}] : {} flashes", i, flashed);
            flashes += flashed as u64;
        }
        flashes
    }

    fn first_synchronized(&mut self, target: u8) -> u64 {
        let mut step = 0u64;
        let mut flashed = 0;
        while flashed < target {
            step += 1;
            flashed = self.step();
            //println!("Step {} => {} flashes", step, flashed);
        }
        step
    }

    // Perform a single step, counting the number that flashed.
    fn step(&mut self) -> u8 {
        // First, increment everything and mark those going to flash.
        let mut to_flash = vec![];
        for (i, r) in self.octopuses.iter_mut().enumerate() {
            for (j, c) in r.iter_mut().enumerate() {
                *c += 1;
                if *c >= 10 {
                    to_flash.push((i, j));
                }
            }
        }

        let mut flashed = vec![];
        // Now do the flashing.
        while let Some((fi, fj)) = to_flash.pop() {
            // Note that this flashed.
            flashed.push((fi, fj));

            // Increment neighbours and add any that flash.
            for incr_i in -1..=1i8 {
                for incr_j in -1..=1i8 {
                    // Skip self flashing, and anything outside the grid.
                    if (incr_i == 0 && incr_j == 0) ||
                        (incr_i < 0 && fi == 0) ||
                        (incr_i > 0 && fi + 1 >= self.octopuses.len()) ||
                        (incr_j < 0 && fj == 0) ||
                        (incr_j > 0 && fj + 1 >= self.octopuses[0].len())
                    {
                        continue;
                    }

                    // Otherwise, increase if possible, and note if it flashes.
                    let upd_i = (fi as i8 + incr_i) as usize;
                    let upd_j = (fj as i8 + incr_j) as usize;
                    self.octopuses[upd_i][upd_j] += 1;
                    let neighbour = self.octopuses[upd_i][upd_j];
                    if neighbour == 10 {
                        to_flash.push((upd_i, upd_j))
                    }
                }
            }
        }

        // Reset everything that flashed to 0.
        for (i, j) in &flashed {
            self.octopuses[*i][*j] = 0;
        }

        flashed.len() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day11.dat");
        let mut grid = OctoGrid::new(data);
        let flashed_100 = grid.step_n(100);
        assert_eq!(flashed_100, 1656);
        let mut grid2 = OctoGrid::new(data);
        let synchronized_flash = grid2.first_synchronized(100);
        assert_eq!(synchronized_flash, 195);
    }
}

