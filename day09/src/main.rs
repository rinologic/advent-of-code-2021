use std::{
    collections::{HashSet, VecDeque},
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BasinPoint {
    row: usize,
    col: usize,
}

fn main() {

    let heightmap: Vec<Vec<i64>> = get_input()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    let width = heightmap.len();
    let height = heightmap[0].len();

    let mut low_points: Vec<BasinPoint> = Vec::new();
    for row in 0..width {
        for col in 0..height {

            let point_height = heightmap[row][col];

            // check row above current row
            if row > 0 && point_height >= heightmap[row - 1][col] {
                continue;
            }

            // check row below current row
            if row < width - 1 && point_height >= heightmap[row + 1][col] {
                continue;
            }

            // check column to the left
            if col > 0 && point_height >= heightmap[row][col - 1] {
                continue;
            }

            // check column to the right
            if col < height - 1 && point_height >= heightmap[row][col + 1] {
                continue;
            }

            low_points.push(BasinPoint { row: row, col });
        }
    }
    let mut totals = 0;
    for low_point in low_points.iter() {
        totals += 1 + (heightmap[low_point.row][low_point.col]);
    }
    println!("PART ONE: {}",totals);

    // On to part two
    let mut basin_sizes: Vec<usize> = Vec::new();
    for point in low_points.iter() {
        let mut basin_points: HashSet<BasinPoint> = HashSet::new();
        let mut queue: VecDeque<BasinPoint> = VecDeque::new();
        queue.push_back(*point);
        basin_points.insert(*point);

        while queue.len() > 0 {

            let cur = queue.pop_front().unwrap();

            let row = cur.row;
            let col = cur.col;

            let mut neighbors: Vec<BasinPoint> = Vec::new();

            // check row above current row
            if row > 0 {
                neighbors.push(BasinPoint { row: row - 1, col })
            }

            // check row below current row
            if row < width - 1 {
                neighbors.push(BasinPoint { row: row + 1, col })
            }

            // check column to the right
            if col > 0 {
                neighbors.push(BasinPoint { row: row, col: col - 1 })
            }

            // check column to the right
            if col < height - 1 {
                neighbors.push(BasinPoint { row: row, col: col + 1 })
            }

            for n in neighbors {
                if basin_points.contains(&n) || heightmap[n.row][n.col] == 9 {
                    continue;
                }
                queue.push_back(n);
                basin_points.insert(n);
            }
        }

        basin_sizes.push(basin_points.len());
    }

    //println!("{:?}", basin_sizes);
    basin_sizes.sort();
    //println!("{:?}", basin_sizes);
    basin_sizes.reverse();
    //println!("{:?}", basin_sizes);
    println!("PART TWO: {}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);

}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
