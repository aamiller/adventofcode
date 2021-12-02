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
    let mut count = -1; // Iterator strugglebus fenceposting
    let mut prev = "-100".to_string();
    for line in lines {
        if let Ok(curr) = line {
            print!("Curr {} Count {} ", curr, count);
            if prev.parse::<i32>().unwrap() < curr.parse::<i32>().unwrap() {
                count = count + 1;
                println!("incr");
            } else {
                println!("");
            }
            prev = curr;
            }
        }
    count
}

fn part2(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut vec = Vec::new();
    let mut sums = Vec::new();
    for line in lines {
        if let Ok(curr) = line {
            vec.push(curr.parse::<i32>().unwrap());
        }
    }
    for i in 0..(vec.len()-2) {
        sums.push(vec[i] + vec[i+1] + vec[i+2]);
    }
    
    let mut prev = sums[0];
    let mut count = 0;
    for i in 1..sums.len() {
        if prev < sums[i] {
            count = count + 1;
        }
        prev = sums[i];
    }
    count
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}