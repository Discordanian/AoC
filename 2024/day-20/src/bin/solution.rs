use day_20::process_part1;
use day_20::process_part2;
use std::fs;

const SAVE: usize = 100;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Day Part 1 answer: {}", process_part1(&file, 2, SAVE));
    println!("Day Part 2 answer: {}", process_part2(&file, 20, SAVE));
}
