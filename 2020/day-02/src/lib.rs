use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

// 1-3 b: cdefg
fn parse(input: &str) -> IResult<&str, Vec<((u32, u32), char, &str)>> {
    seperated_list1(
        line_ending,
        tuple(
            seperated_pair(complete::u32, tag("-"), complete::u32),
            tag(" "),
            complete::char,
            tag(": "),
            complete::alpha1,
        ),
    )(input)
}

pub fn process_part1(input: &str) -> i32 {
    let policies: Vec<Policy> = input.lines().map(|x| Policy::from(x)).collect();
    policies.len() as i32
}

pub fn process_part2(input: &str) -> i32 {
    let policies: Vec<Policy> = input.lines().map(|x| Policy::from(x)).collect();
    policies.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "21-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 15);
    }
}
