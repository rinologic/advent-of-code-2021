use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file_reader(filename: &str) -> BufReader<File> {
    let measurements = File::open(filename).unwrap();
    return BufReader::new(measurements);
}

fn part_one() {
    let mut _hp: i32 = 0;     // horizontal position
    let mut _depth: i32 = 0;  // depth
    let reader = get_file_reader("map.txt");
    for (_index, line) in reader.lines().enumerate() {
        let direction = line.unwrap();
        let v: Vec<&str> = direction.split(" ").collect();
        let direction = v[0];
        let distance = v[1];
        //println!("{} = {}", direction, distance);
        let distance_int: i32 = distance.parse().unwrap();
        if direction == "up" {
            _depth -= distance_int;
        }
        if direction == "down" {
            _depth += distance_int;
        }
        if direction == "forward" {
            _hp += distance_int;
        }
    }
    println!("HP: {} | DEPTH: {} | MULTIPLIED: {}", _hp, _depth, _hp * _depth);
}

fn part_two() {
    let mut _hp: i32 = 0;     // horizontal position
    let mut _depth: i32 = 0;  // depth
    let mut _aim: i32 = 0;    // aim
    let reader = get_file_reader("map.txt");
    for (_index, line) in reader.lines().enumerate() {
        let direction = line.unwrap();
        let v: Vec<&str> = direction.split(" ").collect();
        let direction = v[0];
        let distance = v[1];
        //println!("{} = {}", direction, distance);
        let distance_int: i32 = distance.parse().unwrap();
        if direction == "up" {
            _aim -= distance_int;
        }
        if direction == "down" {
            _aim += distance_int;
        }
        if direction == "forward" {
            _hp += distance_int;
            _depth += _aim * distance_int;
        }
    }
    println!("HP: {} | DEPTH: {} | MULTIPLIED: {}", _hp, _depth, _hp * _depth);
}

fn main() {
    part_one();
    part_two();
}
