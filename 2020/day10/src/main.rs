use std::collections::HashMap;
use std::io::{self, BufRead};

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

    let mut diffs: HashMap<u32, u32> = HashMap::new();
    let mut last = 0;
    for adapter in adapters {
        let diff = adapter - last;
        *diffs.entry(diff).or_insert(0) += 1;
        last = adapter
    }
    *diffs.entry(3).or_insert(0) += 1;

    println!(
        "{:?}",
        diffs.get(&1).unwrap_or(&0) * diffs.get(&3).unwrap_or(&0)
    );
}
