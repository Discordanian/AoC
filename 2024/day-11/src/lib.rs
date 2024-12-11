pub fn stone_after_x(stone: u128, blink: usize) -> u128 {
    let stone_string = format!("{}", stone);
    if blink == 0 {
        return 1;
    }
    if blink == 1 && stone_string.len() % 2 == 1 {
        return 1;
    }
    if stone_string.len() % 2 == 1 {
        return stone_after_x(stone * 2024, blink - 1);
    }
    let half = stone_string.len() / 2;
    let first = &stone_string[0..half];
    let second = &stone_string[half..];
    stone_after_x(first.parse().unwrap(), blink - 1)
        + stone_after_x(second.parse().unwrap(), blink - 1)
}

pub fn next_stones(stones: &[u128]) -> Vec<u128> {
    let mut retval = vec![];
    for &stone in stones.iter() {
        let stone_string = format!("{}", stone);
        if stone == 0 {
            retval.push(1);
        } else if stone_string.len() % 2 == 1 {
            retval.push(stone * 2024);
        } else {
            let half = stone_string.len() / 2;
            let first = &stone_string[0..half];
            let second = &stone_string[half..];
            retval.push(first.parse().unwrap());
            retval.push(second.parse().unwrap());
        }
    }
    retval
}
pub fn process_part1(input: &str) -> u128 {
    let mut stones: Vec<u128> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = next_stones(&stones);
    }
    stones.len() as u128
}

pub fn process_part2(input: &str) -> u128 {
    let mut stones: Vec<u128> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    stones.iter().map(|x| stone_after_x(*x, 55)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 0);
    }
}
