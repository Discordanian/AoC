pub fn process_part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .iter()
        .map(|line| {
            let arr: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.trim().parse().unwrap())
                .collect();
            next1_value(arr)
        })
        .sum()
}

pub fn next1_value(arr: Vec<i32>) -> i32 {
    let mut matrix: Vec<Vec<i32>> = vec![arr];

    let mut idx = 0;
    while !matrix[idx].iter().all(|&x| x == 0) {
        let mut next_arr: Vec<i32> = vec![];
        for i in 0..(matrix[idx].len() - 1) {
            next_arr.push(matrix[idx][i + 1] - matrix[idx][i]);
        }
        matrix.push(next_arr);
        idx += 1;
    }

    let mut last_val = 0;
    for i in (0..(matrix.len() - 1)).rev() {
        last_val += matrix[i].last().unwrap();
        matrix[i].push(last_val);
    }

    *matrix[0].last().unwrap()
}

pub fn process_part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .iter()
        .map(|line| {
            let mut arr: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.trim().parse().unwrap())
                .collect();
            arr.reverse();
            next1_value(arr)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 114);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 2);
    }
}
