use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

struct Coordinate {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn load_input(filename: &str) -> BufReader<File> {
    let measurements = File::open(filename).unwrap();
    return BufReader::new(measurements);
}

// Given a pair of coordinates in this format: 0,9  5,9
// Return a coordinate object
fn parse_cords(coord_to_parse: &str) -> Coordinate {
    let re = Regex::new(r"^([0-9]+),([0-9]+)\s+->\s+([0-9]+),([0-9]+)$").unwrap();
    let cap = re.captures(coord_to_parse).unwrap();
    Coordinate {
        x1: (&cap[1]).parse().unwrap(),
        x2: (&cap[3]).parse().unwrap(),
        y1: (&cap[2]).parse().unwrap(),
        y2: (&cap[4]).parse().unwrap()
    }
}

fn main() {
    let reader = load_input("input.txt");
    let mut c = Coordinate { x1: 0, x2: 0, y1: 0, y2: 0 };
    for (_index, line) in reader.lines().enumerate() {
        c = parse_cords(&line.unwrap());
        println!("{},{}-{},{}", c.x1, c.y2, c.x2, c.y2);
    }
}
