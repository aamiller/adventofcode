use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let res = part2(lines);   
        println!("{}", res);
    }
}
fn get_vect_from_lines(lines: io::Lines<io::BufReader<File>>) -> Vec<String> {
    let mut vect = Vec::new();
    for line in lines {
        if let Ok(ln) = line {
            vect.push(ln);
        }
    }
    return vect;
}

fn add_bingo_number(mut board: Vec<Vec<String>>, num: &str) -> Vec<Vec<String>> {
    for i in 0..5 { // row
        for k in 0..5 { // col 
            if board[i][k] == num {
                board[i][k] = "X".to_string();
            }
        }
    }
    board
}

fn check_board_for_win(board: &Vec<Vec<String>>) -> bool {
    for i in 0..5 { // row
        if board[i].iter().all(|item| item == "X") {
            return true
        }
        for k in 0..5 { // col
            if board[k][i] != "X".to_string() {
                break;
            }
            if k == 4 {
                return true;
            }
        }
    }
    false
}

fn add_board_from_vals(rows: Vec<String>) -> Vec<Vec<String>> {
    let mut array : Vec<Vec<String>>  = vec![vec!["".to_string(); 5]; 5];
    for i in 0..rows.len() {
        array[i] = rows[i].split_whitespace().into_iter().map(|x| x.to_string()).collect::<Vec<String>>();
    }
    array
}

fn sum_board(board: &Vec<Vec<String>>) -> i32 {
    let mut sum = 0;
    for i in 0..board.len() {
        for j in 0..board.len() {
            if board[i][j] != "X".to_string() {
                sum = sum + board[i][j].parse::<i32>().unwrap();
            }
        }
    }
    sum
}

fn part1(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let lines = get_vect_from_lines(lines);
    let bingo_nums = lines[0].split(',').collect::<Vec<&str>>();
    let mut boards: Vec<Vec<Vec<String>>> = vec![];

    for i in (2..lines.len()).step_by(6) {
        let board_rows: Vec<String> = vec![lines[i].clone(), lines[i + 1].clone(), lines[i + 2].clone(), lines[i + 3].clone(), lines[i + 4].clone()];
        let new_board: Vec<Vec<String>> = add_board_from_vals(board_rows);
        boards.append(&mut vec![new_board]);
    }

    for n in 0..bingo_nums.len() {
        for i in 0..boards.len() {
            boards[i] = add_bingo_number(boards[i].clone(), bingo_nums[n]);
            if check_board_for_win(&boards[i]) {
                return sum_board(&boards[i]) * bingo_nums[n].parse::<i32>().unwrap()
            }
        }
    }
    -1
}

fn part2(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let lines = get_vect_from_lines(lines);
    let bingo_nums = lines[0].split(',').collect::<Vec<&str>>();
    let mut boards: Vec<Vec<Vec<String>>> = vec![];

    for i in (2..lines.len()).step_by(6) {
        let board_rows: Vec<String> = vec![lines[i].clone(), lines[i + 1].clone(), lines[i + 2].clone(), lines[i + 3].clone(), lines[i + 4].clone()];
        let new_board: Vec<Vec<String>> = add_board_from_vals(board_rows);
        boards.append(&mut vec![new_board]);
    }

    let mut won_boards: Vec<usize> = vec![];
    for n in 0..bingo_nums.len() {
        for i in 0..boards.len() {
            if won_boards.contains(&i) {
                continue;
            }
            boards[i] = add_bingo_number(boards[i].clone(), bingo_nums[n]);
            if check_board_for_win(&boards[i]) {
                if won_boards.len() == boards.len() - 1 { // If last board
                    return sum_board(&boards[i]) * bingo_nums[n].parse::<i32>().unwrap()
                } else {
                    won_boards.push(i);
                }
            }
        }
    }
    
    -1
}

    

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}