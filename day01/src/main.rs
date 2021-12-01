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

    println!("===================== PART 1 ============================");

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

    println!("TOTALS \nINCREASES = {}\nDECREASES = {}", _increases, _decreases);
    println!("===============================================================");

    // print the results

}

fn part_two() {

    println!("========================= PART 2 ===============================");

    let reader = get_file_reader("measurements.txt");

    // I'm sure there is a better way to do this
    // Iterate through measures and load them into an array of int 32
    let mut measurements: Vec<i32> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        // convert string to an int
        let measurement = line.unwrap();
        let measurement_int: i32 = measurement.parse().unwrap_or(0);
        measurements.push(measurement_int);
    }

    let vector_size = measurements.len() as i32;
    println!("The vector size is {}", vector_size);
    let window_size: i32 = 3;
    let mut _increases: i32 = 0;
    let mut _decreases: i32 = 0;
    let mut _no_change: i32 = 0;
    for (index, measure) in measurements.iter().enumerate() {
        let index_32 = index as i32;
        if (index_32 < (vector_size - window_size)) {
            let mut sum_a = measurements[index] + measurements[index+1] + measurements[index+2];
            let mut sum_b = measurements[index+1] + measurements[index+2] + measurements[index+3];
            print!("{} vs {} = ", sum_a, sum_b);
            if sum_b > sum_a {
                _increases += 1;
                println!("INCREASE");
            }
            if sum_b < sum_a {
                _decreases += 1;
                println!("DECREASE");
            }
            if sum_b == sum_a {
                _no_change += 1;
                println!("NO CHANGE");
            }
        }
    }
    println!("TOTALS \nINCREASES = {}\nDECREASES = {}\nNO CHANGE = {}", _increases, _decreases, _no_change);
}

fn main() {
    part_one();
    part_two();
}

