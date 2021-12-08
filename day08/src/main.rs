// struct seven_segment_display {
//     timer: i32
// }

use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file_reader(filename: &str) -> BufReader<File> {
    let data = File::open(filename).unwrap();
    return std::io::BufReader::new(data);

}


fn part_one() {

    // Iterate over measurements file line by line
    let reader = get_file_reader("input.txt");
    let mut matching_digits = 0;
    for (index, line) in reader.lines().enumerate() {
        let data = line.unwrap();
        let v: Vec<&str> = data.split(" | ").collect();
        let output = v[1];
        //println!("{}", output);
        let v: Vec<&str> = output.split(" ").collect();
        for x in 0..v.len() {
            let length = v[x].len();
            //println!("{} L{}", v[x], length);
            if length == 2 || length == 3 || length == 4 || length == 7 {
                matching_digits += 1;
            }

        }
        //println!();
    }
    println!("{} = Number of times a 1, 4, 7 or 8 appears in the output", matching_digits);

    // d0 = 6
    // d1 = 2  unique  c,f
    // d2 = 5
    // d3 = 5
    // d4 = 4  unique  b,c,d,f
    // d5 = 5
    // d6 = 6
    // d7 = 3  unique  a,c,f
    // d8 = 7  unique  a,b,c,d,e,f
    // d9 = 6

    println!("PART ONE");
    println!("=======================================");
}

fn part_two() {
    println!("PART TWO");
    println!("=======================================");
}

fn main() {
    part_one();
    println!();
    part_two();
}
