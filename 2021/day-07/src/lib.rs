pub fn process_part1(input: &str) -> i32 {
    let crabs: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // let avg = crabs.iter().sum::<i32>() / crabs.len() as i32;
    let start = *crabs.iter().min().expect("Min possible") as usize;
    let end = *crabs.iter().max().expect("Max possible") as usize;
    let mut retval: i32 = i32::MAX;

    for i in start..end {
        let cost: i32 = crabs.iter().map(|x| (x - i as i32).abs()).sum();
        retval = retval.min(cost);
    }

    retval
}

pub fn triangle_cost(n: i32) -> i32 {
    (0..=n).sum::<i32>()
}

pub fn process_part2(input: &str) -> i32 {
    let crabs: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let start = *crabs.iter().min().expect("Min possible") as usize;
    let end = *crabs.iter().max().expect("Max possible") as usize;
    let mut retval: i32 = i32::MAX;

    for i in start..end {
        let cost: i32 = crabs
            .iter()
            .map(|x| triangle_cost((x - i as i32).abs()))
            .sum();
        retval = retval.min(cost);
    }

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_example() {
        let result = process_part1(INPUT);
        assert_eq!(result, 37);
    }

    #[test]
    fn part2_example() {
        let result = process_part2(INPUT);
        assert_eq!(result, 168);
    }
}
