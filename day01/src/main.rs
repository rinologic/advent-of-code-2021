use std::fs::File;
use std::io::{BufRead, BufReader};

// Advent of code Day One Puzzle
// Also, my first RUST program ever :)
// I made it very verbose on purpose

fn main() {

    // file containing the measurements
    let measurements_file_name = "measurements.txt";

    // initialize some variables
    let mut _increases: i32 = 0;
    let mut _decreases: i32 = 0;
    let mut last: i32 = 0;

    // open measurements file and create a buffered reader
    let measurements = File::open(measurements_file_name).unwrap();
    let reader = BufReader::new(measurements);

    println!("===============================================================");

    // Iterate over measurements file line by line
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

