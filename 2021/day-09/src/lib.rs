pub fn process_part1(input: &str) -> i32 {
    let map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|d| d.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    let height = map.len();
    let width = map[0].len();
    let mut retval = 0;
    for (r, v) in map.iter().enumerate() {
        for (c, h) in v.iter().enumerate() {
            let left = c == 0 || map[r][c - 1] > *h;
            let right = c == (width - 1) || map[r][c + 1] > *h;
            let up = r == 0 || map[r - 1][c] > *h;
            let down = r == (height - 1) || map[r + 1][c] > *h;
            if left && right && up && down {
                retval += *h + 1;
            }
        }
    }
    retval
}

pub fn process_part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1_example() {
        let result = process_part1(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    #[ignore]
    fn part2_example() {
        let result = process_part2(INPUT);
        assert_eq!(result, 1134);
    }
}
