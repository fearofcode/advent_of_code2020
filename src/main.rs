use std::fs::File;
use std::io::{self, BufRead};

fn day1() {
    let file = File::open("day1.input").unwrap();
    let str_lines = io::BufReader::new(file).lines();

    let numbers: Vec<usize> = str_lines.into_iter().map(|line| line.unwrap().parse().unwrap()).collect();

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
                    println!("{} * {} * {} = {}. {} + {} + {} = {}",
                             num1,
                             num2,
                             num3,
                             num1 * num2 * num3,
                             num1,
                             num2,
                             num3,
                             num1 + num2 + num3);
                }
            }

        }
    }
}

fn main() {
    day1();
}
