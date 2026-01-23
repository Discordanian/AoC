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

fn find(program: &[u64], target: &[u64], ans: u64) -> Option<u64> {
    if target.is_empty() {
        return Some(ans);
    }

    for t in 0..8 {
        let a = ans << 3 | t;
        let mut b = 0u64;
        let mut c = 0u64;
        let mut output: Option<u64> = None;
        let mut adv3 = false;

        // Process instructions up to (but not including) the final JNZ
        for pointer in (0..program.len() - 2).step_by(2) {
            let ins = program[pointer];
            let operand = program[pointer + 1];

            match ins {
                0 => {
                    assert!(!adv3, "program has multiple ADVs");
                    assert_eq!(operand, 3, "program has ADV with operand other than 3");
                    adv3 = true;
                }
                1 => b ^= operand,
                2 => b = combo(operand, a, b, c) % 8,
                3 => panic!("program has JNZ inside expected loop body"),
                4 => b ^= c,
                5 => {
                    assert!(output.is_none(), "program has multiple OUT");
                    output = Some(combo(operand, a, b, c) % 8);
                }
                6 => b = a >> combo(operand, a, b, c),
                7 => c = a >> combo(operand, a, b, c),
                _ => unreachable!(),
            }
            
            // Check if output matches target (this check happens after each instruction)
            if let Some(out) = output {
                if out == target[target.len() - 1] {
                    if let Some(sub) = find(program, &target[..target.len() - 1], a) {
                        return Some(sub);
                    }
                }
            }
        }
    }

    None
}

pub fn process_part2(input: &str) -> u64 {
    let mut line_iter = input.lines();
    
    // Skip register values (first 3 lines)
    let _: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    let _: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    let _: u64 = parse_vec_u64(line_iter.next().unwrap())[0];
    line_iter.next().unwrap(); // Skip empty line
    let instructions = parse_vec_u64(line_iter.next().unwrap());

    // Assert program ends with JNZ 0
    assert_eq!(
        instructions[instructions.len() - 2..],
        [3, 0],
        "program does not end with JNZ 0"
    );

    // The target is the program itself (we're working backwards)
    find(&instructions, &instructions, 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, 14680);
    }
}
