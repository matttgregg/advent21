use crate::{DayResult, DaySolver};
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day18.dat");
        let start = SystemTime::now();

        let (solved, magnitude) = solve_sums(data);
        let max_pair = max_pair(data);
        let description = format!(
            "Solved the homework to :: {}, with magnitude {} . Max pair across all sums is {} .",
            solved, magnitude, max_pair
        );
        let timed = SystemTime::now().duration_since(start).unwrap();

        DayResult {
            description,
            part1: format!("{}", magnitude),
            part2: format!("{}", max_pair),
            timing_us: timed.as_micros(),
        }
    }
}

fn max_pair(data: &str) -> i64 {
    let mut max_pair = 0;
    let lines = data.lines().collect::<Vec<&str>>();
    let lines_length = lines.len();

    for i in 0..lines_length {
        for j in 0..lines_length {
            // We can't add to self.
            if i != j {
                let sum_problem = format!("[{},{}]", lines[i], lines[j]);
                let mut math = SnailMath::new(&sum_problem);
                let _res = math.solve();
                let sum = math.magnitude();

                if sum > max_pair {
                    max_pair = sum;
                }
            }
        }
    }

    max_pair
}

fn solve_sums(data: &str) -> (String, i64) {
    let mut current = String::from("");
    for l in data.lines() {
        current = if current.is_empty() {
            // First step, just load and solve.
            String::from(l)
        } else {
            // Otherwise, we need to build the new string.
            let sum = format!("[{},{}]", current, l);
            SnailMath::new(&sum).solve()
        };
    }

    let magnitude = SnailMath::new(&current).magnitude();
    (current, magnitude)
}

struct SnailMath {
    left: Vec<char>,
    right: Vec<char>,
    depth: usize,
}

impl SnailMath {
    fn new(data: &str) -> Self {
        let mut right = data.chars().collect::<Vec<char>>();
        right.reverse();

        Self {
            left: vec![],
            right,
            depth: 0,
        }
    }

