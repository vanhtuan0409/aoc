use aoc_2022::get_input_file;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f: io::Result<File> = get_input_file!("input1.txt");
    let r = io::BufReader::new(f.unwrap());
    let score = r
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            Game::from_str(&line).map_err(|err| {
                println!("Cannot parse game. Line `{}`", line);
                err
            })
        })
        .flatten()
        .map(|game| {
            if true {
                println!(
                    "game: {:?} - me: {:?} - score: {}",
                    game,
                    game.me(),
                    game.score()
                );
            }
            game.score()
        })
        .sum::<u32>();

    println!("score: {}", score);
}

#[derive(Debug)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn from_opponent(input: &str) -> Result<Self, ()> {
        match input.to_lowercase().as_str() {
            "a" => Ok(Self::Rock),
            "b" => Ok(Self::Paper),
            "c" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }

    fn from_me(input: &str) -> Result<Self, ()> {
        match input.to_lowercase().as_str() {
            "x" => Ok(Self::Rock),
            "y" => Ok(Self::Paper),
            "z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn combat(&self, other: &Self) -> GameResult {
        match (self, other) {
            (Self::Rock, Self::Scissors) => GameResult::Win,
            (Self::Rock, Self::Rock) => GameResult::Draw,
            (Self::Rock, Self::Paper) => GameResult::Lose,

            (Self::Paper, Self::Rock) => GameResult::Win,
            (Self::Paper, Self::Paper) => GameResult::Draw,
            (Self::Paper, Self::Scissors) => GameResult::Lose,

            (Self::Scissors, Self::Paper) => GameResult::Win,
            (Self::Scissors, Self::Scissors) => GameResult::Draw,
            (Self::Scissors, Self::Rock) => GameResult::Lose,
        }
    }

    fn reverse_action_from_opponent(opponent: &Self, result: &GameResult) -> Action {
        match (opponent, result) {
            (Self::Rock, GameResult::Win) => Self::Paper,
            (Self::Rock, GameResult::Draw) => Self::Rock,
            (Self::Rock, GameResult::Lose) => Self::Scissors,

            (Self::Paper, GameResult::Win) => Self::Scissors,
            (Self::Paper, GameResult::Draw) => Self::Paper,
            (Self::Paper, GameResult::Lose) => Self::Rock,

            (Self::Scissors, GameResult::Win) => Self::Rock,
            (Self::Scissors, GameResult::Draw) => Self::Scissors,
            (Self::Scissors, GameResult::Lose) => Self::Paper,
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn from_str(input: &str) -> Result<Self, ()> {
        match input.to_lowercase().as_str() {
            "x" => Ok(Self::Lose),
            "y" => Ok(Self::Draw),
            "z" => Ok(Self::Win),
            _ => Err(()),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

#[derive(Debug)]
struct Game {
    opponent: Action,
    result: GameResult,
}

impl Game {
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut parts = s.split_whitespace();
        Ok(Self {
            opponent: Action::from_opponent(parts.next().ok_or(())?)?,
            result: GameResult::from_str(parts.next().ok_or(())?)?,
        })
    }

    fn me(&self) -> Action {
        Action::reverse_action_from_opponent(&self.opponent, &self.result)
    }

    fn score(&self) -> u32 {
        self.result.score() + self.me().score()
    }
}
