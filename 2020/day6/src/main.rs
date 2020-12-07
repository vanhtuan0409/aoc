#![feature(iterator_fold_self)]

use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().lock().read_to_string(&mut input);
    let groups = input.split("\n\n").collect::<Vec<_>>();
    let count = groups
        .iter()
        .map(|group| {
            let intersection = group
                .split("\n")
                .filter(|p| !p.is_empty())
                .map(|person| person.chars().collect::<HashSet<_>>())
                .fold_first(|acc, person| acc.intersection(&person).cloned().collect())
                .unwrap();
            intersection.len()
        })
        .sum::<usize>();

    println!("count {:?}", count);
}
