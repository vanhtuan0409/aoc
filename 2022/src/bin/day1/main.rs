use aoc_2022::get_input_file;
use itertools::*;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f: io::Result<File> = get_input_file!("input1.txt");
    let r = io::BufReader::new(f.unwrap());
    let sum = r
        .lines()
        .map(|line| line.unwrap())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(is_sep, _)| !is_sep)
        .map(|(_, g)| {
            g.map(|l| {
                l.parse::<i32>().unwrap_or_else(|err| {
                    println!("cannot parse int. ERR: {}", err);
                    0
                })
            })
            .collect_vec()
        })
        .map(|elf| elf.iter().sum::<i32>())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum::<i32>();

    println!("sum: {}", sum)
}
