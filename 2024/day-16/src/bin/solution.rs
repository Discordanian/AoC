use day_16::process_part1;
use day_16::process_part1_astar;
use day_16::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Day Part 1 answer: {}", process_part1(&file));
    println!("Day Part 1 A* answer: {}", process_part1_astar(&file));
    println!("Day Part 2 answer: {}", process_part2(&file));
}
