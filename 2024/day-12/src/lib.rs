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
            }
        }
        retval += area * perimeter;
    }

    retval
}

pub fn process_part2(input: &str) -> i32 {
    let garden = make_garden(input);
    let mut retval = 0;
    let mut queue = VecDeque::<IPoint>::new();
    let mut seen = BTreeSet::<IPoint>::new();
    let mut plot = BTreeSet::<IPoint>::new();
    let mut plot_map: BTreeMap<IPoint, BTreeSet<IPoint>> = BTreeMap::new();

    for key_point in garden.keys() {
        if seen.contains(key_point) {
            continue;
        }

        let plant = garden[key_point];
        let mut area = 0;
        let mut perimeter = 0;

        queue.push_back(*key_point);
        seen.insert(*key_point);
        plot = BTreeSet::new();
        plot.insert(*key_point);
        plot_map.insert(*key_point, plot.clone());

        // Gather up plots
        while let Some(point) = queue.pop_front() {
            area += 1;
            perimeter += 4; //Assume a lone disconnected island
            if let Some(p) = plot_map.get_mut(key_point) {
                p.insert(point);
            }

            for next_point in DIRECTIONS.map(|dir| point + dir) {
                if garden.contains_key(&next_point) && garden[&next_point] == plant {
                    if !seen.contains(&next_point) {
                        seen.insert(next_point);
                        queue.push_back(next_point);
                    }
                }
            }
        }
    }

    plot_map
        .values()
        .map(|set| {
            let (area, fence) = area_fence_for_point(garden.clone(), set.clone());
            area * fence
        })
        .sum()
}

pub fn area_fence_for_point(garden: BTreeMap<IPoint, char>, set: BTreeSet<IPoint>) -> (i32, i32) {
    let area = set.len() as i32;
    let mut corner = 0;

    for point in set.iter() {
        // Is this a corner?
        let plant = garden.get(point);
        let up = plant == garden.get(&(*point + IPoint { x: 0, y: -1 }));
        let down = plant == garden.get(&(*point + IPoint { x: 0, y: 1 }));
        let right = plant == garden.get(&(*point + IPoint { x: 1, y: 0 }));
        let left = plant == garden.get(&(*point + IPoint { x: -1, y: 0 }));
        let up_right = plant == garden.get(&(*point + IPoint { x: 1, y: -1 }));
        let down_right = plant == garden.get(&(*point + IPoint { x: 1, y: 1 }));
        let up_left = plant == garden.get(&(*point + IPoint { x: -1, y: -1 }));
        let down_left = plant == garden.get(&(*point + IPoint { x: -1, y: 1 }));

        if !up && !left {
            corner += 1;
        }
        if !up && !right {
            corner += 1;
        }
        if !down && !right {
            corner += 1;
        }
        if !down && !left {
            corner += 1;
        }

        if up && right && !up_right {
            corner += 1;
        }
        if down && right && !down_right {
            corner += 1;
        }

        if up && left && !up_left {
            corner += 1;
        }
        if down && left && !down_left {
            corner += 1;
        }
    }

    (area, corner)
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
