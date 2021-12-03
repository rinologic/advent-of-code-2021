use std::fs::File;
use std::io::{BufRead, BufReader};

// The part two code is super sloppy and inefficient.
// I was just glad to get it working.  Obviously I can
// refactor the get high/low count functions into one
// but I just wanted to get it done as soon as possible.
// Also I can refactor the o2 co2 functions. I will
// continue to work on optimizing it.  Also, I know
// that the way I am doing things in Rust is probably
// not very "Rust-like" but so far what you see below
// is the only way I have figured it out.  Lot's
// more reading to do!

fn get_file_reader(filename: &str) -> BufReader<File> {
    let datafile = File::open(filename).unwrap();
    return BufReader::new(datafile);
}

fn part_one() {
    println!("===================================");
    println!("Part One");
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

fn part_two_o2() {
    println!("===================================");
    println!("Part Two - O2");
    let reader = get_file_reader("data.txt");
    let mut all_values: Vec<String> = Vec::new();
    for (_index, _line) in reader.lines().enumerate() {
        all_values.push(_line.unwrap());
    }
    let one_or_zero = get_high_count_by_column(&all_values, 1);
    let mut new_array = filter(&all_values, one_or_zero, 0);
    for x in 2..12 {
        if new_array.len() > 1 {
            let one_or_zero = get_high_count_by_column(&new_array, x);
            new_array = filter(&new_array, one_or_zero, x - 1);
        }
    }
    for (_index, data) in new_array.iter().enumerate() {
        if data.ends_with("1") { // total hack but filters out non 1 values in last column
            let os_dec = isize::from_str_radix(&data, 2).unwrap();
            println!("{} = {}", data, os_dec);
        }
    }
}

fn part_two_co2() {
    println!("===================================");
    println!("Part Two - CO2");
    let reader = get_file_reader("data.txt");
    let mut all_values: Vec<String> = Vec::new();
    for (_index, _line) in reader.lines().enumerate() {
        all_values.push(_line.unwrap());
    }
    let one_or_zero = get_low_count_by_column(&all_values, 1);
    let mut new_array = filter(&all_values, one_or_zero, 0);
    for x in 2..12 {
        if new_array.len() > 1 {
            let one_or_zero = get_low_count_by_column(&new_array, x);
            new_array = filter(&new_array, one_or_zero, x - 1);
        }
    }
    for (_index, data) in new_array.iter().enumerate() {
        if data.ends_with("0") {
            let os_dec = isize::from_str_radix(&data, 2).unwrap();
            println!("{} = {}", data, os_dec);
        }
    }
}


fn get_high_count_by_column(values: &Vec<String>, column: usize) -> &'static str {
    let mut _zero: i32 = 0;
    let mut _one: i32 = 0;
    for (_index, data) in values.iter().enumerate() {
        let v: Vec<&str> = data.split("").collect();
        let v_int: i32 = v[column].parse().unwrap();
        if v_int == 0 {
            _zero += 1;
        }
        if v_int == 1 {
            _one += 1;
        }
    }
    if _one > _zero {
        return "1";
    }
    if _one == _zero {
        return "1";
    }
    return "0";
}

fn get_low_count_by_column(values: &Vec<String>, column: usize) -> &'static str {
    let mut _zero: i32 = 0;
    let mut _one: i32 = 0;
    for (_index, data) in values.iter().enumerate() {
        let v: Vec<&str> = data.split("").collect();
        let v_int: i32 = v[column].parse().unwrap();
        if v_int == 0 {
            _zero += 1;
        }
        if v_int == 1 {
            _one += 1;
        }
    }
    if _one < _zero {
        return "1";
    }
    if _one == _zero {
        return "0";
    }
    return "0";
}

fn filter(values: &Vec<String>, x: &str, position: usize) -> Vec<String> {
    let mut reduced_array: Vec<String> = Vec::new();
    for (_index, data) in values.iter().enumerate() {
        if data.chars().nth(position).unwrap() == x.chars().nth(0).unwrap() {
            reduced_array.push(data.clone());
        }
    }
    return reduced_array;
}

fn main() {
    part_one();
    part_two_o2();
    part_two_co2();
}
