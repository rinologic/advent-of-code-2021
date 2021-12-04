use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_file_reader(filename: &str) -> BufReader<File> {
        let datafile = File::open(filename).unwrap();
        return BufReader::new(datafile);
}

// We are going to load up two boards, one will be the game boards with all the values
// the second will be boards used to record matches.  Those will be the exact same as
// the game boards but we will replace a match with a M.  To get the final total, we
// can use the corresponding game board as a reference.
fn load_boards(game_boards: &mut Vec<Vec<Vec<i32>>>, match_boards: &mut Vec<Vec<Vec<i32>>>,
               board_width: usize, board_height: usize) {

    let board_data_file = get_file_reader("bingo_boards_real.txt");
    let mut board_data: Vec<String> = Vec::new();
    for (_index, line) in board_data_file.lines().enumerate() {
        board_data.push(line.unwrap());
    }

    // First we determine how many boards are in the file
    // Count the number of lines that are not spaces and
    // divide by the board height
    for (_index, _line) in board_data.iter().enumerate() {
        if _line == "" {
            // a space, next array
        }
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
    game_boards.remove(0); // Not sure yet why the first game board is empty
    match_boards.remove(0);
}

fn announce_winner(board_number: usize, gb: Vec<Vec<Vec<i32>>>, mb: Vec<Vec<Vec<i32>>>, bingo_number: i32, col_row: &str, col_row_num: usize)  {

    // 1760 was too low

    println!("BOARD {} IS A WINNER WITH THE CALL OF {} on {} {}!", board_number + 1, bingo_number, col_row, col_row_num+1);
    let game_board = gb.get(board_number);
    let match_board = mb.get(board_number);
    println!("\nGAME BOARD");
    println!("{:?}", game_board);
    println!("\nMATCH BOARD");
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
    println!();
    println!("\nREDEMPTION CODE IS {} * {} = {}", unmarked_sum, bingo_number, unmarked_sum * bingo_number);

    std::process::exit(0);
}

fn main() {

    // Initialize our Vector of game boards
    let board_width = 5;
    let board_height = 5;
    let mut game_boards = vec![vec![vec![0; board_width]; board_height]];
    let mut match_boards = vec![vec![vec![0; board_width]; board_height]];

    // Load the boards up with the board data
    load_boards(&mut game_boards, &mut match_boards, board_width, board_height);

    let bingo_numbers = [ 84,28,29,75,58,71,26,6,73,74,41,39,87,37,16,79,55,60,62,80,64,95,46,15,5,47,2,35,32,78,89,90,96,33,4,69,42,30,54,85,65,83,44,63,20,17,66,81,67,77,36,68,82,93,10,25,9,34,24,72,91,88,11,38,3,45,14,56,22,61,97,27,12,48,18,1,31,98,86,19,99,92,8,43,52,23,21,0,7,50,57,70,49,13,51,40,76,94,53,59 ];


    // We iterate through the match board and set the value to -1 if there is a match
    for bingo_number in bingo_numbers {
        for b in 0..match_boards.len() {
            for r in 0..board_width {
                for c in 0..board_height {
                    if match_boards.get(b).unwrap()[r][c] == bingo_number {
                        // match_board = match_boards.get_mut(b).unwrap()[r][c] = -1;
                        let mb = match_boards.get_mut(b);
                        mb.unwrap()[r][c] = -1;

                        // Check rows for a win
                        for i in 0..4 {
                            if  match_boards.get(b).unwrap()[i][0] == -1 &&
                                match_boards.get(b).unwrap()[i][1] == -1 &&
                                match_boards.get(b).unwrap()[i][2] == -1 &&
                                match_boards.get(b).unwrap()[i][3] == -1 &&
                                match_boards.get(b).unwrap()[i][4] == -1     {
                                announce_winner(b,
                                                game_boards.clone(),
                                                match_boards.clone(),
                                                bingo_number, "ROW", i);
                            }
                        }
                        // Check columns for a win
                        for i in 0..4 {
                            if  match_boards.get(b).unwrap()[0][i] == -1 &&
                                match_boards.get(b).unwrap()[1][i] == -1 &&
                                match_boards.get(b).unwrap()[2][i] == -1 &&
                                match_boards.get(b).unwrap()[3][i] == -1 &&
                                match_boards.get(b).unwrap()[4][i] == -1     {
                                announce_winner(b,
                                                game_boards.clone(),
                                                match_boards.clone(),
                                                bingo_number, "COLUMN", i);
                            }
                        }
                    }
                }
            }
        }
    }
    //println!("\nWITH MATCHES RECORDED");
    //for g in match_boards.iter() {
    //    println!("{:?}", g);
    //}
}