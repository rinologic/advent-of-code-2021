use std::io::{self, Read};

fn main() {
    let mut total: i32 = 0;
    // mark the for statement so we can control where continue statement continues
    '_line: for line in load_data().lines() {
        let mut stack: Vec<char> = Vec::new();  // stack to hold chunk openers
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c), // push to stack if chunk opener
                ')' | ']' | '}' | '>' => {
                    // if chunk closer, append to score if illegal (non match)
                    if stack.len() == 0 || c != match_closer(*stack.last().unwrap()) {
                        total += append_score(c);
                        continue '_line;  // go to next line
                    }
                    stack.pop(); // keep popping the stack until we find matching closer
                }
                _ => panic!("unexpected symbol {}", c),  // should not happen with clean data
            }
        }
    }
    println!("{}", total);
}

fn match_closer(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unexpected symbol {}", c),
    }
}

fn append_score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected symbol {}", c),
    }
}

fn load_data() -> String {
    let mut data = String::new();
    io::stdin().lock().read_to_string(&mut data).unwrap();
    return data.trim().to_string();
}