use nom::character::complete;
use nom::branch;
use nom::bytes::complete::tag;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

// Part one only needed position
#[derive(Debug, Clone, Copy)]
struct Position {
    depth: i32,
    distance: i32,
}

// Part 2 has 'aim'
#[derive(Debug, Clone, Copy)]
struct Submarine {
    depth: i32,
    distance: i32,
    aim: i32,
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    let (input, dir) = branch::alt((tag("forward"), tag("up"), tag("down")))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, length) = complete::i32(input)?;

    let retval = match dir {
        "forward" => Instruction::Forward(length),
        "down" => Instruction::Down(length),
        "up" => Instruction::Up(length),
        _ => panic!("In parse line, unknown direction"),
    };

    Ok((input, retval))
}

#[allow(dead_code)]
fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, depths) = nom::multi::separated_list1(complete::newline, parse_line)(input)?;
    Ok((input, depths))
}

pub fn process_part1(input: &str) -> i32 {
    let (_, x) = parse_input(input).unwrap();

    let final_pos: Position = x.iter().fold(
        Position {
            depth: 0,
            distance: 0,
        },
        |mut pos, inst| {
            match inst {
                Instruction::Forward(x) => pos.distance += x,
                Instruction::Up(x) => pos.depth -= x,
                Instruction::Down(x) => pos.depth += x,
            };
            pos
        },
    );
    final_pos.depth * final_pos.distance
}

pub fn process_part2(input: &str) -> i32 {
    let (_, x) = parse_input(input).unwrap();

    let final_pos: Submarine = x.iter().fold(
        Submarine {
            depth: 0,
            distance: 0,
            aim: 0,
        },
        |mut sub, inst| {
            match inst {
                Instruction::Forward(x) => {
                    sub.distance += x;
                    sub.depth += sub.aim * x;
                }
                Instruction::Up(x) => sub.aim -= x,
                Instruction::Down(x) => sub.aim += x,
            };
            sub
        },
    );
    final_pos.depth * final_pos.distance
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../example.txt");

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 150);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 900);
    }
}
