fn parse_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn step(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut next = vec![vec![false; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let mut on_neighbors = 0u8;
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0
                        && nr < rows as i32
                        && nc >= 0
                        && nc < cols as i32
                        && grid[nr as usize][nc as usize]
                    {
                        on_neighbors += 1;
                    }
                }
            }
            let on = grid[r][c];
            next[r][c] = if on {
                on_neighbors == 2 || on_neighbors == 3
            } else {
                on_neighbors == 3
            };
        }
    }
    next
}

fn count_on(grid: &[Vec<bool>]) -> usize {
    grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell)
        .count()
}

fn force_stuck_corners(grid: &mut [Vec<bool>]) {
    if grid.is_empty() {
        return;
    }
    let rows = grid.len();
    let cols = grid[0].len();
    if cols == 0 {
        return;
    }
    let last_r = rows - 1;
    let last_c = cols - 1;
    grid[0][0] = true;
    grid[0][last_c] = true;
    grid[last_r][0] = true;
    grid[last_r][last_c] = true;
}

pub fn process_part1(input: &str) -> String {
    let mut grid = parse_grid(input);
    for _ in 0..100 {
        grid = step(&grid);
    }
    count_on(&grid).to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut grid = parse_grid(input);
    force_stuck_corners(&mut grid);
    for _ in 0..100 {
        grid = step(&grid);
        force_stuck_corners(&mut grid);
    }
    count_on(&grid).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn example_after_four_steps() {
        let mut grid = parse_grid(EXAMPLE);
        for _ in 0..4 {
            grid = step(&grid);
        }
        assert_eq!(count_on(&grid), 4);
    }

    #[test]
    fn example_part2_after_five_steps() {
        let mut grid = parse_grid(EXAMPLE);
        force_stuck_corners(&mut grid);
        for _ in 0..5 {
            grid = step(&grid);
            force_stuck_corners(&mut grid);
        }
        assert_eq!(count_on(&grid), 17);
    }
}
