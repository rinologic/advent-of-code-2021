use std::fs::{File};
use std::io::{BufRead, BufReader};

// We are going to load up two boards, one will be the game boards with all the values
// the second will be boards used to record matches.  That board will be the exact same as
// the game board but we will replace a match with a -1.  To get the final total, we
// can use the corresponding game board as a reference.

// Note that the reason I used two boards initially was because I thought I would have
// to add up the already called numbers.  I realize now I could have used the same board
// to make the calculations.  Was still nice to have two boards as a reference when
// the winning board was detected.

fn get_file_reader(filename: &str) -> BufReader<File> {
        let datafile = File::open(filename).unwrap();
        return BufReader::new(datafile);
}

fn load_boards(game_boards: &mut Vec<Vec<Vec<i32>>>, match_boards: &mut Vec<Vec<Vec<i32>>>,
               board_width: usize, board_height: usize) {

    let board_data_file = get_file_reader("bingo_boards.txt");
    let mut board_data: Vec<String> = Vec::new();
    for (_index, line) in board_data_file.lines().enumerate() {
        board_data.push(line.unwrap());
    }

    let mut row = 0;
    let mut line_number = 0;

    // Vector two hold all of the game boards
    let mut game_board = vec![vec![0; board_width]; board_height];
    //let mut game_board_matches = vec![vec![0; board_width]; board_height];
    for (_index, _line) in board_data.iter().enumerate() {
        line_number += 1;
        if line_number == 1 || line_number % 5 == 0 {
            let _game_board = vec![vec![0; board_width]; board_height];
        }
        if _line == "" {
            row = 0;
        } else {
            let v: Vec<&str> = _line.split_whitespace().collect();
            for _n in 0..4 {
                game_board[row][0] = v[0].parse().unwrap();
                game_board[row][1] = v[1].parse().unwrap();
                game_board[row][2] = v[2].parse().unwrap();
                game_board[row][3] = v[3].parse().unwrap();
                game_board[row][4] = v[4].parse().unwrap();
            }
            row = row + 1;
            if row == 5 {
                game_boards.push(game_board.clone());
                match_boards.push(game_board.clone());
            }
        }
    }
    // Not sure yet why the first game board is empty
    game_boards.remove(0);
    match_boards.remove(0);
}

fn announce_winner(board_number: usize, gb: Vec<Vec<Vec<i32>>>, mb: Vec<Vec<Vec<i32>>>, bingo_number: i32)  {
    println!("=======================================================================");
    println!("BOARD {} IS A WINNER WITH THE CALL OF {}!", board_number, bingo_number);
    let game_board = gb.get(board_number);
    let match_board = mb.get(board_number);
    println!("{:?}", game_board);
    println!("{:?}", match_board);
    // Now we need to add up all the values on the game board that are not -1
    let mut unmarked_sum: i32 = 0;
    for i in match_board.iter() {
        for r in 0..5 {
            for c in 0..5 {
                if i[r][c] != -1 && i[r][c] != bingo_number {
                    unmarked_sum += i[r][c];
                    print!("{} ", i[r][c])
                }
            }
        }
    }
    println!("\nREDEMPTION CODE IS {} * {} = {}", unmarked_sum, bingo_number, unmarked_sum * bingo_number);
}

fn part_one() {
    println!("PART ONE");
    // Initialize our Vector of game boards
    let board_width = 5;
    let board_height = 5;
    let mut game_boards = vec![vec![vec![]]];
    let mut match_boards = vec![vec![vec![]]];

    // Load the boards up with the board data
    load_boards(&mut game_boards, &mut match_boards, board_width, board_height);

    let bingo_numbers = [ 84,28,29,75,58,71,26,6,73,74,41,39,87,37,16,79,55,60,62,80,64,95,46,15,5,47,2,35,32,78,89,90,96,33,4,69,42,30,54,85,65,83,44,63,20,17,66,81,67,77,36,68,82,93,10,25,9,34,24,72,91,88,11,38,3,45,14,56,22,61,97,27,12,48,18,1,31,98,86,19,99,92,8,43,52,23,21,0,7,50,57,70,49,13,51,40,76,94,53,59 ];

    for bingo_number in bingo_numbers {
        for b in 0..match_boards.len() {
            mark_match(match_boards.get_mut(b).unwrap(), board_width, board_height, bingo_number);

            // Now, check the board for a win
            if is_row_win(match_boards.get(b).unwrap()) ||
                is_column_win(match_boards.get(b).unwrap()) {
                announce_winner(b, game_boards.clone(), match_boards.clone(), bingo_number);
                return;
            }
        }
    } // next bingo number
}

fn is_row_win(match_board: &Vec<Vec<i32>>) -> bool {
    let mut winner = false;
    for i in 0..5 {
        if  match_board[0][i] == -1 &&
            match_board[1][i] == -1 &&
            match_board[2][i] == -1 &&
            match_board[3][i] == -1 &&
            match_board[4][i] == -1 {
            winner = true;
        }
    }
    return winner;
}

fn is_column_win(match_board: &Vec<Vec<i32>>) -> bool {
    let mut winner = false;
    for i in 0..5 {
        if  match_board[i][0] == -1 &&
            match_board[i][1] == -1 &&
            match_board[i][2] == -1 &&
            match_board[i][3] == -1 &&
            match_board[i][4] == -1 {
            winner = true;
        }
    }
    return winner;
}

fn mark_match(match_board: &mut Vec<Vec<i32>>, board_width: usize, board_height: usize, bingo_number: i32) {
    for r in 0..board_width {
        for c in 0..board_height {
            if match_board[r][c] == bingo_number {
                match_board[r][c] = -1;
            }
        }
    }
}

fn part_two() {
    println!("PART TWO");
    // Initialize our Vector of game boards
    let board_width = 5;
    let board_height = 5;
    let mut game_boards = vec![vec![vec![0; board_width]; board_height]];
    let mut match_boards = vec![vec![vec![0; board_width]; board_height]];
    let mut winning_boards:Vec<usize> = vec![];  // track winning boards
    let mut winning_numbers:Vec<i32> = vec![];   // track winning numbers - if had more time, would make a board object to track various properties

    // Load the boards up with the board data
    load_boards(&mut game_boards, &mut match_boards, board_width, board_height);
    let bingo_numbers = [84,28,29,75,58,71,26,6,73,74,41,39,87,37,16,79,55,60,62,80,64,95,46,15,5,47,2,35,32,78,89,90,96,33,4,69,42,30,54,85,65,83,44,63,20,17,66,81,67,77,36,68,82,93,10,25,9,34,24,72,91,88,11,38,3,45,14,56,22,61,97,27,12,48,18,1,31,98,86,19,99,92,8,43,52,23,21,0,7,50,57,70,49,13,51,40,76,94,53,59];
    // We iterate through the match board and set the value to -1 if there is a match

    for bingo_number in bingo_numbers {
        for b in 0..match_boards.len() {
            if !winning_boards.contains(&b) {  // skip boards that have already won

                // Update the board with a -1 if it has a match
                mark_match(match_boards.get_mut(b).unwrap(), board_width, board_height, bingo_number);

                // Now, check the boards for a win
                if is_row_win(match_boards.get(b).unwrap()) ||
                    is_column_win(match_boards.get(b).unwrap()) {
                    winning_boards.push(b);
                    winning_numbers.push(bingo_number);
                }
            }
        }
    } // next bingo number
    announce_winner(winning_boards[winning_boards.len()-1], game_boards.clone(),
                    match_boards.clone(), winning_numbers[winning_boards.len()-1]);
}

fn main() {
    part_one();
    println!();
    part_two();
}