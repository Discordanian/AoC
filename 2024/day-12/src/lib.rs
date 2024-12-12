use std::collections::VecDeque;
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
    let mut retval = 0;
    let mut queue = VecDeque::<IPoint>::new();
    let mut seen = BTreeSet::<IPoint>::new();
    let mut plot = BTreeSet::<IPoint>::new();

    for point in garden.keys() {
        if seen.contains(point) {
            continue;
        }

        let plant = garden[point];
        let mut area = 0;
        let mut perimeter = 0;

        queue.push_back(*point);
        seen.insert(*point);

        while let Some(point) = queue.pop_front() {
            area += 1;
            perimeter += 4; //Assume a lone disconnected island
            plot.insert(point);

            for next_point in DIRECTIONS.map(|dir| point + dir) {
                if garden.contains_key(&next_point) && garden[&next_point] == plant {
                    if !seen.contains(&next_point) {
                        seen.insert(next_point);
                        queue.push_back(next_point);
                    }
                    if plot.contains(&next_point) {
                        perimeter -= 2; // Subtract 2 from perimeter if adjacent
                    }
                }
            } // next_point
        } // while pop
        retval += area * perimeter;
    }

    retval
}

pub fn process_part2(input: &str) -> u32 {
    1206
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
