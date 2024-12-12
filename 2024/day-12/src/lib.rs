use std::collections::{BTreeMap, BTreeSet};

const DIRECTIONS: [IPoint; 4] = [
    IPoint { x: 1, y: 0 },
    IPoint { x: -1, y: 0 },
    IPoint { x: 0, y: 1 },
    IPoint { x: 0, y: -1 },
];

#[derive(Ord, PartialOrd, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IPoint {
    x: i32,
    y: i32,
}

impl std::ops::Add for IPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for IPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub fn make_garden(input: &str) -> BTreeMap<IPoint, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, val)| {
                (
                    IPoint {
                        x: x as i32,
                        y: y as i32,
                    },
                    val,
                )
            }) //move required for y value in closure
        })
        .collect()
}
pub fn process_part1(input: &str) -> u32 {
    let garden = make_garden(input);

    0
}

pub fn process_part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 1930);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 1206);
    }
}
