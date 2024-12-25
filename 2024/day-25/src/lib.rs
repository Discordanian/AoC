use std::collections::HashSet;

pub fn section_to_set(input: &str) -> std::collections::HashSet<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, val)| *val == '#')
                .map(move |(x, _)| (x, y)) //move required for y value in closure
        })
        .collect()
}
pub fn process_part1(input: &str) -> usize {
    let mut locks: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut keys: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut retval = 0;

    for section in input.split("\n\n") {
        let set = section_to_set(section);
        if set.contains(&(0, 0)) {
            locks.push(set);
        } else {
            keys.push(set);
        }
    }
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.intersection(lock).count() == 0 {
                retval += 1;
            }
        }
    }

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 3);
    }
}
