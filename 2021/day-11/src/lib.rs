pub fn process_part1(input: &str) -> u32 {
    let mut grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut retval = 0;

    for _ in 0..100 {
        grid = flash_grid(grid);
        retval += grid
            .iter()
            .map(|row| row.iter().filter(|x| **x == 0).count() as u32)
            .sum::<u32>();
    }

    retval
}

pub fn flash_grid(g: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (-1, 0),
    ];

    let mut grid: Vec<Vec<i32>> = g
        .iter()
        .map(|row| row.iter().map(|x| x + 1).collect())
        .collect();

    let mut flashing = true;

    while flashing {
        flashing = false;
        for r in 0..g.len() {
            for c in 0..g[0].len() {
                if grid[r][c] > 9 {
                    flashing = true;
                    grid[r][c] = 0;
                    for (dr, dc) in &directions {
                        let newr = r as i32 + dr;
                        let newc = c as i32 + dc;
                        if newr < g.len() as i32
                            && newr >= 0
                            && newc < g[0].len() as i32
                            && newc >= 0
                            && grid[newr as usize][newc as usize] != 0
                        {
                            grid[newr as usize][newc as usize] += 1;
                        }
                    }
                }
            }
        }
    }

    grid
}

pub fn process_part2(input: &str) -> usize {
    let mut grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut retval = 0;

    loop {
        retval += 1;
        grid = flash_grid(grid);
        if (grid.len() * grid[0].len())
            == grid
                .iter()
                .map(|row| row.iter().filter(|x| **x == 0).count())
                .sum::<usize>()
        {
            return retval;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part1_example() {
        let result = process_part1(INPUT);
        assert_eq!(result, 1656);
    }

    #[test]
    fn part2_example() {
        let result = process_part2(INPUT);
        assert_eq!(result, 195);
    }
}
