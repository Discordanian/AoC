use day_18::process_part1;
use day_18::process_part2;
use std::fs;

const WIDTH: usize = 71;
const TIME: usize = 1024;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Day Part 1 answer: {}", process_part1(&file, WIDTH, TIME));
    println!("Day Part 2 answer: {}", process_part2(&file, WIDTH));
}
