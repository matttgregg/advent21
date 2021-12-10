use std::time::SystemTime;
use crate::{DayResult, DaySolver};

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day10.dat");
        let start = SystemTime::now();
        let (error_score, completion_score) = diagnose(data);
        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!("Error score of bad lines {}, completion score {}",
                                  error_score, completion_score);
        DayResult{
            description,
            part1: format!("{}", error_score),
            part2: format!("{}", completion_score),
            timing_us: timed.as_micros(),
        }
    }
}

enum LineDiagnosis {
    LineError(u64),
    Incomplete(Vec<char>),
}

fn diagnose(data: &str) -> (u64, u64) {
    let diagnoses = data.lines().map(diagnose_line);
    let mut error_score = 0;
    let mut completion_scores = vec![];

    for d in diagnoses {
        match d {
            LineDiagnosis::LineError(v) => error_score += v,
            LineDiagnosis::Incomplete(v) => completion_scores.push(score_completion(&v)),
        }
    }

    completion_scores.sort();
    // Rust round *up* in this case.
    (error_score, completion_scores[(completion_scores.len()/2)])
}

fn score_completion(incomplete: &Vec<char>) -> u64 {
    let mut score = 0;
    for c in incomplete.iter().rev() {
        score *= 5;
        match c {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => panic!("Unexpected '{}' in incompletion", c),
        }
    }
    score
}

fn diagnose_line(line: &str) -> LineDiagnosis {
    // We check char by char, keeping a stack.
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => if stack.pop().unwrap() != '(' { return LineDiagnosis::LineError(3); },
            ']' => if stack.pop().unwrap() != '[' { return LineDiagnosis::LineError(57); },
            '}' => if stack.pop().unwrap() != '{' { return LineDiagnosis::LineError(1197); },
            '>' => if stack.pop().unwrap() != '<' { return LineDiagnosis::LineError(25137); },
            _ => panic!("unexpected char {} in input", c),
        }
    }

    LineDiagnosis::Incomplete(stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day10.dat");
        let (error_score, completion_score) = diagnose(data);
        assert_eq!(error_score, 26397);
        assert_eq!(completion_score, 288957);
    }
}
