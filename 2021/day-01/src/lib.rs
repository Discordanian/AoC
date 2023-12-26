use itertools::Itertools;
use nom::character::complete;
// use nom::bytes::complete::tag;
use nom::IResult;

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, depth) = complete::u32(input)?;
    Ok((input, depth))
}

fn parse_input(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, depths) = nom::multi::separated_list1(complete::newline, parse_line)(input)?;
    Ok((input, depths))
}

pub fn process_part1(input: &str) -> usize {
    let (_, depths) = parse_input(input).unwrap();

    depths
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

pub fn process_part2(input: &str) -> usize {
    let (_, depths) = parse_input(input).unwrap();

    depths
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect::<Vec<u32>>()
        .iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../example.txt");

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 5);
    }
}
