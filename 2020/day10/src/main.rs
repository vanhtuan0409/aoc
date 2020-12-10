use std::collections::HashMap;
use std::io::{self, BufRead};

fn count_paths(
    map: &mut HashMap<usize, u64>,
    adapters: &[u32],
    start_index: usize,
    max_diff: u32,
    target: usize,
) -> u64 {
    if start_index == target {
        return 1;
    }
    if let Some(result) = map.get(&start_index) {
        return result.clone();
    }

    let current_adapter = adapters[start_index];
    let next_moves = adapters
        .iter()
        .skip(start_index + 1)
        .enumerate()
        .take_while(|(_, adapter)| *adapter - current_adapter <= max_diff)
        .map(|(index, _)| start_index + index + 1)
        .collect::<Vec<_>>();
    let result = next_moves
        .into_iter()
        .rev()
        .map(|index| count_paths(map, adapters, index, max_diff, target))
        .sum();

    map.insert(start_index, result);
    result
}

fn main() {
    let mut adapters = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| {
            let l = l.ok()?;
            l.parse::<u32>().ok()
        })
        .collect::<Vec<_>>();
    adapters.sort();

    let max_diff = 3;
    let mut map = HashMap::new();
    let init_moves = adapters
        .iter()
        .enumerate()
        .take_while(|(_, adapter)| **adapter <= max_diff)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let count: u64 = init_moves
        .into_iter()
        .rev()
        .map(|index| count_paths(&mut map, &adapters, index, max_diff, adapters.len() - 1))
        .sum();

    println!("{}", count);
}
