use std::fs::File;
use std::io::{ self, BufRead };
use std::convert::TryInto;

fn main() {
    let map = read_lines();
    let _1by1 = find_trees(map, 1, 1);

    let map = read_lines();
    let _3by1 = find_trees(map, 3, 1);

    let map = read_lines();
    let _5by1 = find_trees(map, 5, 1);

    let map = read_lines();
    let _7by1 = find_trees(map, 7, 1);

    let map = read_lines();
    let _1by2 = find_trees(map, 1, 2);

    println!("{}", _1by1 * _3by1 * _5by1 * _7by1 * _1by2);
}

fn find_trees(map: Vec<String>, right: u128, down: u128) -> u128 {
    let mut tree_count = 0;
    let mut x_pos: u128 = 0;
    let mut y_pos: u128 = 0;

    for (_size, row) in map.iter().enumerate() {
        println!("{}", row.len());
        if y_pos != down {
            y_pos += 1;
        } else {
            y_pos = 0;
            x_pos += right;
            if x_pos > (row.len() - 1) as u128 {
                x_pos = x_pos - (row.len() as u128);
            }

            let position = row.chars().nth((x_pos).try_into().unwrap()).unwrap();
            if position == '#' {
                tree_count += 1;
            }

            y_pos += 1;
        }
    }
    tree_count
}

fn read_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
        lines.filter_map(io::Result::ok)
        .map(|s| s.parse().unwrap())
        .collect()
}
