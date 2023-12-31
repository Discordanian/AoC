pub fn process_part1(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap_or(0) as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    dbg!(&grid);
    let line_count = grid.len();
    let col_count = grid[0].len();
    // dbg!((line_count, col_count));
    let mut gamma = 0;
    let mut epsilon = 0;
    for col in 0..col_count {
        let one_count = grid.iter().filter(|x| x[col] == 1).count();
        if col == 0 {
            dbg!(grid
                .iter()
                .filter(|x| x[col] == 1)
                .map(|x| x.clone())
                .collect::<Vec<Vec<usize>>>());
        }
        gamma *= 2;
        epsilon *= 2;
        dbg!((one_count, col));
        match one_count > line_count / 2 {
            true => gamma += 1,
            false => epsilon += 1,
        }
    }

    dbg!((gamma, epsilon));

    gamma * epsilon
}

pub fn process_part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../example.txt");

    #[test]
    fn part1_example() {
        let result = process_part1(INPUT);
        assert_eq!(result, 198);
    }

    #[test]
    fn part2_example() {
        let result = process_part2(INPUT);
        assert_eq!(result, 0);
    }
}
