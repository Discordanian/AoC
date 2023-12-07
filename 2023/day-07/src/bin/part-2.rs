use day_07::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Day Part 2 answer: {}", process_part2(&file));
}
