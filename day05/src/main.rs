use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::fmt;
use std::iter::Map;

struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    //lx: i32,                   // the lower x value
    //hx: i32,                   // the higher x value
    //ly: i32,                   // the lower y value
    //hy: i32,                   // the higher y value
    is_horizontal: bool,
    is_vertical: bool
}

struct MapPoint {
    x: i32,
    y: i32,
    density: i32,
}

impl MapPoint {
    fn new() -> Self {
        MapPoint {
                  x: 0,
                  y: 0,
            density: 0        // will track how dangerous this point is
        }
    }
}

struct OceanFloor {
         min_x: i32,
         max_x: i32,
         min_y: i32,
         max_y: i32,
        x_span: i32,           // the total distance from min x to max x
        y_span: i32,           // the total distance from min y to max y
    map_points: Vec<MapPoint>
}

impl OceanFloor {
    fn new() -> Self {
        OceanFloor {
             min_x: 0,
             max_x: 0,
             min_y: 0,
             max_y: 0,
            x_span: 0,
            y_span: 0,
            map_points: Vec::new()
        }
    }
}

impl fmt::Display for OceanFloor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MIN X = {} | MIN Y = {} | MAX X = {} | MAX Y = {} | X SPAN = {} | Y SPAN = {} | Map Points = {}",
               self.min_x, self.min_y, self.max_x, self.max_y, self.x_span, self.y_span, self.map_points.len())
    }
}


fn load_input(filename: &str) -> BufReader<File> {
    let measurements = File::open(filename).unwrap();
    return BufReader::new(measurements);
}

// Given a pair of coordinates in this format: 0,9  5,9
// Return a coordinate object
fn parse_lines(coord_to_parse: &str) -> Line {
    let re = Regex::new(r"^([0-9]+),([0-9]+)\s+->\s+([0-9]+),([0-9]+)$").unwrap();
    let cap = re.captures(coord_to_parse).unwrap();
    let x1 = (&cap[1]).parse().unwrap();
    let x2 = (&cap[3]).parse().unwrap();
    let y1 = (&cap[2]).parse().unwrap();
    let y2 = (&cap[4]).parse().unwrap();
    Line {
        x1, x2, y1, y2,
        is_horizontal: (y1 == y2),
        is_vertical: (x1 == x2),
    }
}

