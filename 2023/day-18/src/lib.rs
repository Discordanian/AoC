// https://en.wikipedia.org/wiki/Shoelace_formula
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    direction: Direction,
    distance: i64,
}

pub fn instructions_to_area(instructions: Vec<Instruction>) -> i64 {
    let mut area: i64 = 0;
    let mut perimeter = 0;
    let mut points: Vec<(i64, i64)> = vec![(0, 0)];
    let mut current_point: (i64, i64) = (0, 0);

    for inst in instructions {
        match inst.direction {
            Direction::Up => current_point.1 -= inst.distance,
            Direction::Down => current_point.1 += inst.distance,
            Direction::Right => current_point.0 += inst.distance,
            Direction::Left => current_point.0 -= inst.distance,
        }
        perimeter += inst.distance;
        points.push(current_point);
    }

    for (p1, p2) in points.into_iter().tuples() {
        // area += (x2-x1)*(y2+y1)
        area += (p2.0 - p1.0) * (p2.1 + p1.1)
    }

    area.abs() / 2 + perimeter / 2 + 1
}

pub fn process_part1(input: &str) -> i64 {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = match parts.next().unwrap() {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Unknown Direction Recevied"),
            };
            let distance = parts.next().unwrap().parse().unwrap();

            Instruction {
                direction,
                distance,
            }
        })
        .collect();

    instructions_to_area(instructions)
}

pub fn p2_parse(line: &str) -> Instruction {
    let hex: String = line
        .chars()
        .skip(line.find('#').unwrap() + 1)
        .take(6)
        .collect::<String>();
    let dist = i64::from_str_radix(&hex[0..5], 16).unwrap();
    let dir = match hex.chars().last().unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!("Unknown Direction received"),
    };

    Instruction {
        direction: dir,
        distance: dist,
    }
}

pub fn process_part2(input: &str) -> i64 {
    let instructions: Vec<Instruction> = input.lines().map(p2_parse).collect();
    instructions_to_area(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";


    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 15);
    }
}
