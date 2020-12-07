use std::io::{self, BufRead};

struct Range(usize, usize);

impl Range {
    fn split(self) -> (Self, Self) {
        let Range(lower, upper) = self;
        let middle = lower + (upper - lower + 1) / 2;
        return (Self(lower, middle - 1), Self(middle, upper));
    }

    fn process(self, input: &[char]) -> Option<usize> {
        if input.len() == 0 {
            return match self.0 == self.1 {
                true => Some(self.0),
                false => {
                    println!("No more input but cannot locate seat");
                    None
                }
            };
        }

        let (lower, upper) = self.split();
        let (op, tail) = input.split_at(1);
        match op.get(0).unwrap() {
            'F' => lower.process(tail),
            'B' => upper.process(tail),
            'L' => lower.process(tail),
            'R' => upper.process(tail),
            _ => None,
        }
    }
}

fn main() {
    let mut ids = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let (row_input, col_input) = line.split_at(7);
            let row = Range(0, 127).process(&row_input.chars().collect::<Vec<_>>())?;
            let col = Range(0, 7).process(&col_input.chars().collect::<Vec<_>>())?;
            Some((row, col))
        })
        .filter(|(row, _)| *row != 0 && *row != 127)
        .map(|(row, col)| row * 8 + col)
        .collect::<Vec<_>>();
    ids.sort();

    for i in 0..ids.len() - 1 {
        let seat_id = ids.get(i).unwrap();
        let next_seat_id = ids.get(i + 1).unwrap();
        if next_seat_id - seat_id > 1 {
            println!("{}, {}", seat_id, next_seat_id);
        }
    }
}
