use regex::Regex;

// Ignores everything else in the str slice and returns all positive integers found
pub fn parse_vec_u32(s: &str) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").expect("parse_vec_u32 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<u32>().unwrap());
    }

    retval
}
pub fn parse_vec_u64(s: &str) -> Vec<u64> {
    let re = Regex::new(r"(\d+)").expect("parse_vec_u32 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<u64>().unwrap());
    }

    retval
}

pub fn combo(operand: u32, a: u64, b: u64, c: u64) -> u64 {
    assert!(operand < 7);
    match operand {
        0..=3 => operand as u64,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    }
}

pub fn process_part1(input: &str) -> String {
    let mut line_iter = input.lines();
    let mut retval: Vec<u64> = Vec::new();

    let mut a: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    let mut b: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    let mut c: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    let mut pc: usize = 0;
    line_iter.next().unwrap();
    let instructions = parse_vec_u32(line_iter.next().unwrap());

    while pc < instructions.len() {
        assert!(pc < (instructions.len() - 1));

        let operation = instructions[pc];
        let operand = instructions[pc + 1];

        match operation {
            0 => a = a >> combo(operand, a, b, c),
            1 => b ^= (operand as u64),
            2 => b = combo(operand, a, b, c) % 8,
            3 => {
                if a != 0 {
                    pc = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => retval.push(combo(operand, a, b, c) % 8),
            6 => b = a >> combo(operand, a, b, c),
            7 => c = a >> combo(operand, a, b, c),
            _ => panic!("Bad operation received"),
        }
        pc += 2;
    }

    retval
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn process_part2(input: &str) -> u32 {
    input.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 117440);
    }
}
