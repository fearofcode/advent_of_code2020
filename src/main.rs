use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::hash::Hash;
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

fn day4part1() {
    let file_contents = fs::read_to_string("day4.input").unwrap();

    // breaking this up into steps to make debugging easier

    // the collects are not required and these steps could be combined but whatever, it doesn't
    // really matter

    let joined_passports: Vec<String> = file_contents
        .split("\n\n")
        .map(|pw_str| pw_str.replace("\n", " "))
        .collect();

    // cid is not required
    let required_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();

    let passport_fields: Vec<HashSet<&str>> = joined_passports
        .iter()
        .map(|line| {
            // "byr:1971 eyr:2039 hgt:172in pid:170cm hcl:17106b iyr:2012 ecl:gry cid:339"
            line.split_whitespace()
                .map(|chunk| chunk.split(":").nth(0).unwrap())
                .collect()
        })
        .collect();

    let valid_passport_count = passport_fields
        .iter()
        .filter(|fields| required_fields.is_subset(&fields))
        .count();

    println!("valid passport count = {}", valid_passport_count);
}

fn is_valid_passport_str_day4part2(line: &str) -> bool {
    // cid is not required
    let required_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();

    let key_map: HashMap<&str, &str> = line
        .split_whitespace()
        .map(|chunk| {
            let parts: Vec<&str> = chunk.split(":").collect();
            (parts[0], parts[1])
        })
        .collect();
    let keys: HashSet<&str> = key_map.keys().map(|key| *key).collect();
    if required_fields.is_subset(&keys) {
        let mut is_valid = true;
        for (key, value) in key_map.iter() {
            let key = *key;
            let value = *value;
            // not going to bother writing a parser with a validation predicate
            // although that's what I would do if I was going to reuse these records elsewhere
            match key {
                "byr" => {
                    if value.len() != 4 {
                        is_valid = false;
                        break;
                    }
                    let parsed_value: usize = value.parse().unwrap();
                    if parsed_value < 1920 || parsed_value > 2002 {
                        is_valid = false;
                        break;
                    }
                }
                "iyr" => {
                    if value.len() != 4 {
                        is_valid = false;
                        break;
                    }
                    let parsed_value: usize = value.parse().unwrap();
                    if parsed_value < 2010 || parsed_value > 2020 {
                        is_valid = false;
                        break;
                    }
                }
                "eyr" => {
                    if value.len() != 4 {
                        is_valid = false;
                        break;
                    }
                    let parsed_value: usize = value.parse().unwrap();
                    if parsed_value < 2020 || parsed_value > 2030 {
                        is_valid = false;
                        break;
                    }
                }
                "hgt" => {
                    if !value.ends_with("cm") && !value.ends_with("in") {
                        is_valid = false;
                        break;
                    }
                    if value.ends_with("cm") {
                        let parsed_value: usize =
                            value.strip_suffix("cm").unwrap().parse().unwrap();
                        // If cm, the number must be at least 150 and at most 193.
                        if parsed_value < 150 || parsed_value > 193 {
                            is_valid = false;
                            break;
                        }
                    } else {
                        let parsed_value: usize =
                            value.strip_suffix("in").unwrap().parse().unwrap();
                        // If in, the number must be at least 59 and at most 76.
                        if parsed_value < 59 || parsed_value > 76 {
                            is_valid = false;
                            break;
                        }
                    }
                }
                "hcl" => {
                    if !value.starts_with("#") || value.len() != 7 {
                        is_valid = false;
                        break;
                    }

                    let valid_chars: HashSet<char> = vec![
                        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
                        'f',
                    ]
                    .into_iter()
                    .collect();

                    for char in value.strip_prefix("#").unwrap().chars() {
                        if !valid_chars.contains(&char) {
                            is_valid = false;
                            break;
                        }
                    }
                }
                "ecl" => {
                    let valid_colors: HashSet<&str> =
                        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                            .into_iter()
                            .collect();

                    if !valid_colors.contains(value) {
                        is_valid = false;
                        break;
                    }
                }
                "pid" => {
                    if value.len() != 9 {
                        is_valid = false;
                        break;
                    }

                    // could also do a regex match but this is simple enough to bang out
                    let valid_chars: HashSet<char> =
                        vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
                            .into_iter()
                            .collect();

                    for char in value.chars() {
                        if !valid_chars.contains(&char) {
                            is_valid = false;
                            break;
                        }
                    }
                }
                "cid" => { /* do nothing */ }
                _ => panic!("Shouldn't happen"),
            }
        }
        is_valid
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password() {
        // invalid
        assert_eq!(
            is_valid_passport_str_day4part2(
                "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
            ),
            false
        );
        assert_eq!(
            is_valid_passport_str_day4part2(
                "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"
            ),
            false
        );
        assert_eq!(
            is_valid_passport_str_day4part2(
                "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
            ),
            false
        );
        assert_eq!(
            is_valid_passport_str_day4part2(
                "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
            ),
            false
        );

        // valid
        assert_eq!(
            is_valid_passport_str_day4part2(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"
            ),
            true
        );
        assert_eq!(
            is_valid_passport_str_day4part2(
                "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
            ),
            true
        );
        assert_eq!(
            is_valid_passport_str_day4part2(
                "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"
            ),
            true
        );
        assert_eq!(
            is_valid_passport_str_day4part2(
                "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            ),
            true
        );
    }
}

fn day4part2() {
    let file_contents = fs::read_to_string("day4.input").unwrap();

    let joined_passports: Vec<String> = file_contents
        .split("\n\n")
        .map(|pw_str| pw_str.replace("\n", " "))
        .collect();

    let mut valid_passport_count = 0;

    for line in joined_passports {
        if is_valid_passport_str_day4part2(&line) {
            valid_passport_count += 1;
        }
    }
    println!("valid password count = {}", valid_passport_count);
}

fn main() {
    day4part2();
}
