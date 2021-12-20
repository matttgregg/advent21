use crate::{DayResult, DaySolver};
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day20.dat");
        let start = SystemTime::now();

        let mut scan_data = ScannerData::new(data, 50);
        let mut after_2 = 0;
        let mut final_count = 0;
        for i in 1..=50 {
            if i == 2 {
                after_2 = scan_data.enhance();
            } else {
                final_count = scan_data.enhance();
            }
        }

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!(
            "After 2 enhancements, {} lit, after 50 {}",
            after_2, final_count
        );

        DayResult {
            description,
            part1: format!("{}", after_2),
            part2: format!("{}", final_count),
            timing_us: timed.as_micros(),
        }
    }
}

struct ScannerData {
    algorithm: Vec<bool>,
    data: Vec<Vec<bool>>,
}

fn bools_to_int(from: &[bool; 9]) -> usize {
    let mut power = 256;
    let mut result = 0;
    for i in 0..9 {
        if from[i] {
            result += power;
        }
        power /= 2;
    }
    result
}

impl ScannerData {
    fn enhance(&mut self) -> usize {
        let mut to_write = vec![];
        let mut to_blank = vec![];
        let mut lit = to_write.len();

        // Fill our scratch buffer
        for i in 1..(self.data.len() - 1) {
            let mut row_string = vec![];
            for j in 1..(self.data[i].len() - 1) {
                // Read the number.
                let bools = [
                    self.data[i - 1][j - 1],
                    self.data[i - 1][j],
                    self.data[i - 1][j + 1],
                    self.data[i][j - 1],
                    self.data[i][j],
                    self.data[i][j + 1],
                    self.data[i + 1][j - 1],
                    self.data[i + 1][j],
                    self.data[i + 1][j + 1],
                ];
                let val = bools_to_int(&bools);
                row_string.push(format!("[{:3}]", val));
                if self.algorithm[val] {
                    // Note squares which are lit.
                    lit += 1;
                    if !self.data[i][j] {
                        // Note this value as needing update.
                        to_write.push((i, j));
                    }
                } else if self.data[i][j] {
                    // This is an on square that we need to turn off.
                    to_blank.push((i, j));
                }
            }
        }

        // If the initial algorithm bit is zero, we also need to flip the boundaries.
        let flipped = !self.data[0][0];
        if self.algorithm[0] {
            for i in 0..self.data.len() {
                let len = self.data[i].len();
                self.data[i][0] = flipped;
                self.data[i][len - 1] = flipped;
            }
            for i in 0..self.data[0].len() {
                self.data[0][i] = flipped;
            }
            let len = self.data.len();
            for i in 0..self.data[len - 1].len() {
                self.data[len - 1][i] = flipped;
            }
        }

        for w in to_write {
            self.data[w.0][w.1] = true;
        }

        for w in to_blank {
            self.data[w.0][w.1] = false;
        }

        lit
    }

    fn new(raw_data: &str, max_iters: usize) -> Self {
        let buffer_size = max_iters + 2;
        let mut data_size = 0;
        let mut data = vec![];
        let mut algorithm = vec![];
        let mut scanning_data = false;

        for l in raw_data.lines() {
            if l.is_empty() {
                scanning_data = true;
            } else if !scanning_data {
                algorithm = l.chars().map(|c| c == '#').collect();
            } else {
                if data.len() == 0 {
                    // Initialize with enough space for all iterations.
                    data_size = l.len();
                    data = vec![vec![false; data_size + 2 * buffer_size]; buffer_size]
                }
                // Collect this line.
                let mut data_line = vec![false; buffer_size];
                let mut read_data = l.chars().map(|c| c == '#').collect::<Vec<bool>>();
                data_line.append(&mut read_data);
                data_line.append(&mut vec![false; buffer_size]);
                data.push(data_line);
            }
        }

        // We finally pad the data array.
        data.append(&mut vec![
            vec![false; data_size + 2 * buffer_size];
            buffer_size
        ]);

        Self { algorithm, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day20.dat");
        let mut scan_data = ScannerData::new(data, 50);
        let mut after_2 = 0;
        let mut final_count = 0;
        for i in 1..=50 {
            if i == 2 {
                after_2 = scan_data.enhance();
            } else {
                final_count = scan_data.enhance();
            }
        }
        assert_eq!(after_2, 35);
        assert_eq!(final_count, 3351);
    }
}
