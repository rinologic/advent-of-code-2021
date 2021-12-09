use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file_reader(filename: &str) -> BufReader<File> {
    let data = File::open(filename).unwrap();
    return std::io::BufReader::new(data);
}

fn part_one() {
    println!("PART ONE");
    println!("=====================================================================================");

    // Iterate over measurements file line by line
    let reader = get_file_reader("input.txt");
    let mut matching_digits = 0;
    for (_index, line) in reader.lines().enumerate() {
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

    // d0 = 6          a,b,c,e,f,g    16
    // d1 = 2  unique  c,f            4
    // d2 = 5          a,c,d,e,g      16
    // d3 = 5          a,c,d,f,g      16
    // d4 = 4  unique  b,c,d,f        10
    // d5 = 5          a,b,d,f,g      16
    // d6 = 6          a,b,d,e,f,g    18
    // d7 = 3  unique  a,c,f          8
    // d8 = 7  unique  a,b,c,d,e,f    20
    // d9 = 6          a,b,c,d,f,g    18
}

#[derive(Clone, Debug)]
struct SevenSegmentDisplay {
    pos0: char,
    pos1: char,
    pos2: char,
    pos3: char,
    pos4: char,
    pos5: char,
    pos6: char,
    num_chars: i8,
    known_digit: bool,
    digit: i8,
    scrambled: String
}

// impl SevenSegmentDisplay {
//     fn new() -> Self {
//         SevenSegmentDisplay {
//             pos0: 'n',
//             pos1: 'n',
//             pos2: 'n',
//             pos3: 'n',
//             pos4: 'n',
//             pos5: 'n',
//             pos6: 'n',
//             num_chars: 0,
//             known_digit: false,
//             digit: 0,
//             scrambled: "".to_string()
//         }
//     }
// }

impl fmt::Display for SevenSegmentDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {}, {}, {}, {}, {}, {}, {} ] ", self.pos0, self.pos1, self.pos2, self.pos3, self.pos4, self.pos5, self.pos6)
        //write!(f, "NUM CHARS: {} | ", self.num_chars)
        //print!("{:width$} | ", self.scrambled, width = 7)
        //write!(f, "KNOWN DIGIT: {} | ", self.known_digit)
        //write!(f, "{}", self.digit)
    }
}

