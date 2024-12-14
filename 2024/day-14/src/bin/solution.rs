use day_14::process_part1;
use day_14::process_part2;
use std::fs;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Day Part 1 answer: {}", process_part1(&file, HEIGHT, WIDTH));
    println!("Day Part 2 answer: {}", process_part2(&file, HEIGHT, WIDTH));
}
