use day_10::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Day Part 1 answer: {}", process_part1(&file));
}
