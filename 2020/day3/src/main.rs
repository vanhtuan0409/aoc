use std::io::{self, BufRead};

fn is_tree(map: &Vec<String>, position: (usize, usize)) -> bool {
    let (row, col) = position;
    let col_bound = map.get(0).unwrap().len();
    map.get(row).unwrap().chars().nth(col % col_bound).unwrap() == '#'
}

fn traverse(map: &Vec<String>, right: usize, down: usize) -> usize {
    let mut position: (usize, usize) = (0, 0);
    let mut count = match is_tree(&map, position) {
        true => 1,
        false => 0,
    };
    while position.0 + down < map.len() {
        position = (position.0 + down, position.1 + right);
        count += match is_tree(&map, position) {
            true => 1,
            false => 0,
        }
    }
    count
}

fn main() {
    let map: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mul = slopes
        .into_iter()
        .map(|(right, down)| traverse(&map, right, down))
        .fold(1, |acc, x| acc * x);
    println!("count: {:?}", mul)
}
