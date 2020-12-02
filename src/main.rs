use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let numbers = read_lines();

    let mut valid :u128 = 0;

    for (_index, item) in numbers.iter().enumerate() {
        let token: Vec<&str> = item.split(":").collect();

        let size_with_letter: Vec<&str> = token[0].split(" ").collect();
        let sizes: Vec<&str> = size_with_letter[0].split("-").collect();

        let password = token[1].trim();
        let letter = size_with_letter[1].chars().next().unwrap();
        let min: usize = sizes[0].parse().unwrap() ;
        let max: usize = sizes[1].parse().unwrap();

        if password.chars().count() >= max {
            let mut valid_count: u8 = 0;

            if password.chars().nth(min-1).unwrap() == letter {
                valid_count += 1;
            }
            if password.chars().nth(max-1).unwrap() == letter {
                valid_count += 1;
            }
            if valid_count == 1 {
                valid += 1;
            }
        }
    }

    println!("valid: {}", valid);
}

fn read_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
        lines.filter_map(io::Result::ok)
        .map(|s| s.parse().unwrap())
        .collect()
}
