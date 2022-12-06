use aoc_2022::get_input_file;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f: io::Result<File> = get_input_file!("input1.txt");
    let r = io::BufReader::new(f.unwrap());

    let sum = r
        .lines()
        .map(|line| line.unwrap())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(uniq_char)
                .reduce(|accum, item| accum.intersection(&item).map(|c| c.to_owned()).collect())
        })
        .flatten()
        .map(|hset| hset.iter().next().unwrap().to_owned())
        .map(priority_calc)
        .sum::<u16>();

    println!("sum: {}", sum)
}

fn split_comparment(input: String) -> (String, String) {
    let l = input.len();
    let (left, right) = input.split_at(l / 2);
    (left.to_string(), right.to_string())
}

fn uniq_char(input: String) -> HashSet<char> {
    input.chars().unique().collect()
}

fn priority_calc(c: char) -> u16 {
    if !c.is_alphabetic() {
        println!("invalid alphabet common char `{}`", c);
        return 0;
    }

    let ascii_code = c as u8;
    if c.is_lowercase() {
        (ascii_code - ('a' as u8) + 1) as u16
    } else {
        (ascii_code - ('A' as u8) + 27) as u16
    }
}
