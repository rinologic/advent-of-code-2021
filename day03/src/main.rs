use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file_reader(filename: &str) -> BufReader<File> {
    let datafile = File::open(filename).unwrap();
    return BufReader::new(datafile);
}

fn part_one() {
    println!("Part One");
    println!("--------------------------------");
    let mut gamma_string: String = "".to_owned();
    let mut epsilon_string: String = "".to_owned();
    for b in 1..13 {
        let mut _zero: i32 = 0;
        let mut _one: i32 = 0;
        let reader = get_file_reader("data.txt");
        for (_index, _line) in reader.lines().enumerate() {
            let data = _line.unwrap();
            let v: Vec<&str> = data.split("").collect();
            let v_int: i32 = v[b].parse().unwrap();
            if v_int == 0 {
                _zero += 1;
            }
            if v_int == 1 {
                _one += 1;
            }
        }
        if _one > _zero {
            gamma_string.push_str("1");
            epsilon_string.push_str("0");
        }
        if _zero > _one {
            gamma_string.push_str("0");
            epsilon_string.push_str("1");
        }
    }
    println!("{}", gamma_string);
    println!("{}", epsilon_string);
    let gamma_dec = isize::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&epsilon_string, 2).unwrap();
    println!("{} * {}", gamma_dec, epsilon_dec);
    println!("{}" , gamma_dec * epsilon_dec);
}

fn part_two() {
    println!("Part Two");
    for b in 1..2 {
        let mut _zero: i32 = 0;
        let mut _one: i32 = 0;
        let reader = get_file_reader("sample_data.txt");
        let mut all_values: Vec<String> = Vec::new();
        let mut o2_values: Vec<String> = Vec::new();
        let mut co2_values: Vec<String> = Vec::new();
        for (_index, _line) in reader.lines().enumerate() {
            all_values.push(_line.unwrap());
        }
        o2_values = all_values.clone();
        co2_values = all_values.clone();

        for (_index, _line) in all_values.iter().enumerate() {
            let data = _line;
            let v: Vec<&str> = data.split("").collect();
            let v_int: i32 = v[b].parse().unwrap();
            if v_int == 0 {
                _zero += 1;
            }
            if v_int == 1 {
                _one += 1;
            }
        }
        if _one > _zero {
            filter(o2_values, "1", 0);
        } else {
            filter(o2_values, "0", 0);
        }
    }
}

fn filter(values: Vec<String>, x: &str, position: usize) {
    let mut reduced_array: Vec<&String> = Vec::new();
    for (index, data) in values.iter().enumerate() {
        if data.chars().nth(position).unwrap() == x.chars().nth(0).unwrap() {
            reduced_array.push(data);
        }
    }
    for value in reduced_array {
        println!("{}", value);
    }
}

fn main() {
    part_one();
    part_two();
}
