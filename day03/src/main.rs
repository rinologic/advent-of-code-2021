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
    let reader = get_file_reader("sample_data.txt");
    let mut all_values: Vec<String> = Vec::new();
    for (_index, _line) in reader.lines().enumerate() {
        all_values.push(_line.unwrap());
    }
    let mut new_array = Vec::new();
    for x in 0..5 {
        let value = get_high_count_by_column(&all_values, x+1);
        println!("{} = {}", x, value);
        new_array = filter(&all_values, value, x);
    }
    for (index, data) in new_array.iter().enumerate() {
        println!("{}", data);
    }
}


fn get_high_count_by_column(values: &Vec<String>, column: usize) -> &'static str {
    let mut _zero: i32 = 0;
    let mut _one: i32 = 0;
    for (index, data) in values.iter().enumerate() {
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
    } else {
        return "0";
    }
}

fn filter(values: &Vec<String>, x: &str, position: usize) -> Vec<String> {
    let mut reduced_array: Vec<String> = Vec::new();
    for (index, data) in values.iter().enumerate() {
        if data.chars().nth(position).unwrap() == x.chars().nth(0).unwrap() {
            reduced_array.push(data.clone());
        }
    }
    return reduced_array;
}

fn main() {
    part_one();
    part_two();
}
