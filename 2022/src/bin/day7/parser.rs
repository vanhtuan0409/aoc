use itertools::Itertools;
use pest::Parser;
use pest_derive::*;
use std::fs::File;
use std::io::prelude::*;

pub fn parse(f: &mut File) -> Result<Vec<Command>, ()> {
    let mut content = String::new();
    f.read_to_string(&mut content).map_err(|_| ())?;

    let parsed = InputParser::parse(Rule::program, &content)
        .map_err(|_| ())?
        .next()
        .ok_or(())?;

    let mut commands = Vec::<Command>::new();
    parsed
        .into_inner()
        .for_each(|execution| match execution.as_rule() {
            Rule::cd_command => {
                let dest = execution.into_inner().next().unwrap().as_str();
                commands.push(Command::Cd(dest.to_string()));
            }
            Rule::ls_command => {
                let entries = execution
                    .into_inner()
                    .map(|entry| match entry.as_rule() {
                        Rule::file_entry => LsEntry::from_file_entry(entry.as_str()).unwrap(),
                        Rule::dir_entry => LsEntry::from_dir_entry(entry.as_str()).unwrap(),
                        _ => unreachable!(),
                    })
                    .collect_vec();
                commands.push(Command::Ls(entries));
            }
            _ => {}
        });

    Ok(commands)
}

#[derive(Debug)]
pub enum LsEntry {
    FileEntry(u64, String),
    DirEntry(String),
}

impl LsEntry {
    fn from_file_entry(s: &str) -> Option<Self> {
        let mut parts = s.split(" ");
        let size: u64 = parts.next()?.parse().ok()?;
        let name = parts.next()?.to_string();
        Some(LsEntry::FileEntry(size, name))
    }
    fn from_dir_entry(s: &str) -> Option<Self> {
        let target = s.strip_prefix("dir ")?.to_string();
        Some(LsEntry::DirEntry(target))
    }
}

#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls(Vec<LsEntry>),
}

#[derive(Parser)]
#[grammar = "bin/day7/parser.pest"]
pub struct InputParser;
