use std::collections::HashMap;
use std::collections::HashSet;

pub fn parse_dir(dir: &str) -> (char, usize) {
    let d = dir
        .chars()
        .nth(0)
        .expect("A character representing direction");
    let distance: usize = dir
        .strip_prefix(&d.to_string())
        .expect("Able to strip prefix")
        .parse()
        .expect("A valid usize");

    (d, distance)
}

pub fn path_to_hashmap(path: &str) -> HashMap<(i32, i32), u32> {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    let dirs: Vec<(char, usize)> = path
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| parse_dir(x))
        .collect();
    let mut x = 0;
    let mut y = 0;
    let mut steps: u32 = 0;

    for dir in dirs {
        match dir.0 {
            'U' => {
                for _ in 0..dir.1 {
                    y += 1;
                    steps += 1;
                    let key = (x, y);
                    map.entry(key).or_insert(steps);
                }
            }
            'D' => {
                for _ in 0..dir.1 {
                    y -= 1;
                    steps += 1;
                    let key = (x, y);
                    map.entry(key).or_insert(steps);
                }
            }
            'R' => {
                for _ in 0..dir.1 {
                    x += 1;
                    steps += 1;
                    let key = (x, y);
                    map.entry(key).or_insert(steps);
                }
            }
            'L' => {
                for _ in 0..dir.1 {
                    x -= 1;
                    steps += 1;
                    let key = (x, y);
                    map.entry(key).or_insert(steps);
                }
            }
            _ => panic!("at the disco"),
        }
    }
    map
}

pub fn path_to_set(path: &str) -> HashSet<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let dirs: Vec<(char, usize)> = path
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| parse_dir(x))
        .collect();
    let mut x = 0;
    let mut y = 0;

    for dir in dirs {
        match dir.0 {
            'U' => {
                for _ in 0..dir.1 {
                    y += 1;
                    set.insert((x, y));
                }
            }
            'D' => {
                for _ in 0..dir.1 {
                    y -= 1;
                    set.insert((x, y));
                }
            }
            'R' => {
                for _ in 0..dir.1 {
                    x += 1;
                    set.insert((x, y));
                }
            }
            'L' => {
                for _ in 0..dir.1 {
                    x -= 1;
                    set.insert((x, y));
                }
            }
            _ => panic!("at the disco"),
        }
    }
    set
}
pub fn process_part1(input: &str) -> u32 {
    let sets: Vec<HashSet<(i32, i32)>> = input.lines().map(path_to_set).collect();
    let intersection: HashSet<(i32, i32)> = sets[0]
        .intersection(&sets[1])
        .copied()
        .collect::<HashSet<_>>();

    let mut retval = i32::MAX;
    for xy in intersection {
        if xy.0.abs() + xy.1.abs() < retval {
            retval = xy.0.abs() + xy.1.abs();
        }
    }

    retval as u32
}

pub fn process_part2(input: &str) -> u32 {
    let maps: Vec<HashMap<(i32, i32), u32>> = input.lines().map(path_to_hashmap).collect();
    let keys1: HashSet<(i32, i32)> = maps[0].keys().copied().collect::<_>();
    let keys2: HashSet<(i32, i32)> = maps[1].keys().copied().collect::<_>();
    let mut retval = u32::MAX;
    let intersection: HashSet<(i32, i32)> =
        keys1.intersection(&keys2).copied().collect::<HashSet<_>>();

    for key in intersection {
        let step1 = maps[0][&key];
        let step2 = maps[1][&key];
        if step1 + step2 < retval {
            retval = step1 + step2;
        }
    }

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = "R8,U5,L5,D3
U7,R6,D4,L4";

    const INPUT_SMALL1: &str = "R2,U2
U2,R2";

    const INPUT1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    const INPUT2: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn part1_small() {
        let result = process_part1(INPUT_SMALL);
        assert_eq!(result, 6);
    }

    #[test]
    fn part1a_works() {
        let result = process_part1(INPUT1);
        assert_eq!(result, 159);
    }

    #[test]
    fn part1b_works() {
        let result = process_part1(INPUT2);
        assert_eq!(result, 135);
    }

    #[test]
    fn part2_small() {
        let result = process_part2(INPUT_SMALL);
        assert_eq!(result, 30);
    }

    #[test]
    fn part2a_test() {
        let result = process_part2(INPUT1);
        assert_eq!(result, 610);
    }

    #[test]
    fn part2b_test() {
        let result = process_part2(INPUT2);
        assert_eq!(result, 410);
    }
}
