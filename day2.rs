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

fn part1(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;
    for line in lines {
        if let Ok(curr) = line {
            let vec: Vec<&str> = curr.split_whitespace().collect();
            let num = vec[1].parse::<i32>().unwrap();
            if vec[0] == "down" {
                depth = depth + num;
            }
            if vec[0] == "up" {
                depth = depth - num;
            }
            if vec[0] == "forward" {
                horizontal = horizontal + num;
            }
        }
        println!("{} {}", depth, horizontal);
    }
    depth * horizontal
}

fn part2(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in lines {
        if let Ok(curr) = line {
            let vec: Vec<&str> = curr.split_whitespace().collect();
            let num = vec[1].parse::<i32>().unwrap();
            if vec[0] == "down" {
                aim = aim + num;
            }
            if vec[0] == "up" {
                aim = aim - num;
            }
            if vec[0] == "forward" {
                horizontal = horizontal + num;
                depth = depth + (aim * num);
            }
        }
        println!("{} {}", depth, horizontal);
    }
    depth * horizontal
}
    

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}