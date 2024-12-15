use std::collections::BTreeMap;
// I like BTree over Hash because it makes debugging easier
// If you want speed, use Hash instead

pub fn make_grid(input: &str) -> BTreeMap<(i32, i32), char> {
    let mut grid = BTreeMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid.insert((row as i32, col as i32), c);
        }
    }
    grid
    // input.lines().map(|line| line.chars().collect()).collect()
}

pub fn make_instructions(input: &str) -> Vec<char> {
    input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

pub fn bot_position(g: &BTreeMap<(i32, i32), char>) -> (i32, i32) {
    for key in g.keys() {
        if g.get(key) == Some(&'@') {
            return *key;
        }
    }
    (0, 0)
}

pub fn process_part1(input: &str) -> i32 {
    let (g, i) = input.split_once("\n\n").unwrap();

    let mut grid = make_grid(g);
    let instructions = make_instructions(i);
    let mut botpos = bot_position(&grid);

    for dir in instructions.iter() {
        let delta = match dir {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => panic!("At the disco"),
        };
        let mut targets = vec![];

        let mut look = true;
        let mut canmove = true;
        let mut next = botpos;

        while look {
            next = (next.0 + delta.0, next.1 + delta.1);
            match grid.get(&next) {
                Some(&'O') => targets.push(next),
                Some(&'#') => {
                    canmove = false;
                    look = false;
                    break;
                }
                Some(&'.') => {
                    targets.push(next);
                    look = false;
                }
                Some(x) => {
                    look = false;
                    dbg!(x);
                    panic!("What???");
                }
                None => panic!("None received moving along look"),
            }
        }
        if canmove {
            // move bot to first position
            // move crate from pos one to final position
            if let Some(firstpos) = targets.first() {
                // Vacate the bot position
                if let Some(val) = grid.get_mut(&botpos) {
                    *val = '.';
                }
                botpos = *firstpos;
                // Mark first pos with bot
                if let Some(val) = grid.get_mut(firstpos) {
                    *val = '@';
                }
            }
            // If targets > 1 we have crates to move.  Just put one at the end.
            if targets.len() > 1 {
                if let Some(lastpos) = targets.last() {
                    if let Some(val) = grid.get_mut(lastpos) {
                        *val = 'O';
                    }
                }
            }
        }

        /*
        targets = [(r, c)]
        cr = r
        cc = c
        go = True
        while True:
            cr += dr
            cc += dc
            char = grid[cr][cc]
            if char == "#":
                go = False
                break
            if char == "O":
                targets.append((cr, cc))
            if char == ".":
                break
        if not go: continue
        grid[r][c] = "."
        grid[r + dr][c + dc] = "@"
        for br, bc in targets[1:]:
            grid[br + dr][bc + dc] = "O"
        r += dr
        c += dc
            */
    }

    score_part1(grid)
}

pub fn score_part1(grid: BTreeMap<(i32, i32), char>) -> i32 {
    grid.iter()
        .map(|(pos, v)| match v {
            'O' => pos.0 * 100 + pos.1,
            _ => 0,
        })
        .sum::<i32>()
}

pub fn process_part2(input: &str) -> u32 {
    input.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 10092);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 9021);
    }
}
