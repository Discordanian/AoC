use regex::Regex;

// Ignores everything else in the str slice and returns all integers found
pub fn parse_vec_i64(s: &str) -> Vec<i64> {
    let re = Regex::new(r"(-?\d+)").expect("parse_vec_i64 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<i64>().unwrap());
    }

    retval
}

pub struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    pub fn test(&self, apresses: i64, bpresses: i64) -> bool {
        apresses * self.button_a.0 + bpresses * self.button_b.0 == self.prize.0
            && apresses * self.button_a.1 + bpresses * self.button_b.1 == self.prize.1
    }
    pub fn test2(&self, apresses: i64, bpresses: i64) -> bool {
        apresses * self.button_a.0 + bpresses * self.button_b.0 == self.prize.0 + 10000000000000
            && apresses * self.button_a.1 + bpresses * self.button_b.1
                == self.prize.1 + 10000000000000
    }
}

// Really should learn nom
pub fn get_machines(input: &str) -> Vec<Machine> {
    let mut retval = vec![];
    for block in input.split("\n\n") {
        let mut line = block.lines();
        let a_vec = parse_vec_i64(line.next().unwrap());
        let b_vec = parse_vec_i64(line.next().unwrap());
        let prize_vec = parse_vec_i64(line.next().unwrap());
        retval.push(Machine {
            button_a: (a_vec[0], a_vec[1]),
            button_b: (b_vec[0], b_vec[1]),
            prize: (prize_vec[0], prize_vec[1]),
        });
    }
    retval
}

// Brute force
pub fn process_part1(input: &str) -> i64 {
    let machines: Vec<Machine> = get_machines(input);
    let mut answers: Vec<i64> = Vec::new();
    for m in &machines {
        let mut cost = i64::MAX;

        for a in 0..=100 {
            for b in 0..=100 {
                if m.test(a as i64, b as i64) {
                    cost = cost.min(3 * a as i64 + b as i64);
                }
            }
        }
        if cost < i64::MAX {
            answers.push(cost);
        }
    }

    answers.iter().sum()
}

// Math
pub fn process_part2(input: &str) -> i64 {
    let machines: Vec<Machine> = get_machines(input);
    let mut answers: Vec<i64> = Vec::new();
    for m in &machines {
        let px = m.prize.0 + 10000000000000;
        let py = m.prize.1 + 10000000000000;

        let ax = m.button_a.0;
        let ay = m.button_a.1;

        let bx = m.button_b.0;
        let by = m.button_b.1;

        // Let's Math solve.
        // https://en.wikipedia.org/wiki/Cramer%27s_rule
        let apresses: i64 = ((px * by) - (bx * py)) / ((by * ax) - (bx * ay));
        let bpresses: i64 = (py - (ay * apresses)) / by;

        if m.test2(apresses, bpresses) {
            answers.push(apresses * 3 + bpresses);
        }
    }

    answers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 480);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 875318608908);
    }
}
