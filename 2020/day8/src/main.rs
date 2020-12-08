use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Op {
    Nop,
    Acc(i32),
    Jump(i32),
}

#[derive(Debug, Default)]
struct Program {
    offset: i32,
    ops: Vec<Op>,
    acc: i32,
    visited: HashSet<i32>,
}

impl Program {
    fn new(ops: Vec<Op>) -> Self {
        Self {
            ops,
            ..Self::default()
        }
    }

    fn process(&mut self, op: Op) {
        match op {
            Op::Nop => {
                self.offset += 1;
            }
            Op::Acc(val) => {
                self.acc += val;
                self.offset += 1;
            }
            Op::Jump(offset) => {
                self.offset += offset;
            }
        }
    }

    fn get_current_op(&self) -> Option<Op> {
        self.ops.get(self.offset as usize).cloned()
    }

    fn run(&mut self) -> Result<i32, ()> {
        loop {
            if self.visited.contains(&self.offset) {
                break;
            }
            self.visited.insert(self.offset.clone());
            let op = self.get_current_op().ok_or(())?;
            println!("Handle op at {} - {:?}", self.offset, op);
            self.process(op);
        }

        Ok(self.acc)
    }
}

fn parse_op(line: &str) -> Option<Op> {
    let parts = line.split(" ").collect::<Vec<_>>();
    let (&op, &val) = (parts.get(0)?, parts.get(1)?);
    let val = val.parse::<i32>().ok()?;
    match op {
        "nop" => Some(Op::Nop),
        "acc" => Some(Op::Acc(val)),
        "jmp" => Some(Op::Jump(val)),
        _ => None,
    }
}

fn main() {
    let ops = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            parse_op(&line)
        })
        .collect::<Vec<_>>();

    let mut p = Program::new(ops);
    let acc = p.run().unwrap();
    println!("acc {}", acc);
}
