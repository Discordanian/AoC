use glam::IVec2;

use std::collections::HashMap;

pub fn input_to_hashmap(input: &str) -> HashMap<IVec2, char> {
    let mut grid_map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid_map.insert(IVec2::new(x as i32, y as i32), ch);
        }
    }
    grid_map
}
pub fn process_part1(input: &str) -> u32 {
    let map = input_to_hashmap(input);
    map.len() as u32
}

pub fn process_part2(input: &str) -> u32 {
    let map = input_to_hashmap(input);
    map.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 102);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 102);
    }
}
