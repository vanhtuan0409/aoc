use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::io::{self, Read};

macro_rules! unwrap_or_return {
    ($e: expr, $val: expr) => {
        match $e {
            Some(x) => x,
            None => return $val,
        }
    };
}

fn parse_passport(line: &str) -> HashMap<String, String> {
    let mut ret = HashMap::new();
    lazy_static! {
        static ref PASSPORT: Regex = Regex::new(r"(?m)^(?P<key>[^:]*):(?P<val>[^:]*)$").unwrap();
    }
    line.split(" ")
        .filter(|part| !part.is_empty())
        .for_each(|part| {
            let caps: Captures = PASSPORT.captures(part).unwrap();
            let (key, val) = (
                caps.name("key").unwrap().as_str().to_string(),
                caps.name("val").unwrap().as_str().to_string(),
            );
            ret.insert(key, val);
        });
    ret
}

fn year_validator(year: &str, min: usize, max: usize) -> Option<usize> {
    let year = year.parse::<usize>().ok()?;
    match year >= min && year <= max {
        true => Some(year),
        false => None,
    }
}

fn is_valid(passport: &HashMap<String, String>) -> bool {
    unwrap_or_return!(
        passport
            .get("byr")
            .and_then(|byr| year_validator(byr, 1920, 2002)),
        false
    );

    unwrap_or_return!(
        passport
            .get("iyr")
            .and_then(|iyr| year_validator(iyr, 2010, 2020)),
        false
    );

    unwrap_or_return!(
        passport
            .get("eyr")
            .and_then(|eyr| year_validator(eyr, 2020, 2030)),
        false
    );

    unwrap_or_return!(
        passport.get("hgt").and_then(|hgt| {
            lazy_static! {
                static ref HEIGHT: Regex =
                    Regex::new(r"(?m)^(?P<height>\d+)(?P<unit>in|cm)$").unwrap();
            };
            let caps: Captures = HEIGHT.captures(hgt)?;
            let (height, unit) = (
                caps.name("height")?.as_str().parse::<i32>().ok()?,
                caps.name("unit")?.as_str().to_string(),
            );
            match (height, unit.as_str()) {
                (_, "in") => match height >= 59 && height <= 76 {
                    true => Some(height),
                    false => {
                        println!("hgt invalid {}in", height);
                        None
                    }
                },
                (_, "cm") => match height >= 150 && height <= 193 {
                    true => Some(height),
                    false => {
                        println!("hgt invalid {}cm", height);
                        None
                    }
                },
                (_, _) => {
                    println!("hgt invalid {}{}", height, unit);
                    None
                }
            }
        }),
        false
    );

    unwrap_or_return!(
        passport.get("hcl").and_then(|hcl| {
            lazy_static! {
                static ref HCL: Regex = Regex::new(r"(?m)^#[0-9a-f]{6}$").unwrap();
            };
            match HCL.is_match(hcl) {
                true => Some(hcl),
                false => {
                    println!("hcl invalid {}", hcl);
                    None
                }
            }
        }),
        false
    );

    unwrap_or_return!(
        passport.get("ecl").and_then(|ecl| {
            match ecl == "amb"
                || ecl == "blu"
                || ecl == "brn"
                || ecl == "gry"
                || ecl == "grn"
                || ecl == "hzl"
                || ecl == "oth"
            {
                true => Some(ecl),
                false => {
                    println!("ecl invalid {}", ecl);
                    None
                }
            }
        }),
        false
    );

    unwrap_or_return!(
        passport.get("pid").and_then(|pid| {
            lazy_static! {
                static ref PID: Regex = Regex::new(r"(?m)^\d{9}$").unwrap();
            };
            match PID.is_match(pid) {
                true => Some(pid),
                false => {
                    println!("pid invalid {}", pid);
                    None
                }
            }
        }),
        false
    );

    true
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let count = input
        .split("\n\n")
        .map(|p| parse_passport(&p.replace("\n", " ")))
        .filter(|p| is_valid(p))
        .count();

    println!("count: {}", count);
}
