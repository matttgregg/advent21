use std::time::SystemTime;
use std::collections::HashMap;
use crate::{DayResult, DaySolver};

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let test_data = include_str!("./data/day4.dat");
        let mut game = BingoGame::from_input(test_data);
        let start = SystemTime::now();
        let (first, last) = game.play();
        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("First winning score is {}, last winning score is {}",
                 first,
                 last);

        DayResult{
            description,
            part1: format!("{}", first),
            part2: format!("{}", last),
            timing_us: timed.as_micros(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let test_data = include_str!("./data/test_day4.dat");
        let mut game = BingoGame::from_input(test_data);
        let (first, last) = game.play();
        assert_eq!(first, 4512);
        assert_eq!(last, 1924);
    }
}

#[derive(Debug)]
struct Board {
    vals_to_row: HashMap<i32, i32>,
    vals_to_col: HashMap<i32, i32>,
    vals_to_call: HashMap<i32, i32>,
    called_in_row: HashMap<i32, i32>,
    called_in_col: HashMap<i32, i32>,
    rows: i32,
    cols: i32,
}

impl Board {
    fn empty() -> Self {
        Board {
            vals_to_row: Default::default(),
            vals_to_col: Default::default(),
            vals_to_call: Default::default(),
            called_in_row: Default::default(),
            called_in_col: Default::default(),
            rows: 0,
            cols: 0
        }
    }

    // Call a single value on this board. If it completes a row/column, return a score.
    fn call(&mut self, val: i32) -> i32 {
        let val_called = self.vals_to_call.entry(val).or_insert(0);
        if *val_called == 0 {
            // We don't have this value, so just return zero.
            return 0;
        }

        // We cross this value off the card.
        *val_called -= 1;

        let row = self.vals_to_row[&val];
        let col = self.vals_to_col[&val];

        let called_in_col = self.called_in_col.entry(col).or_insert(0);
        *called_in_col += 1;
        let called_in_row = self.called_in_row.entry(row).or_insert(0);
        *called_in_row += 1;

        //println!("Called: {} at R/C {}/{}. Col completion {}/{}. Row completion {}/{}",
        //    val, row, col, called_in_col, self.rows, called_in_row, self.cols
        //);

        if *called_in_col == self.rows || *called_in_row == self.cols {
            // Work out the score and return that.
            let mut score = 0;
            for (to_call, count) in self.vals_to_call.iter() {
                score += to_call *  count;
            }
            score * val
        } else {
            0
        }
    }

}

#[derive(Debug)]
struct BingoGame {
    boards: Vec<Board>,
    calls: Vec<i32>,
}

#[derive(Debug)]
enum ParsingMode {
    ParseCalls,
    ParseBoard,
}

impl BingoGame {
    // Play the game to completion.
    fn play(&mut self) -> (i32, i32) {
        let mut ignored: Vec<bool> = vec![false; self.boards.len()];
        let mut last_score = 0;
        let mut first_score = 0;
        for call in &self.calls {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if ignored[i] {
                    continue;
                }
                let scored = board.call(*call);
                if scored > 0 {
                    if first_score == 0 {
                        first_score = scored;
                    }
                    ignored[i] = true;
                    last_score = scored;
                }
            }
        }
        (first_score, last_score)
    }

    fn from_input(inp: &str) -> Self {
        // The initial row is the calls, the remainder are the boards.
        let mut parse_mode = ParsingMode::ParseCalls;
        let mut calls = vec![];
        let mut current_board = Board::empty();
        let mut boards = vec![];
        let mut row = 0;
        for line in inp.split("\n") {
            match parse_mode {
                ParsingMode::ParseCalls => {
                    // We expect a line of comma separated calls.
                    calls = line.split(",").map(|x|  x.parse::<i32>().unwrap()).collect();
                    parse_mode = ParsingMode::ParseBoard;
                },
                ParsingMode::ParseBoard => {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() <= 1 {
                        if current_board.rows > 0 {
                            // If we've put anything into this board, store it and start a new one.
                            boards.push(current_board);
                            current_board = Board::empty();
                        }
                        row = 0;
                    } else {
                        // We add to the current board.
                        for (col, val) in parts.iter().map(|x| x.parse::<i32>().unwrap()).enumerate() {
                            let to_call = current_board.vals_to_call.entry(val).or_insert(0);
                            *to_call += 1;
                            current_board.vals_to_row.insert(val, row);
                            current_board.vals_to_col.insert(val, col as i32);

                            if (col + 1) as i32 >= current_board.cols {
                                current_board.cols = (col + 1) as i32;
                            }
                        }
                        row += 1;
                        current_board.rows = row;
                    }
                },
            }
        }

        if current_board.rows > 0 {
            boards.push(current_board);
        }

        BingoGame {
            calls,
            boards,
        }
    }
}