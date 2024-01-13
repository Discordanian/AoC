pub fn process_part1(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap_or(0) as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let line_count = grid.len();
    let col_count = grid[0].len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for col in 0..col_count {
        let one_count = grid.iter().filter(|x| x[col] == 1).count();
        gamma *= 2;
        epsilon *= 2;
        match one_count > line_count / 2 {
            true => gamma += 1,
            false => epsilon += 1,
        }
    }
    gamma * epsilon
}

pub fn process_part2(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap_or(0) as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let line_count = grid.len();
    let col_count = grid[0].len();

    // Create a vec of the most common entries
    let mut most_common : Vec<usize> = Vec::new();
    for col in 0..col_count {
        let one_count = grid.iter().filter(|x| x[col] == 1).count();
        match one_count > line_count / 2 {
            true => most_common.push(1),
            false => most_common.push(0),
        }
    }


    let mut gamma_candidates : Vec<Vec<usize>> = grid.clone();
    let mut col = 0;
    while gamma_candidates.len() != 1 && col < col_count {
        gamma_candidates = gamma_candidates.iter().filter(|x| x[col] == most_common[col]).collect();
        col += 1;
    }

    let mut col = 0;
    let mut epsilon_candidates : Vec<Vec<usize>> = grid.clone();
    while epsilon_candidates.len() != 1 && col < col_count {
        epsilon_candidates = epsilon_candidates.iter().filter(|x| x[col] != most_common[col]).collect();
        col += 1;
    }

    bin2dec(gamma_candidates[0]) * bin2dec(epsilon_candidates[0])
}

fn bin2dec(arr: Vec<usize>) -> usize {
    arr.iter().fold(0,|mut r, x| {r *=2; r+=x; r})
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
