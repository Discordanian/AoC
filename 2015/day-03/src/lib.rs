pub fn process_part1(input: &str) -> String {
    use std::collections::HashSet;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = (0, 0);
    visited.insert((x, y));

    for ch in input.chars() {
        match ch {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => continue,
        }
        visited.insert((x, y));
    }

    visited.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    use std::collections::HashSet;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    visited.insert((0, 0));

    for (idx, ch) in input.chars().enumerate() {
        let (x, y) = if idx % 2 == 0 {
            (&mut santa_x, &mut santa_y)
        } else {
            (&mut robo_x, &mut robo_y)
        };

        match ch {
            '^' => *y += 1,
            'v' => *y -= 1,
            '>' => *x += 1,
            '<' => *x -= 1,
            _ => continue,
        }

        visited.insert((*x, *y));
    }

    visited.len().to_string()
}