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
    // Data can either be 0 (.), 1 (#), or 2 unset yet.
    data: Vec<Vec<u8>>,
    buffering: usize,
    inf_value: u8,
}

fn bools_to_int(from: &[u8; 9], inf_value: u8) -> usize {
    let mut power = 256;
    let mut result = 0;
    for i in 0..9 {
        let digit = if from[i] > 1 { inf_value } else { from[i] };

        if digit != 0 {
            result += power;
        }
        power /= 2;
    }
    result
}

impl ScannerData {
    #[allow(dead_code)]
    fn print(&self) -> String {
        let mut lines = vec![];
        for l in &self.data {
            let line = l
                .iter()
                .map(|c| {
                    if *c == 0u8 {
                        "."
                    } else if *c == 1u8 {
                        "#"
                    } else {
                        "_"
                    }
                })
                .collect::<Vec<&str>>();
            lines.push(line.join(""));
        }
        lines.join("\n")
    }

    fn enhance(&mut self) -> usize {
        let mut to_write = vec![];
        let mut to_blank = vec![];
        let mut lit = to_write.len();
        self.buffering -= 1;

        // Fill our scratch buffer
        for i in (1 + self.buffering)..(self.data.len() - self.buffering - 1) {
            for j in (1 + self.buffering)..(self.data[i].len() - self.buffering - 1) {
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
                let val = bools_to_int(&bools, self.inf_value);
                if self.algorithm[val] {
                    // Note squares which are lit.
                    lit += 1;
                    if self.data[i][j] != 1 {
                        // Note this value as needing update.
                        to_write.push((i, j));
                    }
                } else if self.data[i][j] != 0 {
                    // This is an on square that we need to turn off.
                    to_blank.push((i, j));
                }
            }
        }

        // If the initial algorithm bit is zero, we also need to flip the value at infinity.
        if self.algorithm[0] {
            self.inf_value = if self.inf_value == 1 { 0 } else { 1 };
        }

        for w in to_write {
            self.data[w.0][w.1] = 1;
        }

        for w in to_blank {
            self.data[w.0][w.1] = 0;
        }

        lit
    }

    fn new(raw_data: &str, max_iters: usize) -> Self {
        let buffer_size = max_iters + 1;
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
                    data = vec![vec![3; data_size + 2 * buffer_size]; buffer_size]
                }
                // Collect this line.
                let mut data_line = vec![3; buffer_size];
                let mut read_data = l
                    .chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect::<Vec<u8>>();
                data_line.append(&mut read_data);
                data_line.append(&mut vec![3; buffer_size]);
                data.push(data_line);
            }
        }

        // We finally pad the data array.
        data.append(&mut vec![vec![3; data_size + 2 * buffer_size]; buffer_size]);

        Self {
            algorithm,
            data,
            buffering: max_iters,
            inf_value: 0,
        }
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
