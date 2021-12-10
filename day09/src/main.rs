use std::io::{self, Read};

fn part_one() {

    // Load the data directly into a vector of vectors of type i32.
    // We map the lines to a string, then map the string chars
    // in 2nd mapping
    let heightmap: Vec<Vec<i32>> = load_data()
        .lines()
        .map(|line| {
        line.chars()
            .map(|c | c.to_digit(10).unwrap() as i32)
            .collect()
    }).collect();
    //println!("{:?}", heightmap);
    
    let height = heightmap.len();
    let width = heightmap[0].len();
    //println!("W={} H={}", width, height);

    // hold low point for later summing up
    let mut low_points: Vec<i32> = Vec::new();

    for row in 0..height {
        for col in 0..width {
            let map_value = heightmap[row][col];

            // look at row above current row
            if row > 0 && map_value >= heightmap[row-1][col] {
                continue;
            }
            // look at row below current row
            if row < height-1 && map_value >= heightmap[row+1][col] {
                continue;
            }
            // look at column to the left
            if col > 0 && map_value >= heightmap[row][col-1] {
                continue
            }
            // look at column to the right
            if col < width-1 && map_value >= heightmap[row][col+1] {
                continue
            }
            low_points.push(map_value);
        }
    }
    let mut total = 0;
    for low_point in low_points.iter() {
        total += low_point+1;
        //print!("{}|", low_point);
    }
    println!("\nTOTAL: {}", total);
}

fn main() {
    part_one();
}

fn load_data() -> String {
    let mut data = String::new();
    io::stdin().lock().read_to_string(&mut data).unwrap();
    return data.trim().to_string();
}