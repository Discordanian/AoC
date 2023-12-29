use nom::character::complete;
// use nom::sequence::separated_pair;
use nom::branch;
use nom::bytes::complete::tag;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Clone, Copy)]
struct Position {
    depth: u32,
    distance: u32,
}


fn parse_line(input: &str) -> IResult<&str, Instruction> {
    let (input, dir) = branch::alt((tag("forward"), tag("up"), tag("down")))( input,)?;
    let (input, _) = tag(" ")(input)?;
    let (input, length) = complete::u32(input)?;

    let retval = match dir {
        "forward" => Instruction::Forward(length),
        "down" => Instruction::Down(length),
        "up" => Instruction::Up(length),
        _ => panic!("In parse line, unknown direction"),
    };

    Ok((input,retval))
}

#[allow(dead_code)]
fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, depths) = nom::multi::separated_list1(complete::newline, parse_line)(input)?;
    Ok((input, depths))
}

pub fn process_part1(input: &str) -> u32 {
    // let x : Vec<Instruction> = input.lines().map(parse_line).flat_map(|Ok((_,x))| x).collect();
    let x : Vec<_> = input.lines().map(parse_line).map(|x| { let (_, inst) = x.unwrap(); inst}).collect();
    // let (_,x) : Vec<_> = parse_input(input).unwrap();
    
    let final_pos: Position = x.iter().fold(Position{depth: 0, distance: 0},|mut pos, inst| {
                  match inst {
                    Instruction::Forward(x) => pos.distance += x,
                    Instruction::Up(x) => pos.depth -= x,
                    Instruction::Down(x) => pos.depth += x,
                  };
                  pos});
    dbg!(&final_pos);
   //  dbg!(x);
   final_pos.depth * final_pos.distance
}

pub fn process_part2(input: &str) -> u32 {
    input.len() as u32
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
        assert_eq!(result, 0);
    }
}
