pub fn process_part1(input: &str) -> u32 {
    let risk_grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    total_cost(risk_grid)
}

pub fn total_cost(risk_grid: Vec<Vec<u32>>) -> u32 {
    let ymax = risk_grid.len() - 1;
    let xmax = risk_grid.len() - 1;

    let mut cost_grid: Vec<Vec<u32>> = Vec::new();
    for _ in 0..risk_grid.len() {
        let mut r: Vec<u32> = Vec::new();
        for _ in 0..risk_grid[0].len() {
            r.push(u32::MAX);
        }
        cost_grid.push(r);
    }

    assert_eq!(risk_grid.len(), cost_grid.len());
    assert_eq!(risk_grid[0].len(), cost_grid[0].len());

    // y , x, cost
    let mut stack: Vec<(usize, usize, u32)> = vec![(0, 0, 0)];

    while let Some(yxc) = stack.pop() {
        let y = yxc.0;
        let x = yxc.1;
        let cost = yxc.2;

        if cost < cost_grid[y][x] {
            cost_grid[y][x] = cost;
            if x > 0 {
                stack.push((y, x - 1, cost + risk_grid[y][x - 1]));
            }
            if y > 0 {
                stack.push((y - 1, x, cost + risk_grid[y - 1][x]));
            }
            if y < ymax {
                stack.push((y + 1, x, cost + risk_grid[y + 1][x]));
            }
            if x < xmax {
                stack.push((y, x + 1, cost + risk_grid[y][x + 1]));
            }
        }
    }

    cost_grid[ymax][xmax]
}

pub fn big_grid(grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut g: Vec<Vec<u32>> = Vec::new();

    for line in &grid {
        g.push(line.clone());
    }

    for i in 1..=4 {
        for line in &grid {
            g.push(
                line.iter()
                    .clone()
                    .map(|x| x + i)
                    .map(|x| match x > 9 {
                        true => x - 9,
                        _ => x,
                    })
                    .collect(),
            );
        }
    }

    for y in 0..g.len() {
        for i in 1..=4 {
            for x in 0..grid[0].len() {
                let mut val = g[y][x] + i;
                if val > 9 {
                    val -= 9;
                }
                g[y].push(val);
            }
        }
    }

    g
}

pub fn process_part2(input: &str) -> u32 {
    let risk_grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    // The risk_grid needs to be 5x bigger

    total_cost(big_grid(risk_grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part1_example() {
        let result = process_part1(INPUT);
        assert_eq!(result, 40);
    }

    #[test]
    fn part2_example() {
        let result = process_part2(INPUT);
        assert_eq!(result, 315);
    }
}
