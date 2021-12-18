use crate::{DayResult, DaySolver};
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/test_day18.dat");
        let start = SystemTime::now();

        test_one();

        let timed = SystemTime::now().duration_since(start).unwrap();
        /*
        let solved = solve_sums(data);
        let magnitude = magnitude(&solved);
        let description = format!(
            "Solved the homework to :: {}, with magnitude {}",
            solved, magnitude
        );*/

        DayResult {
            description: String::from(""),
            part1: format!("{}", 0),
            part2: format!("{}", 0),
            timing_us: timed.as_micros(),
        }
    }
}

fn magnitude(data: &str) -> u64 {
    0
}

fn solve_sums(data: &str) -> String {
    let mut current = String::from("");
    for l in data.lines() {
        println!("\t{}\n+\t{}", current, l);
        current = if current.len() == 0 {
            // First step, just load and solve.
            String::from(l)
        } else {
            // Otherwise, we need to build the new string.
            let sum = format!("[{},{}]", current, l);
            SnailMath::new(&sum).solve()
        };
        println!("=\t{}", current);
    }
    println!("");
    current
}

fn test_one() {
    let mut math = SnailMath::new(
        "[[[[[4,3],4],4],[7,[[8,4],9]]], [1,1]]", //"[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
    );
    println!("= {}", math.solve());
}

fn test() {
    for test in [
        "[7,[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[[[0,7],4],[15,[0,13]]],[1,1]]",
        "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
    ]
    .iter()
    {
        let mut math = SnailMath::new(test);
        math.explode_or_split();
        println!("{} -> {}", test, math.left_string());
    }

    for test in ["[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"].iter() {
        let mut math = SnailMath::new(test);
        println!("{} -> {}", test, math.solve());
    }

    for test in [
        "[1,1]\n[2,2]\n[3,3]\n[4,4]",
        "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]",
        "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]",
    ]
    .iter()
    {
        println!("{} ==>\n\t {}", test, solve_sums(test));
    }
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
        println!("{}", self.right_string());
        while self.explode_or_split() {
            println!("=> {}", self.left_string());
            self.reset();
        }

        self.left_string()
    }

    fn reset(&mut self) {
        while self.move_left().is_some() {}
        self.depth = 0;
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

    fn explode_or_split(&mut self) -> bool {
        let mut since_last_digit = 0;
        // Step until depth reaches 4 or end.
        while self.peek().is_some() && self.depth < 5 {
            // We only note the *first* digit in a number.
            if self.peek_is_number() && !self.peek_back_is_number() {
                since_last_digit = 0;

                // If it's a number > 9, we split and finish immediately.
                if self.peek_number() > 9 {
                    self.split();
                    return true;
                }
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
        /*
        println!(
            "Will explode {}, {} :: {}/{} -<<- {}",
            l,
            r,
            self.left.len(),
            self.right.len(),
            since_last_digit
        );
         */

        // Handle the last left number.
        if since_last_digit < self.left.len() {
            since_last_digit -= 1;
            for _ in 0..since_last_digit {
                self.move_left();
            }

            /*
            println!(
                "Shiften back to: {}||{}",
                self.left_string(),
                self.right_string()
            );*/

            let last_left = self.consume_number();
            let number_len = last_left.to_string().chars().collect::<Vec<char>>().len();
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
    fn test_data() {}
}
