use regex::Regex;

pub fn multiply(s: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Multiply regex is bad");

    let retval = if let Some((_, [a, b])) = re.captures_iter(s).map(|c| c.extract()).next() {
        a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
    } else {
        0
    };
    retval
}

pub fn process_part1(input: &str) -> u32 {
    let re =
        Regex::new(r"mul\(([1-9][0-9]?[0-9]?),([1-9][0-9]?[0-9]?)\)").expect("Poorly formed regex");

    let mut results = vec![];
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((a, b));
    }

    results
        .iter()
        .map(|(a, b)| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\([1-9][0-9]?[0-9]?,[1-9][0-9]?[0-9]?\))")
        .expect("Poorly formed regex");

    let mut results = vec![];
    for (_, [a]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push(a);
    }

    let mut enabled = true;
    let mut sum = 0;

    for s in results.iter() {
        match *s {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    sum += multiply(s);
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT1);
        assert_eq!(result, 161);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, 48);
    }
}
