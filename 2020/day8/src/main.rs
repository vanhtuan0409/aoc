use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
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

    fn run(&mut self) -> Result<bool, ()> {
        loop {
            if self.visited.contains(&self.offset) {
                break;
            }
            self.visited.insert(self.offset.clone());
            let op = match self.get_current_op() {
                Some(op) => op,
                None => break,
            };
            println!("Handle op at {} - {:?}", self.offset, op);
            self.process(op);
        }

        Ok(self.offset == self.ops.len() as i32)
    }

    fn self_heal(&self) -> Vec<Self> {
        let mut ret = vec![];
        for (index, op) in self.ops.iter().enumerate() {
            if let Op::Jump(_) = op {
                let mut new_ops = self.ops.clone();
                new_ops[index] = Op::Nop; //replace jump with nop
                ret.push(Self {
                    ops: new_ops,
                    ..Self::default()
                });
            }
        }
        ret
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

    let p = Program::new(ops);
    let mut possibilities = p.self_heal();
    possibilities.push(p);

    for mut p in possibilities {
        println!("try 1 possibility");
        if let Ok(true) = p.run() {
            println!("==== found acc {}", p.acc);
            break;
        }
    }
}
