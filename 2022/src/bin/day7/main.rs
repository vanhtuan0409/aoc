use aoc_2022::get_input_file;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
mod parser;

fn main() {
    let mut f: File = get_input_file!("input1.txt").unwrap();
    let commands = parser::parse(&mut f).unwrap();

    let mut state = GameState::new();
    commands
        .iter()
        .for_each(|command| state.handle_command(command));

    let total_used = state.stats.get("/").unwrap().clone();
    let threshold = 70000000 - 30000000;
    let delete_threshold = total_used - threshold;
    let delete_entry = state
        .stats
        .iter()
        .map(|entry| {
            println!("{} - {}", entry.0, entry.1);
            entry
        })
        .filter(|(_, size)| **size >= delete_threshold)
        .sorted_by(|a, b| std::cmp::Ord::cmp(a.1, b.1))
        .next();

    println!("== total used: {}", total_used);
    println!("== threshold: {}", threshold);
    println!("== delete threshold: {}", delete_threshold);
    println!("== target: {:?}", delete_entry);
}

struct GameState {
    stats: HashMap<String, u64>,
    dir_stack: Vec<String>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            stats: HashMap::new(),
            dir_stack: vec!["/".to_string()],
        }
    }

    fn current_dir(&self) -> String {
        self.dir_stack.join("/")
    }

    fn handle_command(&mut self, command: &parser::Command) {
        match command {
            parser::Command::Cd(dest) => match dest.as_str() {
                "/" => {
                    self.dir_stack = vec!["/".to_string()];
                }
                ".." => {
                    self.dir_stack.pop();
                }
                dest => {
                    self.dir_stack.push(dest.to_string());
                }
            },
            parser::Command::Ls(entries) => {
                entries.into_iter().for_each(|entry| match entry {
                    parser::LsEntry::FileEntry(size, _) => {
                        for i in 1..=self.dir_stack.len() {
                            let path = self.dir_stack.iter().take(i).join("/");
                            *self.stats.entry(path).or_insert(0) += size
                        }
                    }
                    _ => {}
                });
            }
        }
    }
}
