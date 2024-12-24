use day_24::process_part1;
use day_24::process_part2;
use std::fs;

const BITS: usize = 46;
const PAIRS: usize = 4;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Day Part 1 answer: {}", process_part1(&file, BITS));
    println!("Day Part 2 answer: {}", process_part2(&file, BITS, PAIRS));
}
