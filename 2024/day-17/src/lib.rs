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

pub fn combo(operand: u64, a: u64, b: u64, c: u64) -> u64 {
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
    let instructions = parse_vec_u64(line_iter.next().unwrap());

    while pc < instructions.len() {
        assert!(pc < (instructions.len() - 1));

        let operation = instructions[pc];
        let operand = instructions[pc + 1];

        match operation {
            0 => a = a >> combo(operand, a, b, c),
            1 => b ^= operand,
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

pub fn process_part2(input: &str) -> u64 {
    let mut line_iter = input.lines();
    let mut retval = 0;

    let _: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    let _: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    let _: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    line_iter.next().unwrap();
    let instructions = parse_vec_u64(line_iter.next().unwrap());

    let mut output = 23; // can never be 23
    let mut scan_idx = instructions.len() - 1;
    while scan_idx >= 0 {
        dbg!("Loop start");
        let targetnum = instructions[scan_idx];
        for d in 0..8 {
            let mut a = retval << 3 | d;
            let mut b = 0;
            let mut c = 0;
            let mut pc = 0;
            // dbg!(("Starting A", a));
            while pc < (instructions.len() - 2) {
                assert!(pc < instructions.len() - 2);
                let ins = instructions[pc];
                let operand = instructions[pc + 1];
                // dbg!((output, ins, operand));
                match ins {
                    0 => a = a >> combo(operand, a, b, c),
                    1 => b ^= operand,
                    2 => b = combo(operand, a, b, c) % 8,
                    3 => panic!(),
                    4 => b ^= c,
                    5 => output = combo(operand, a, b, c) % 8,
                    6 => b = a >> combo(operand, a, b, c),
                    7 => c = a >> combo(operand, a, b, c),
                    _ => unreachable!(),
                }
                pc += 2;
            } // going through instructions
              // dbg!((output, targetnum, d, retval));
            if output == targetnum {
                retval = retval << 3 | d;
                if scan_idx == 0 {
                    return retval >> 3;
                }
                assert!(scan_idx > 0);
                scan_idx -= 1;
                break;
            } else if d == 7 {
                retval = retval << 3;
                dbg!(retval);
                assert!(retval < 1_123_456_789_012);
                // panic!("Ouch");
            }
        }
    }
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

instructions: 0,1,5,4,3,0";

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
