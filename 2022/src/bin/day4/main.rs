use aoc_2022::get_input_file;
use itertools::Itertools;
use std::cmp::Ord;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let f: io::Result<File> = get_input_file!("input1.txt");
    let r = io::BufReader::new(f.unwrap());

    let sum = r
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<Assignment>())
        .flatten()
        .filter(|it| it.is_overlapped())
        .map(|it| {
            println!("{:?}", it);
            it
        })
        .count();

    println!("sum {}", sum)
}

#[derive(Debug)]
struct CleanRange(usize, usize);

impl CleanRange {
    fn is_fully_contain(&self, other: &Self) -> bool {
        (self.0 <= other.0) && (self.1 >= other.1)
    }

    fn is_overlapped(&self, other: &Self) -> bool {
        std::cmp::max(self.0, other.0) <= std::cmp::min(self.1, other.1)
    }

    fn len(&self) -> usize {
        self.1 - self.0 + 1
    }
}

impl FromStr for CleanRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start: usize = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let end: usize = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(CleanRange(start, end))
    }
}

#[derive(Debug)]
struct Assignment {
    l: Vec<CleanRange>,
}

impl Assignment {
    fn is_fully_contain(&self) -> bool {
        let left = self.l.get(0).unwrap();
        let right = self.l.get(1).unwrap();
        left.is_fully_contain(right)
    }

    fn is_overlapped(&self) -> bool {
        let left = self.l.get(0).unwrap();
        let right = self.l.get(1).unwrap();
        left.is_overlapped(right)
    }
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',');
        Ok(Assignment {
            l: parts
                .map(CleanRange::from_str)
                .flatten()
                .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()))
                .collect(),
        })
    }
}
