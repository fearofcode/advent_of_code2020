use std::fs::File;
use std::io::{self, BufRead};

// this is shitty code written to get the right answer as soon as possible and not how I write
// actual production code

fn day1() {
    let file = File::open("day1.input").unwrap();
    let str_lines = io::BufReader::new(file).lines();

    let numbers: Vec<usize> = str_lines
        .into_iter()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers.iter().enumerate() {
            if j < i {
                continue;
            }

            for (k, num3) in numbers.iter().enumerate() {
                if k < j {
                    continue;
                }

                if num1 + num2 + num3 == 2020 {
                    println!(
                        "{} * {} * {} = {}. {} + {} + {} = {}",
                        num1,
                        num2,
                        num3,
                        num1 * num2 * num3,
                        num1,
                        num2,
                        num3,
                        num1 + num2 + num3
                    );
                }
            }
        }
    }
}

fn day2() {
    let file = File::open("day2.input").unwrap();
    let str_lines = io::BufReader::new(file).lines();

    let mut correct_password_cnt = 0;

    for line in str_lines {
        // example input: 6-9 z: qzzzzxzzfzzzz
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        let bound_str = parts[0];
        // could also use regexes here
        let bound_parts: Vec<usize> = bound_str
            .split("-")
            .map(|token| token.parse().unwrap())
            .collect();
        let lower_bound = bound_parts[0];
        let upper_bound = bound_parts[1];

        let goal_char_part = parts[1];
        let goal_chars: Vec<char> = goal_char_part.chars().collect::<Vec<char>>();
        let goal_char = goal_chars[0];

        let test_pw = parts[2];

        let mut occurrences = 0;

        for pw_chr in test_pw.chars() {
            if pw_chr == goal_char {
                occurrences += 1;
            }
        }

        if occurrences >= lower_bound && occurrences <= upper_bound {
            correct_password_cnt += 1;
        }
    }

    println!("correct passwords (first part): {}", correct_password_cnt);

    correct_password_cnt = 0;

    // need to reread the file to make borrow checker happy
    let file = File::open("day2.input").unwrap();
    let str_lines = io::BufReader::new(file).lines();

    for line in str_lines {
        // example input: 6-9 z: qzzzzxzzfzzzz
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        let bound_str = parts[0];
        // could also use regexes here
        let bound_parts: Vec<usize> = bound_str
            .split("-")
            .map(|token| token.parse().unwrap())
            .collect();
        let index1 = bound_parts[0] - 1;
        let index2 = bound_parts[1] - 1;

        let goal_char_part = parts[1];
        let goal_chars: Vec<char> = goal_char_part.chars().collect::<Vec<char>>();
        let mut goal_bytes = [0; 1];
        goal_chars[0].encode_utf8(&mut goal_bytes);
        let goal_byte = goal_bytes[0];

        let test_pw = parts[2].as_bytes();

        let occurs_at_index1 = test_pw[index1] == goal_byte;
        let occurs_at_index2 = test_pw[index2] == goal_byte;

        if (occurs_at_index1 && !occurs_at_index2) || (occurs_at_index2 && !occurs_at_index1) {
            correct_password_cnt += 1;
        }
    }

    println!("correct passwords (second part): {}", correct_password_cnt);
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum MapCell {
    Square,
    Tree,
}

fn trees_encountered(right: usize, down: usize) -> usize {
    let file = File::open("day3.input").unwrap();

    // could also use bool and you'd have to pick whether true meant tree or square
    let map_lines: Vec<Vec<MapCell>> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| {
                    assert!(c == '.' || c == '#');
                    if c == '.' {
                        MapCell::Square
                    } else {
                        MapCell::Tree
                    }
                })
                .collect()
        })
        .collect();

    let mut current_col = 0;

    let mut trees_encountered = 0;

    for (i, line) in map_lines.iter().step_by(down).enumerate() {
        // let column wrap around
        let effective_col = current_col % line.len();
        if line[effective_col] == MapCell::Tree && i > 0 {
            trees_encountered += 1;
        }

        current_col += right;
    }

    trees_encountered
}

fn day3() {
    /*
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    */

    let right1down1 = trees_encountered(1, 1);
    // print out values to see if we get reasonable-looking return values
    println!("right 1 down 1 = {}", right1down1);
    let right3down1 = trees_encountered(3, 1);
    println!("right 3 down 1 = {}", right3down1);
    assert_eq!(right3down1, 272);

    let right5down1 = trees_encountered(5, 1);
    println!("right 5 down 1 = {}", right5down1);
    let right7down1 = trees_encountered(7, 1);
    println!("right 7 down 1 = {}", right7down1);
    let right1down2 = trees_encountered(1, 2);
    println!("right 1 down 2 = {}", right1down2);

    println!(
        "product = {}",
        right1down1 * right3down1 * right5down1 * right7down1 * right1down2
    );
}

fn main() {
    day3();
}
