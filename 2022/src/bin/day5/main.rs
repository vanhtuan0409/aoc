use aoc_2022::get_input_file;
use itertools::Itertools;
use lazy_static::lazy_static;
use pest::Parser;
use pest_derive::*;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
    let f: io::Result<File> = get_input_file!("input1.txt");
    let mut f = f.expect("Cannot read input file");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("Cannot read input file");

    let game_input = InputParser::parse(Rule::input, &content)
        .expect("Cannot parse rules")
        .next()
        .unwrap();
    let mut game = Game::new();

    // parsing
    game_input
        .into_inner()
        .for_each(|line| match line.as_rule() {
            Rule::crate_line => line
                .into_inner()
                .enumerate()
                .filter(|(_, block)| block.as_rule() == Rule::crate_block)
                .for_each(|(idx, block)| {
                    let block_name = block
                        .as_str()
                        .strip_prefix("[")
                        .and_then(|s| s.strip_suffix("]"))
                        .unwrap();
                    game.stacks[idx].push(block_name.to_string());
                }),
            Rule::command => {
                let command: Command = line.as_str().parse().unwrap();
                game.commands.push(command);
            }
            _ => (),
        });

    // reverse stack
    game.stacks.iter_mut().for_each(|stack| {
        stack.reverse();
    });

    println!("{:?}", game.solve())
}

#[derive(Debug)]
struct Game {
    stacks: Vec<Vec<String>>,
    commands: Vec<Command>,
}

impl Game {
    fn new() -> Self {
        Self {
            stacks: vec![vec![]; 20], // preallocate a list of stack
            commands: vec![],
        }
    }

    fn solve(&mut self) -> Result<String, ()> {
        // moving stuffs
        self.commands.iter().for_each(|command| {
            let count = command.count;
            for _ in 0..count {
                let taken = self.stacks[command.from_stack].pop().unwrap();
                self.stacks[command.to_stack].push(taken);
            }
        });

        let s = self.stacks.iter().map_while(|stack| stack.last()).join("");
        Ok(s)
    }
}

#[derive(Debug)]
struct Command {
    count: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref COMMAND_RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }

        let groups = COMMAND_RE.captures(s).ok_or(())?;
        let count: usize = groups.get(1).ok_or(())?.as_str().parse().map_err(|_| ())?;
        let from_stack: usize = groups.get(2).ok_or(())?.as_str().parse().map_err(|_| ())?;
        let to_stack: usize = groups.get(3).ok_or(())?.as_str().parse().map_err(|_| ())?;
        Ok(Command {
            count,
            from_stack: from_stack - 1,
            to_stack: to_stack - 1,
        })
    }
}

#[derive(Parser)]
#[grammar = "bin/day5/parser.pest"]
pub struct InputParser;
