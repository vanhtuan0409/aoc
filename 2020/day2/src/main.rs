extern crate regex;

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Default)]
struct Condition {
    lower: usize,
    upper: usize,
    character: char,
}

fn parse_line(line: &str) -> Result<(Condition, String), ()> {
    lazy_static! {
        static ref INPUT: Regex =
            Regex::new(r"(?m)^(?P<lower>\d+)-(?P<upper>\d+) (?P<character>\w): (?P<pass>.*)$")
                .unwrap();
    }
    let caps: Captures = INPUT.captures(line).ok_or(())?;
    let cond = Condition {
        lower: caps
            .name("lower")
            .ok_or(())?
            .as_str()
            .parse::<usize>()
            .map_err(|_| ())?,
        upper: caps
            .name("upper")
            .ok_or(())?
            .as_str()
            .parse::<usize>()
            .map_err(|_| ())?,
        character: caps
            .name("character")
            .ok_or(())?
            .as_str()
            .chars()
            .next()
            .ok_or(())?,
    };
    let pass = caps.name("pass").ok_or(())?.as_str().to_string();
    Ok((cond, pass))
}

#[allow(dead_code)]
fn part1(cond: &Condition, pass: &str) -> bool {
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in pass.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    match freq.get(&cond.character) {
        Some(&count) => count >= cond.lower && count <= cond.upper,
        None => false,
    }
}

fn part2(cond: &Condition, pass: &str) -> bool {
    let chars: Vec<_> = pass.chars().collect();
    match (chars.get(cond.lower - 1), chars.get(cond.upper - 1)) {
        (Some(&c), None) => c == cond.character,
        (None, Some(&c)) => c == cond.character,
        (Some(&c1), Some(&c2)) => {
            (c1 == cond.character && c2 != cond.character)
                || (c1 != cond.character && c2 == cond.character)
        }
        (None, None) => false,
    }
}

fn main() {
    let count = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            parse_line(&line).ok()
        })
        .filter(|(cond, pass)| part2(cond, pass))
        .count();
    println!("{}", count);
}
