use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
enum SlotType {
    Empty,
    Occupied,
    Floor,
}

type SlotMap = Vec<Vec<SlotType>>;

impl From<char> for SlotType {
    fn from(input: char) -> Self {
        match input {
            'L' => SlotType::Empty,
            '#' => SlotType::Occupied,
            '.' => SlotType::Floor,
            _ => unimplemented!("Unknown slot type"),
        }
    }
}

impl SlotType {
    fn to_char(self) -> char {
        match self {
            SlotType::Empty => 'L',
            SlotType::Occupied => '#',
            SlotType::Floor => '.',
        }
    }
}

fn count_first_occupied_by_slop(map: &SlotMap, position: (usize, usize), slop: (i32, i32)) -> u32 {
    let mut curr = (position.0 as i32 + slop.0, position.1 as i32 + slop.1);
    loop {
        let (row, col) = curr;
        if row < 0 || col < 0 {
            return 0;
        }
        let slot = map.get(row as usize).and_then(|it| it.get(col as usize));
        match slot {
            Some(SlotType::Occupied) => return 1,
            Some(SlotType::Floor) => {
                curr = (row + slop.0, col + slop.1);
                continue;
            }
            _ => return 0,
        }
    }
}

fn count_surround_occupied(map: &SlotMap, position: (usize, usize)) -> u32 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if (i, j) != (0, 0) {
                let slop = (i, j);
                count += count_first_occupied_by_slop(map, position, slop);
            }
        }
    }

    count
}

fn apply_rule(map: &SlotMap, position: (usize, usize)) -> SlotType {
    let current_slot = map.get(position.0).unwrap().get(position.1).unwrap();
    let surround_occupied = count_surround_occupied(map, position);
    match (current_slot, surround_occupied) {
        (SlotType::Empty, 0) => SlotType::Occupied,
        (SlotType::Occupied, x) if x >= 5 => SlotType::Empty,
        _ => current_slot.clone(),
    }
}

fn run_round(map: &Vec<Vec<SlotType>>) -> SlotMap {
    let mut new_map = map.clone();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let transformed = apply_rule(map, (row, col));
            new_map[row][col] = transformed;
        }
    }
    new_map
}

fn debug(map: &Vec<Vec<SlotType>>) {
    map.iter()
        .map(|row| {
            row.iter()
                .map(|slot| slot.clone().to_char())
                .collect::<String>()
        })
        .for_each(|row| println!("{}", row))
}

fn main() {
    let mut map = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let slots = line.chars().map(|c| c.into()).collect::<Vec<_>>();
            Some(slots)
        })
        .collect::<Vec<_>>();

    loop {
        let new_map = run_round(&map);
        if new_map == map {
            map = new_map;
            break;
        }
        map = new_map;
    }

    let seated: usize = map
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&slot| slot == &SlotType::Occupied)
                .count()
        })
        .sum();

    println!("{}", seated);
}
