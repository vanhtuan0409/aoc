use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Debug, Default)]
struct Input {
    bag_name: String,
    includes: Vec<(String, usize)>,
}

fn parse_input(input: &str) -> Option<Input> {
    lazy_static! {
        static ref INPUT: Regex =
            Regex::new(r"(?m)^(?P<source>.+) bags? contain (?P<includes>.*)\.$").unwrap();
    }
    let caps: Captures = INPUT.captures(input)?;
    let bag_name = caps.name("source")?.as_str().to_string();
    let includes = caps.name("includes")?.as_str().to_string();
    if includes == "no other bags" {
        return Some(Input {
            bag_name,
            includes: vec![],
        });
    }

    let includes: Vec<_> = includes
        .split(",")
        .map(|p| p.trim())
        .filter_map(|p| {
            lazy_static! {
                static ref INCLUDE: Regex =
                    Regex::new(r"(?m)^(?P<count>\w+) (?P<bag>.*) bags?$").unwrap();
            }
            let caps: Captures = INCLUDE.captures(p)?;
            let quantity = caps.name("count")?.as_str().parse::<usize>().ok()?;
            let bag_name = caps.name("bag")?.as_str().to_string();
            Some((bag_name, quantity))
        })
        .collect();
    Some(Input { bag_name, includes })
}

struct State {
    node: String,
    visited: HashSet<String>,
    path: Vec<String>,
}

// perform DFS to find the shiny gold bag
fn part1(graph: &HashMap<String, Vec<(String, usize)>>) {
    // init states
    let mut stack: Vec<State> = vec![];
    let mut start_node = HashSet::new();
    for node in graph.keys() {
        let mut visited = HashSet::new();
        visited.insert(node.clone());
        stack.push(State {
            node: node.clone(),
            visited,
            path: vec![node.clone()],
        });
    }

    // dfs on stack
    while let Some(state) = stack.pop() {
        if state.node == "shiny gold" {
            if state.path.len() > 1 {
                start_node.insert(state.path.first().unwrap().clone());
                println!("{:?}", state.path);
            }
            continue;
        }

        // generate next states
        let mut next_states = graph
            .get(&state.node)
            .unwrap_or(&vec![])
            .iter()
            .filter(|&it| !state.visited.contains(&it.0))
            .map(|next_node| {
                let mut visited = state.visited.clone();
                visited.insert(next_node.0.clone());
                let mut path = state.path.clone();
                path.push(next_node.0.clone());
                State {
                    node: next_node.0.clone(),
                    visited,
                    path,
                }
            })
            .collect::<Vec<_>>();

        stack.append(&mut next_states);
    }

    println!("answers {:?}", start_node.len());
}

fn count(graph: &HashMap<String, Vec<(String, usize)>>, bag: &str) -> usize {
    println!("Checking bag {}", bag);
    let node = graph.get(bag);
    if node.is_none() {
        return 0;
    }
    let children = node.unwrap();
    if children.len() == 0 {
        return 1;
    }

    let mut total = 1;
    for child in children {
        total += child.1 * count(graph, &child.0);
    }

    total
}

fn part2(graph: &HashMap<String, Vec<(String, usize)>>) {
    let count = count(graph, "shiny gold") - 1;
    println!("count {}", count);
}

fn main() {
    let inputs = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            parse_input(&line)
        })
        .collect::<Vec<_>>();

    // build graph
    let mut graph: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for input in inputs {
        let connected = input.includes.into_iter().collect::<Vec<_>>();
        graph.insert(input.bag_name, connected);
    }

    part2(&graph);
}
