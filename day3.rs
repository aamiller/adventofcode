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

fn convert_from_binary(input: &String) -> i32 {
    isize::from_str_radix(input, 2).unwrap() as i32
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

fn part1(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut vect = vec![0; 12];
    let mut length = 0;

    for line in lines {
        length = length + 1;  // Why is it so complicated to get len of lines???? how low I've stooped
        if let Ok(code) = line {
            for (index, val) in code.chars().enumerate() {
                vect[index] = vect[index] + val.to_digit(10).unwrap();
            }
        }
    }

    let mut gamma_rate: String = "".to_owned();
    let mut epsilon_rate: String = "".to_owned();
    for val in &vect {
        if val > &((length/2)) {
            gamma_rate = gamma_rate + "1";
            epsilon_rate = epsilon_rate + "0";
        } else {
            gamma_rate = gamma_rate + "0";
            epsilon_rate = epsilon_rate + "1";
        }
    }

    convert_from_binary(&gamma_rate) * convert_from_binary(&epsilon_rate)
}

fn part2(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let lines = get_vect_from_lines(lines); // no more io::Lines finally
    let mut vect = vec![0; 12];

    for i in 0..lines.len() {
        for (index, val) in lines[i].chars().enumerate() {
            vect[index] = vect[index] + val.to_digit(10).unwrap();
        }
    }

    let mut gamma_rate: String = "".to_owned();
    let mut epsilon_rate: String = "".to_owned();
    for val in &vect {
        if val > &((lines.len() as u32/2)) {
            gamma_rate = gamma_rate + "1";
            epsilon_rate = epsilon_rate + "0";
        } else {
            gamma_rate = gamma_rate + "0";
            epsilon_rate = epsilon_rate + "1";
        }
    }

    // Copy for filtering
    let mut gamma_filtering = lines.clone();
    let mut epsilon_filtering = lines.clone();


    // Find gamma match
    for i in 0..lines.len() {
        let mut sum = 0;
        let length = gamma_filtering.len();
        for k in 0..gamma_filtering.len() {
            if gamma_filtering[k].chars().nth(i).unwrap() == '1' {
                sum = sum + 1;
            }
        }
        // Majority character
        let char_for_position = if &sum >= &(((length + 1)/2)) { '1' } else { '0' };
        gamma_filtering.retain(|x| x.chars().nth(i).unwrap() == char_for_position);
        if gamma_filtering.len() == 1 {
            break;
        }
    }

    // Find epsilon match
    for i in 0..lines.len() {
        let mut sum = 0;
        let length = epsilon_filtering.len();
        for k in 0..epsilon_filtering.len() {
            if epsilon_filtering[k].chars().nth(i).unwrap() == '1' {
                sum = sum + 1;
            }
        }

        // Minority character
        let char_for_position = if &sum <= &((length/2)) && &sum*2 != length  { '1' } else { '0' };
        epsilon_filtering.retain(|x| x.chars().nth(i).unwrap() == char_for_position);
        if epsilon_filtering.len() == 1 {
            break;
        }
    }

    convert_from_binary(&gamma_filtering[0]) * convert_from_binary(&epsilon_filtering[0])
}
    

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}