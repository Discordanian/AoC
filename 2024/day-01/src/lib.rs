pub fn process_part1(input: &str) -> u32 {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    for line in input.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        let l: u32 = v[0].parse().expect("Parse fail");
        let r: u32 = v[1].parse().expect("Parse fail");
        left_list.push(l);
        right_list.push(r);
    }

    left_list.sort();
    right_list.sort();

    std::iter::zip(left_list, right_list)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    for line in input.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        let l: u32 = v[0].parse().expect("Parse fail");
        let r: u32 = v[1].parse().expect("Parse fail");
        left_list.push(l);
        right_list.push(r);
    }

    left_list.sort();
    right_list.sort();

    let mut sum = 0;
    for val in &left_list {
        let mult = right_list.iter().filter(|x| **x == *val).count() as u32;
        sum += val * mult;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 11);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 31);
    }
}
