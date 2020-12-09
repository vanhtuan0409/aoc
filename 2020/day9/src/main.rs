use std::collections::HashSet;
use std::io::{self, BufRead};

fn is_valid(nums: &[i32], target: i32) -> bool {
    let mut s = HashSet::new();
    for i in nums {
        if s.contains(&(target - i)) {
            return true;
        }
        s.insert(i);
    }
    false
}

fn part1(nums: &[i32], window_size: usize) -> Option<i32> {
    let found = nums
        .windows(window_size + 1)
        .find(|&window| !is_valid(&window[..window_size], window[window_size]))?;
    found.get(window_size).cloned()
}

fn part2(nums: &[i32], target: i32) -> Option<i32> {
    if nums.len() == 0 {
        return None;
    }

    let (mut left, mut right) = (0, 0);
    let mut sum = nums[0];
    loop {
        if sum == target {
            let window = &nums[left..=right];
            let min = window.iter().min().unwrap();
            let max = window.iter().max().unwrap();
            return Some(min + max);
        }

        if let Some(num) = nums.get(right + 1) {
            if sum + num <= target {
                // try to advance right
                sum += num;
                right += 1;
            } else if left < right {
                // try to advance left
                sum -= nums[left];
                left += 1;
            } else {
                // break if cannot advance
                break;
            }
        }
    }
    None
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| {
            let l = l.ok()?;
            l.parse::<i32>().ok()
        })
        .collect::<Vec<_>>();

    let window_size = 25;
    let invalid = part1(&lines, window_size).unwrap();
    let weakness = part2(&lines, invalid).unwrap();
    println!("{:?}", weakness);
}