fn part_two() {
     println!("PART TWO");
     println!("=====================================================================================");
     println!("Please see my Python solution for part two.");
}
//
//     // Highest letter possible is g
//     // a b c d e f g
//
//     // Total segments are seven (seven segment display)
//
//     // Need a "decoder struct" that maps the seven segment
//     // display's positions to the scrambled values
//     // values.  So for example, for non-scrambled display
//     // it would look like this:
//
//     // decoder and 0-9 values, under normal circumstances look as follows, last part
//     // of data shows how many segments are populated and whether or not it is unique
//
//     // decoder: { [0]['a'], [1]['b'], [2]['c'], [3]['d'], [4]['e'], [5]['f'], [6]['g'] }
//
//     //    zero: { [0]['a'], [1]['b'], [2]['c'], [3]['_'], [4]['e'], [5]['f'], [6]['g'] }   6
//     //     one: { [0]['_'], [1]['_'], [2]['c'], [3]['_'], [4]['_'], [5]['f'], [6]['_'] }   2u
//     //     two: { [0]['a'], [1]['_'], [2]['c'], [3]['d'], [4]['e'], [5]['_'], [6]['g'] }   5
//     //   three: { [0]['a'], [1]['_'], [2]['c'], [3]['d'], [4]['_'], [5]['f'], [6]['g'] }   5
//     //    four: { [0]['_'], [1]['b'], [2]['c'], [3]['d'], [4]['_'], [5]['f'], [6]['_'] }   4u
//     //    five: { [0]['a'], [1]['b'], [2]['_'], [3]['d'], [4]['_'], [5]['f'], [6]['g'] }   5
//     //     six: { [0]['a'], [1]['b'], [2]['_'], [3]['d'], [4]['_'], [5]['f'], [6]['g'] }   6
//     //   seven: { [0]['a'], [1]['_'], [2]['c'], [3]['_'], [4]['_'], [5]['f'], [6]['_'] }   3u
//     //   eight: { [0]['a'], [1]['b'], [2]['c'], [3]['d'], [4]['e'], [5]['f'], [6]['g'] }   7u
//     //    nine: { [0]['a'], [1]['b'], [2]['c'], [3]['d'], [4]['_'], [5]['f'], [6]['g'] }   6
//
//     // okay, so now we start out with an empty decoder and use pairs of unique, similarly
//     // scrambled values to slowly populate our decoder
//     // entries to slowly populate the decoder, then each time we identify one, we must
//     // update the
//
//
//
//     let reader = get_file_reader("input_sample.txt");
//     for (_index, line) in reader.lines().enumerate() {
//
//         // First, break the whole line into two pieces
//         let dataline = line.unwrap();
//         let v: Vec<&str> = dataline.split(" | ").collect();
//         let signal_wire_entry_data = v[0];
//         let encoded_values = v[1];
//
//         // Now break up the signals into individual units and create seven_segment_displays
//         let signal_wire_entries: Vec<&str> = signal_wire_entry_data.split(" ").collect();
//         let mut ssds:Vec<SevenSegmentDisplay> = vec![];
//
//         for index in 0..signal_wire_entries.len() {
//             let entry_length = signal_wire_entries[index].len();
//             let scrambled_string: String = signal_wire_entries[index].parse().unwrap();
//             let mut scrambled_chars: Vec<_> = scrambled_string.chars().collect();
//             let sc_slice: &str = &scrambled_string[..];
//             let mut sc_chars: Vec<char> = sc_slice.chars().collect();
//             sc_chars.sort_by(|a,b| a.cmp(b));
//             let mut ssd = SevenSegmentDisplay {
//                 pos0: '0',
//                 pos1: '0',
//                 pos2: '0',
//                 pos3: '0',
//                 pos4: '0',
//                 pos5: '0',
//                 pos6: '0',
//                 num_chars: entry_length as i8,
//                 known_digit: entry_length == 2 || entry_length == 3 || entry_length == 4 || entry_length == 7,
//                 digit: -1,
//                 scrambled: scrambled_string
//             };
//             if ssd.known_digit {     // if it is a unique one, we can populate its positions with scrambled values
//                 if ssd.num_chars == 2 {
//                     ssd.digit = 1;
//                     ssd.pos2 = scrambled_chars[0];
//                     ssd.pos5 = scrambled_chars[1];
//                 }
//                 if ssd.num_chars == 4 {
//                     ssd.digit = 4;
//                     ssd.pos1 = scrambled_chars[0];
//                     ssd.pos2 = scrambled_chars[1];
//                     ssd.pos3 = scrambled_chars[2];
//                     ssd.pos5 = scrambled_chars[3];
//                 }
//                 if ssd.num_chars == 3 {
//                     ssd.digit = 7;
//                     ssd.pos0 = scrambled_chars[0];
//                     ssd.pos2 = scrambled_chars[1];
//                     ssd.pos5 = scrambled_chars[2];
//                 }
//                 if ssd.num_chars == 7 {
//                     ssd.digit = 8;
//                     ssd.pos0 = scrambled_chars[0];
//                     ssd.pos1 = scrambled_chars[1];
//                     ssd.pos2 = scrambled_chars[2];
//                     ssd.pos3 = scrambled_chars[3];
//                     ssd.pos4 = scrambled_chars[4];
//                     ssd.pos5 = scrambled_chars[5];
//                     ssd.pos6 = scrambled_chars[6];
//                 }
//             }
//             ssds.push(ssd);
//         }
//         create_decoder(ssds);
//         //if index == 0 {
//
//             //break  // only look at first one initially
//         //}
//     }
// }



// fn create_decoder(ssds: Vec<SevenSegmentDisplay>) {
//
//     let mut fives:Vec<SevenSegmentDisplay> = vec![];
//     let mut sixes:Vec<SevenSegmentDisplay> = vec![];
//     let mut ssd_one = SevenSegmentDisplay::new();
//     let mut ssd_three = SevenSegmentDisplay::new();
//     let mut ssd_four = SevenSegmentDisplay::new();
//     let mut ssd_six = SevenSegmentDisplay::new();
//     let mut ssd_seven = SevenSegmentDisplay::new();
//     let mut ssd_eight = SevenSegmentDisplay::new();
//
//     for ssd in ssds {
//         if ssd.num_chars == 2 {
//             ssd_one = ssd.clone();
//         }
//         if ssd.num_chars == 4 {
//             ssd_four = ssd.clone();
//         }
//         if ssd.num_chars == 3 {
//             ssd_seven = ssd.clone();
//         }
//         if ssd.num_chars == 7 {
//             ssd_eight = ssd.clone();
//         }
//         // get the fives
//         if ssd.num_chars == 5 {
//             fives.push(ssd.clone());
//         }
//         // get the sixes
//         if ssd.num_chars == 6 {
//             sixes.push(ssd.clone());
//         }
//     }
//
//     // whichever five has all of the digits from one is the correct one
//     for ssd in fives {
//         let ssd_one_chars = ssd_one.scrambled.chars();
//         let mut is_ssd_three = true;
//         for ssd_one_char in ssd_one_chars {
//             if !ssd.scrambled.contains(ssd_one_char) {
//                 is_ssd_three = false;
//             }
//         }
//         if is_ssd_three {
//             ssd_three = ssd.clone();
//             ssd_three.known_digit = true;
//             ssd_three.digit = 3;
//         }
//     }
//     println!("{}", ssd_one);
//     println!("{}", ssd_three);
//     println!("{}", ssd_four);
//     println!("{}", ssd_seven);
//     println!("{}", ssd_eight);
//     println!("====================================");
// }

fn main() {
    part_one();
    println!();
    part_two();
}