    fn left_string(&self) -> String {
        self.left
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn right_string(&self) -> String {
        self.right
            .iter()
            .rev()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn solve(&mut self) -> String {
        // Do explosions and splits.
        loop {
            while self.try_explode() {
                self.reset();
            }
            self.reset();

            if !self.try_split() {
                // We didn't do anything! So we're done.
                break;
            }

            // We performed a split - go back and try exploding again.
            self.reset();
        }

        self.left_string()
    }

    fn reset(&mut self) {
        while self.move_left().is_some() {}
        self.depth = 0;
    }

    fn try_split(&mut self) -> bool {
        // Step until depth reaches 4 or end.
        while self.peek().is_some() {
            // We only note the *first* digit in a number.
            if self.peek_is_number() && !self.peek_back_is_number() {
                // If it's a number > 9, we split and finish immediately.
                if self.peek_number() > 9 {
                    self.split();
                    return true;
                }
            }
            self.move_right();
        }
        false
    }

    fn split(&mut self) {
        let n = self.consume_number();
        let l = n / 2;
        let r = if n % 2 == 0 { l } else { l + 1 };
        self.left.push('[');
        self.push_number(l);
        self.left.push(',');
        self.push_number(r);
        self.left.push(']');

        // Now consume to the end, and finish.
        while self.move_right().is_some() {}
    }

    fn try_explode(&mut self) -> bool {
        let mut since_last_digit = 0;
        // Step until depth reaches 4 or end.
        while self.peek().is_some() && self.depth < 5 {
            // We only note the *first* digit in a number.
            if self.peek_is_number() && !self.peek_back_is_number() {
                since_last_digit = 0;
            }
            self.move_right();
            since_last_digit += 1;
        }

        if self.depth != 5 {
            // We didn't find anything to explode.
            return false;
        }

        // We've just consumed the opening '[' of a pair.
        self.left.pop();
        let (l, r) = self.consume_number_pair();
        // Consume the closing ']'
        assert_eq!(self.consume_right(), Some(']'));

        // Handle the last left number.
        if since_last_digit < self.left.len() {
            since_last_digit -= 1;
            for _ in 0..since_last_digit {
                self.move_left();
            }

            let last_left = self.consume_number();
            let number_len = last_left.to_string().chars().count();
            self.push_number(last_left + l);
            // Get back to the right position, accounting for the number that we got rid of.
            for _ in 0..(since_last_digit - number_len) {
                self.move_right();
            }
        }

        // Replace with a zero.
        self.left.push('0');

        // Find next right number.
        while self.peek().is_some() && !self.peek_is_number() {
            self.move_right();
        }

        // If we've got a number, do the increment.
        if self.peek_is_number() {
            let next_right = self.consume_number();
            self.push_number(next_right + r);
        }

        // Move everything to the left.
        while self.peek().is_some() {
            self.move_right();
        }

        true
    }

    fn push_number(&mut self, n: i64) {
        for c in n.to_string().chars() {
            self.left.push(c);
        }
    }

    fn peek(&self) -> Option<&char> {
        self.right.last()
    }

    fn peek_back(&self) -> Option<&char> {
        self.left.last()
    }

    fn peek_back_is_number(&self) -> bool {
        if let Some(c) = self.peek_back() {
            c.is_digit(10)
        } else {
            false
        }
    }

    fn peek_is_number(&self) -> bool {
        if let Some(c) = self.peek() {
            c.is_digit(10)
        } else {
            false
        }
    }

    fn consume_number_pair(&mut self) -> (i64, i64) {
        self.inner_number_pair(true)
    }

    fn magnitude(&mut self) -> i64 {
        // We recursively collapse until we have a single number left.
        self.reset();
        while !self.peek_is_number() {
            while self.peek().is_some() {
                // Try consuming a number pair
                self.try_complete_number_pair();
            }

            self.reset();
        }
        // Should be left with just a number.
        self.right_string().parse::<i64>().unwrap()
    }

    fn try_complete_number_pair(&mut self) {
        if let Some('[') = self.peek() {
            self.consume_right();
            if self.peek_is_number() {
                let left = self.consume_number();
                if self.peek() == Some(&',') {
                    self.consume_right();
                    if self.peek_is_number() {
                        //  We have a complete number, but need to tidy up the terminating ']'
                        let right = self.consume_number();
                        assert_eq!(self.consume_right(), Some(']'));
                        self.push_number(3 * left + 2 * right);
                    } else {
                        // We've read 'l,' , so now need to put that back.
                        for c in format!("[{},", left).chars() {
                            self.left.push(c);
                        }
                    }
                } else {
                    // We've consumed a number, so now need to put it back
                    for c in format!("[{}", left).chars() {
                        self.left.push(c);
                    }
                }
            } else {
                self.left.push('[');
            }
        } else {
            self.move_right();
        }
    }

    fn inner_number_pair(&mut self, remove: bool) -> (i64, i64) {
        let left = self.inner_number(remove);
        assert_eq!(self.inner_right(remove), Some(','));
        let right = self.inner_number(remove);
        (left, right)
    }

    fn consume_number(&mut self) -> i64 {
        self.inner_number(true)
    }

    fn peek_number(&mut self) -> i64 {
        // Peek gets the next number without consuming or moving.
        let mut shifted = 0;
        let mut digits = vec![];
        while self.peek_is_number() {
            shifted += 1;
            let digit = self.move_right().unwrap().to_string();
            digits.push(digit);
        }

        for _ in 0..shifted {
            self.move_left();
        }

        digits.join("").parse::<i64>().unwrap()
    }

    fn inner_number(&mut self, remove: bool) -> i64 {
        let mut digits = vec![];
        while self.peek_is_number() {
            digits.push(self.inner_right(remove).unwrap().to_string())
        }
        digits.join("").parse::<i64>().unwrap()
    }

    fn consume_right(&mut self) -> Option<char> {
        self.inner_right(true)
    }

    fn move_right(&mut self) -> Option<char> {
        self.inner_right(false)
    }

    fn inner_right(&mut self, remove: bool) -> Option<char> {
        if let Some(c) = self.right.pop() {
            if !remove {
                self.left.push(c);
            }

            // Update depth if necessary.
            if c == '[' {
                self.depth += 1;
            } else if c == ']' {
                self.depth -= 1;
            }

            Some(c)
        } else {
            None
        }
    }

    fn move_left(&mut self) -> Option<char> {
        if let Some(c) = self.left.pop() {
            self.right.push(c);
            Some(c)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day18.dat");
        let (_solved, magnitude) = solve_sums(data);
        let max_pair = max_pair(data);
        assert_eq!(magnitude, 3488);
        assert_eq!(max_pair, 3805);
    }

    #[test]
    fn test_data_b() {
        let data = include_str!("data/test_day18b.dat");
        let (_solved, magnitude) = solve_sums(data);
        let max_pair = max_pair(data);
        assert_eq!(magnitude, 4140);
        assert_eq!(max_pair, 3993);
    }
}