fn main() {
    let reader = load_input("input.txt");
    let mut lines: Vec<Line> = vec![];
    let mut lines_filtered: Vec<Line> = vec![];
    for (_index, data_line) in reader.lines().enumerate() {
        lines.push(parse_lines(&data_line.unwrap()));
    }
    println!(" LOADED {} Lines", lines.len());

    // for now, only consider horizontal and vertical lines
    for l in lines {
        if l.is_horizontal || l.is_vertical {
            lines_filtered.push(l);
        }
    }
    println!("KEEPING {} LINES", lines_filtered.len());

    // Next I think we need to make a huge grid and initialize all the values to zero

    // Determine min x and min y so we can make some map points
    // минималды - language practice while i work :D
    let min_x1 = lines_filtered.iter().min_by_key(|c| c.x1).unwrap().x1;
    let min_x2 = lines_filtered.iter().min_by_key(|c| c.x2).unwrap().x2;
    let min_y1 = lines_filtered.iter().min_by_key(|c| c.y1).unwrap().y1;
    let min_y2 = lines_filtered.iter().min_by_key(|c| c.y2).unwrap().y2;
    // максималды
    let max_x1 = lines_filtered.iter().max_by_key(|c| c.x1).unwrap().x1;
    let max_x2 = lines_filtered.iter().max_by_key(|c| c.x2).unwrap().x2;
    let max_y1 = lines_filtered.iter().max_by_key(|c| c.y1).unwrap().y1;
    let max_y2 = lines_filtered.iter().max_by_key(|c| c.y2).unwrap().y2;

    //println!("MIN: X1={} Y1={} | X2={} Y2={}", min_x1, min_y1, min_x2, min_y2);
    //println!("MAX: X1={} Y1={} | X2={} Y2={}", max_x1, max_y1, max_x2, max_y2);

    // Now we create an Ocean Floor to represent our data
    let mut ocean_floor: OceanFloor = OceanFloor::new();
    ocean_floor.max_x = if max_x1 > max_x2 { max_x1 } else { max_x2 };
    ocean_floor.min_x = if min_x1 < min_x2 { min_x1 } else { min_x2 };
    ocean_floor.max_y = if max_y1 > max_y2 { max_y1 } else { max_y2 };
    ocean_floor.min_y = if min_y1 < min_y2 { min_y1 } else { min_y2 };
    ocean_floor.x_span = ocean_floor.max_x - ocean_floor.min_x;
    ocean_floor.y_span = ocean_floor.max_y - ocean_floor.min_y;

    // Now we populate it with MapPoints, one for each x and y coordinate
    println!();
    println!("OCEAN FLOOR DATA");

    // Now we will populate the Ocean Floor with one map point for each x and y coordinate
    for x_point in ocean_floor.min_x..ocean_floor.max_x + 1 {
        for y_point in ocean_floor.min_y..ocean_floor.max_y + 1 {
            let mp = MapPoint { x: x_point, y: y_point, density: 0 };
            ocean_floor.map_points.push(mp);
        }
    }
    println!("{}", ocean_floor.to_string());

    // Register the line data
    println!("REGISTERING MAP DATA");
    let mut iterations = 0;
    for l in lines_filtered.iter() {

        // register vertical hits
        if l.is_vertical {
            let mut start_y = 0;
            let mut end_y = 0;
            if l.y1 < l.y2 { start_y = l.y1 } else { start_y = l.y2 }
            if l.y1 > l.y2 { end_y = l.y1 } else { end_y = l.y2 }
            //print!("V: {},{} - {},{} |", l.x1, start_y, l.x2, end_y);
            for y_point in start_y..end_y + 1 {
                //print!("{},{} ", l.x1, y_point);
                set_point_value_density(&mut ocean_floor, l.x1, y_point, 1);
            }
            //println!();
        }

        // register horizontal hits
        if l.is_horizontal {
            let mut start_x = 0;
            let mut end_x = 0;
            if l.x1 < l.x2 { start_x = l.x1 } else { start_x = l.x2 }
            if l.x1 > l.x2 { end_x = l.x1 } else { end_x = l.x2 }
            //print!("H: {},{} - {},{} |", start_x, l.y1, end_x, l.y2);
            for x_point in start_x..end_x + 1 {
                //print!("{},{} ", x_point, l.y1)
                set_point_value_density(&mut ocean_floor, x_point, l.y1, 1);
            }
            //println!();
        }

        iterations += 1;
        println!("{}/{} iterations complete", iterations, lines_filtered.len());
    }


    // Print out the ocean floor and count danger zones! 
    // waaaaay to slow with my super inefficient map point getter/setter on big maps
    println!("Calculating danger zones");
    //let mut danger_zones = 0;
    //for x_point in ocean_floor.min_x..ocean_floor.max_x+1 {
    //  for y_point in ocean_floor.min_y..ocean_floor.max_y+1 {
    //    let map_value: i32 = get_map_point_density(&ocean_floor, x_point, y_point);
    //    if map_value == 0 {
    //print!("{:width$}", ".", width = 2)
    //       } else {
    //print!("{:width$}", map_value , width = 2)
    //      }
    //     if map_value >= 2 {
    //        danger_zones += 1;
    //   }
    //}
    //println!();
    //}
    println!("DANGER ZONES: {}", get_danger_zones(&ocean_floor));
}


fn get_danger_zones(ocean_floor: &OceanFloor) -> i32 {
    let mut danger_zones: i32 = 0;
    for mp in ocean_floor.map_points.iter() {
        if mp.density >= 2 {
            danger_zones += 1;
        }
    }
    return danger_zones;
}


fn get_map_point_density(ocean_floor: &OceanFloor, x_point: i32, y_point:i32) -> i32 {
    let mut density:i32 = 0;
    for mp in ocean_floor.map_points.iter() {
        if mp.x == x_point && mp.y == y_point {
            density = mp.density;
        }
    }
    return density;
}
fn set_point_value_density(ocean_floor: &mut OceanFloor, x_point: i32, y_point:i32, value:i32) {
    for mp in ocean_floor.map_points.iter_mut() {
        if mp.x == x_point && mp.y == y_point {
            mp.density = mp.density + value;
        }
    }
}
