use std::fs::File;
use std::io::{BufRead, BufReader};

// So I had this all in the main function for part one of the puzzle but
// now I am exploring functions with RUST, passing params, return types,
// etc. I moved the file reader to its own function since it is needed
// in part one and two.  Also created a function for each puzzle part
// and calling them from main.

// Pass in a file name and returns a file buffer reader.  This assumes
// the file is in the root project directory.
fn get_file_reader(filename: &str) -> BufReader<File> {
    let measurements = File::open(filename).unwrap();
    return BufReader::new(measurements);
}

fn part_one() {
    // initialize some variables
    let mut _increases: i32 = 0;
    let mut _decreases: i32 = 0;
    let mut last: i32 = 0;

    println!("===============================================================");

    // Iterate over measurements file line by line
    let reader = get_file_reader("measurements.txt");
    for (index, line) in reader.lines().enumerate() {

        // convert string to an int
        let measurement = line.unwrap();
        let measurement_int: i32 = measurement.parse().unwrap_or(0);

        // If we are on the first reading, set the "last" to it and continue
        if index == 0 {
            last = measurement_int;
            println!("LAST = {}", measurement_int);
            continue;
        }

        // Determine if current reading is greater or less than the last one and tally it
        if measurement_int > last {
            _increases += 1;
            println!("{} > {}, INCREASES = {}", measurement_int, last, _increases);
        }
        if measurement_int < last  {
            _decreases += 1;
            println!("{} < {}, DECREASES = {}", measurement_int, last, _decreases);
        }

        // Set the last reading to the current one and keep going!
        last = measurement_int;
        println!("LAST = {}", last);
    }

    println!("===============================================================");

    // print the results
    print!("TOTALS \nINCREASES = {}\nDECREASES = {}", _increases, _decreases);
}

fn part_two() {
    let reader = get_file_reader("measurements.txt");
    for (index, line) in reader.lines().enumerate() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    part_one();
    part_two();
}

