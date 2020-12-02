use std::fs::File;
use std::io::{self, BufRead};
use std::borrow::Borrow;

fn main() {
    println!("Hello, world!");

    let numbers = read_lines();

    println!("first: {}, length: {}", numbers[0], numbers.len());

    let mut valid :u128 = 0;
    let mut idx: u128 = 8;

    for (_index, item) in numbers.iter().enumerate() {
        let token: Vec<&str> = item.split(":").collect();

        let sizeWithLetter: Vec<&str> = token[0].split(" ").collect();
        let sizes: Vec<&str> = sizeWithLetter[0].split("-").collect();

        let password = token[1].chars();
        let letter = sizeWithLetter[1].chars().next().unwrap();
        let min: usize = sizes[0].parse().unwrap();
        let max: usize = sizes[1].parse().unwrap();

        println!("{}", password.as_str());
        let count = password.filter(|s| s.eq(letter.borrow())).count();

        if count >= min && count <= max {
            valid+=1;
        }
        idx +=1;
        println!("{}: min {}, max {}, count {}", idx, min, max, count)
    }

    println!("{}", valid);
}

fn read_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
        lines.filter_map(io::Result::ok)
        .map(|s| s.parse().unwrap())
        .collect()
}
