pub fn increasing(v: &[u32]) -> bool {
    let mut prev = 0;
    for x in v.iter() {
        if *x < prev {
            return false;
        }
        prev = *x;
    }
    true
}

pub fn input_to_tuple(input: &str) -> (usize, usize) {
    let x: Vec<usize> = input
        .split("-")
        .map(|x| x.parse().expect("Able to make usize"))
        .collect::<_>();

    (x[0], x[1])
}

pub fn has_double(v: &[u32]) -> bool {
    for i in 1..v.len() {
        if v[i] == v[i - 1] {
            return true;
        }
    }
    false
}

pub fn has_double2(v: &[u32]) -> bool {
    let mut prev = 10;
    let mut count = 0;

    for x in v.iter() {
        if *x == prev {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            count = 1;
        }
        prev = *x;
    }
    count == 2
}
pub fn is_valid(n: usize) -> bool {
    let a: Vec<u32> = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).expect("Able to make a digit"))
        .collect();
    increasing(&a) && has_double(&a)
}

pub fn is_valid2(n: usize) -> bool {
    let a: Vec<u32> = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).expect("Able to make a digit"))
        .collect();
    increasing(&a) && has_double2(&a)
}

pub fn process_part1(input: &str) -> usize {
    let tup = input_to_tuple(input.trim());
    let mut valids: Vec<usize> = Vec::new();

    for x in tup.0..tup.1 {
        if is_valid(x) {
            valids.push(x);
        }
    }

    valids.len()
}

pub fn process_part2(input: &str) -> usize {
    let tup = input_to_tuple(input.trim());
    let mut valids: Vec<usize> = Vec::new();

    for x in tup.0..tup.1 {
        if is_valid2(x) {
            valids.push(x);
        }
    }

    valids.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_increasing() {
        let result = is_valid(11111);
        assert!(result);
    }

    #[test]
    fn test_duplicate_fail() {
        let result = is_valid(123789);
        assert!(!result);
    }

    #[test]
    fn test_increase_fail() {
        let result = is_valid(223450);
        assert!(!result);
    }

    #[test]
    fn test_tuple_input() {
        let result = input_to_tuple("10-23");
        assert_eq!(result, (10, 23));
    }

    #[test]
    fn part2a() {
        let result = is_valid2(112233);
        assert!(result);
    }

    #[test]
    fn part2b() {
        let result = is_valid2(123444);
        assert!(!result);
    }

    #[test]
    fn part2c() {
        let result = is_valid2(111122);
        assert!(result);
    }
}
