pub fn run_computer_test(mut program: Vec<i64>) -> i64 {
    let mut pointer: usize = 0;

    while program[pointer] != 99 {
        assert!(program.len() > pointer + 4);
        let lhs: i64 = program[program[pointer + 1] as usize];
        let rhs: i64 = program[program[pointer + 2] as usize];
        let ptr: usize = program[pointer + 3] as usize;
        match program[pointer] {
            1 => program[ptr] = rhs + lhs,
            2 => program[ptr] = rhs * lhs,
            _ => panic!("Unknown Opcode"),
        }
        pointer += 4; // Next opcode
    }

    program[0]
}

pub fn run_computer(mut program: Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut pointer: usize = 0;
    program[1] = noun;
    program[2] = verb;

    while program[pointer] != 99 {
        assert!(program.len() > pointer + 4);
        let lhs: i64 = program[program[pointer + 1] as usize];
        let rhs: i64 = program[program[pointer + 2] as usize];
        let ptr: usize = program[pointer + 3] as usize;
        match program[pointer] {
            1 => program[ptr] = rhs + lhs,
            2 => program[ptr] = rhs * lhs,
            _ => panic!("Unknown Opcode"),
        }
        pointer += 4; // Next opcode
    }

    program[0]
}

pub fn process_part1(input: &str) -> i64 {
    let computer: Vec<i64> = input
        .trim()
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<i64>().expect("Parsable Number in input"))
        .collect();
    run_computer(computer, 12, 2)
}

pub fn process_part2(input: &str) -> i64 {
    let computer: Vec<i64> = input
        .trim()
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<i64>().expect("Parsable Number in input"))
        .collect();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let memory = computer.clone();
            let r = run_computer(memory, noun as i64, verb as i64);
            if r == 19690720 {
                return (100 * noun + verb) as i64;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a_works() {
        let result = run_computer_test(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(result, 3500);
    }

    #[test]
    fn part1b_works() {
        let result = run_computer_test(vec![1, 0, 0, 0, 99]);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1c_works() {
        let result = run_computer_test(vec![2, 3, 0, 3, 99]);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1d_works() {
        let result = run_computer_test(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1e_works() {
        let result = run_computer_test(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(result, 30);
    }
}
