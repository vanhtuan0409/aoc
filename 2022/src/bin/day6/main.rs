use aoc_2022::get_input_file;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f: io::Result<File> = get_input_file!("input1.txt");
    let r = io::BufReader::new(f.unwrap());

    r.lines().map(|line| line.unwrap()).for_each(|line| {
        println!("======");
        let chars = line.chars().collect_vec();
        let (idx, _) = chars
            .windows(14)
            .enumerate()
            .find(|(_, window)| window.iter().all_unique())
            .unwrap();
        println!("original signal: {}. Idx {}", line, idx + 14);
    });
}
