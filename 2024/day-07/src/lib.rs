use regex::Regex;

// Ignores everything else in the str slice and returns all positive integers found
pub fn parse_vec_u64(s: &str) -> Vec<u64> {
    let re = Regex::new(r"(\d+)").expect("parse_vec_u64 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<u64>().unwrap());
    }

    retval
}

pub fn recursion_possible(target: u64, v: &[u64]) -> bool {
    if v.len() == 1 && v[0] == target {
        return true;
    };
    if v.len() == 1 && v[0] != target {
        return false;
    }
    assert!(v.len() > 1);
    let mut add_copy: Vec<u64> = Vec::new();
    let mut mul_copy: Vec<u64> = Vec::new();
    add_copy.push(v[0] + v[1]);
    mul_copy.push(v[0] * v[1]);
    let rest: Vec<u64> = v[2..].to_vec();
    add_copy.extend(rest.clone());
    mul_copy.extend(rest);

    recursion_possible(target, &add_copy) || recursion_possible(target, &mul_copy)
}

pub fn equation_possible(v: &[u64]) -> bool {
    let target = v[0];
    let rest = &v[1..];

    recursion_possible(target, rest)
}

pub fn equation_possible2(v: &[u64]) -> bool {
    let target = v[0];
    let rest = &v[1..];

    recursion_possible2(target, rest)
}

pub fn recursion_possible2(target: u64, v: &[u64]) -> bool {
    if v.len() == 1 && v[0] == target {
        return true;
    };
    if v.len() == 1 && v[0] != target {
        return false;
    }
    assert!(v.len() > 1);
    let mut add_copy: Vec<u64> = Vec::new();
    let mut mul_copy: Vec<u64> = Vec::new();
    let mut cat_copy: Vec<u64> = Vec::new();
    add_copy.push(v[0] + v[1]);
    mul_copy.push(v[0] * v[1]);
    // cat_copy
    let cat_str = format!("{}{}", v[0], v[1]);
    cat_copy.push(cat_str.parse().expect("Can make a u64"));

    let rest: Vec<u64> = v[2..].to_vec();
    add_copy.extend(rest.clone());
    mul_copy.extend(rest.clone());
    cat_copy.extend(rest);

    recursion_possible2(target, &add_copy)
        || recursion_possible2(target, &mul_copy)
        || recursion_possible2(target, &cat_copy)
}

pub fn process_part1(input: &str) -> u64 {
    let equations: Vec<Vec<u64>> = input.lines().map(parse_vec_u64).collect();

    equations
        .into_iter()
        .filter(|v| equation_possible(v))
        .map(|v| v[0])
        .sum()
}

pub fn process_part2(input: &str) -> u64 {
    let equations: Vec<Vec<u64>> = input.lines().map(parse_vec_u64).collect();

    equations
        .into_iter()
        .filter(|v| equation_possible2(v))
        .map(|v| v[0])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 3749);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 11387);
    }
}
