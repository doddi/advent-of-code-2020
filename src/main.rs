use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");

    let numbers = read_lines();

    println!("first: {}, length: {}", numbers[0], numbers.len());

    for (firstIndex, firstItem) in numbers.iter().enumerate() {
        for (secondIndex, secondItem) in numbers.iter().enumerate() {
            for (thirdIndex, thirdItem) in numbers.iter().enumerate() {
                if (firstIndex != secondIndex ||
                    firstIndex != thirdIndex ||
                    secondIndex != thirdIndex) {
                    if (firstItem + secondItem + thirdItem == 2020) {
                        println!("Result: {}", firstItem * secondItem * thirdItem);
                    }
                }
            }
        }
    }
}

fn read_lines() -> Vec<u32> {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
        lines.filter_map(io::Result::ok)
        .map(|s| s.parse().unwrap())
        .collect()
}
