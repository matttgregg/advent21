use crate::utils;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn solve() {
    utils::print_day(8);
    let data = include_str!("data/day8.dat");
    let start = SystemTime::now();
    let uniques = count_uniques(data);
    let decoded_sum = decode(data);
    let timed = SystemTime::now().duration_since(start).unwrap();
    println!("Counting 1, 4, 7, 8 -> {}", utils::fmt_bright(&uniques));
    println!("Fully decoded sum -> {}", utils::fmt_bright(&decoded_sum));
    utils::print_duration(timed);
}

fn count_uniques(data: &str) -> i32 {
    let lines = data.lines().map(|l| l.split("|").collect::<Vec<&str>>()[1])
        .map(|l| l.split_whitespace().collect::<Vec<&str>>());

    let mut total_uniques = 0;
    for line in lines {
        for val in line {
            let incr = match val.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
            total_uniques += incr;
        }
    }
    total_uniques

}

fn decode(data: &str) -> usize {
    data.split("\n").map(|l| decode_line(l)).sum()
}

fn decode_line(data: &str) -> usize {
    // Get the front and back parts.
    let front_back: Vec<&str> = data.split("|").collect();
    let front: Vec<&str> = front_back[0].split_whitespace().collect();
    let back: Vec<&str> = front_back[1].split_whitespace().collect();

    let decoder = decode_numbers(front);

    // Now we have the decoder we can work out the represented digit.
    let mut result = 0;
    let mut power = 1000;

    for r in back {
        let repr = Representation::from(r);
        for (i, decoded) in decoder.iter().enumerate() {
            if repr.is(&decoded) {
               result += i * power;
            }
        }
        power /= 10;
    }

    result
}

fn decode_numbers(front: Vec<&str>) -> Vec<Representation> {
    let mut numbers = vec![Representation::from(""); 10];
    let mut segments = vec![Representation::from(""); 7];

    let mut maybe235 = vec![];
    for f in front {
        match f.len() {
            // First we decode using the front part. First identify the easy numbers.
            2 => numbers[1] = Representation::from(f),
            4 => numbers[4] = Representation::from(f),
            3 => numbers[7] = Representation::from(f),
            7 => numbers[8] = Representation::from(f),
            5 => maybe235.push(Representation::from(f)),
            _ => (),
        }
    }

    segments[0] = numbers[7].minus(&numbers[1]);

    // We can work out 3, as it differs in only one place from 2 and 5 (which differ by two from each other).
    if maybe235[0].intersection(&maybe235[1]).length() == 3 {
        numbers[3] = maybe235[2].clone();
    } else if maybe235[0].intersection(&maybe235[2]).length() == 3 {
        numbers[3] = maybe235[1].clone();
    } else {
        numbers[3] = maybe235[0].clone();
    }

    // This gives enough to work out a few more segments
    segments[6] = numbers[3].minus(&numbers[4]).minus(&segments[0]);
    segments[3] = numbers[3].minus(&numbers[1]).minus(&segments[0]).minus(&segments[6]);
    segments[1] = numbers[4].minus(&numbers[1]).minus(&segments[3]);

    // Can now work out which is 2 and which is 5.
    let s1 = segments[1].segments.keys().collect::<Vec<&char>>()[0];
    for maybe in maybe235 {
        if maybe.segments.contains_key(s1) {
            // This is the 5
            numbers[5] = maybe.clone();
        } else if !maybe.is(&numbers[3]) {
            // This is the number 2
            numbers[2] = maybe.clone();
        }
    }

    // Can now work out the remaining segments and numbers.
    segments[4] = numbers[2].minus(&numbers[3]);
    numbers[0] = numbers[8].minus(&segments[3]);
    numbers[6] = numbers[5].add(&segments[4]);
    numbers[9] = numbers[8].minus(&segments[4]);

    numbers
}

#[derive(Clone)]
struct Representation {
    segments: HashMap<char, bool>
}

impl Representation {
    fn from(s: &str) -> Self {
        let mut segments = HashMap::new();
        for c in s.chars() {
            segments.insert(c, true);
        }
        Self {
            segments
        }
    }

    fn length(&self) -> usize {
        self.segments.len()
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut segments: HashMap<char, bool> = HashMap::new();
        for c in self.segments.keys() {
            // Check whether the other also has this set.
            if *other.segments.get(c).unwrap_or(&false) {
                segments.insert(*c, self.segments[c]);
            }
        }
        Self{
            segments
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut segments: HashMap<char, bool> = HashMap::new();
        for c in self.segments.keys() {
            segments.insert(*c, self.segments[c]);
        }

        for c in other.segments.keys() {
            segments.insert(*c, other.segments[c]);
        }

        Self{
            segments
        }
    }

    fn is(&self, other: &Self) -> bool {
        // Two representation are equal if of the same length and intersection.
        self.intersection(other).length() == self.length() && self.length() == other.length()
    }

    fn minus(&self, other: &Self) -> Self {
        let mut segments: HashMap<char, bool> = HashMap::new();
        for c in self.segments.keys() {
            // Check whether the other has the segment set (in which case ignore it).
            if !other.segments.get(c).unwrap_or(&false) {
                segments.insert(*c, self.segments[c]);
            }
        }
        Self{
            segments
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_data() {
        let data = include_str!("data/test_short8.dat");
        let uniques = count_uniques(data);
        let decoded_sum = decode(data);
        assert_eq!(uniques, 0);
        assert_eq!(decoded_sum, 5353);
    }

    #[test]
    fn test_data() {
        let data = include_str!("data/test_day8.dat");
        let uniques = count_uniques(data);
        let decoded_sum = decode(data);
        assert_eq!(uniques, 26);
        assert_eq!(decoded_sum, 61229);
    }
}
