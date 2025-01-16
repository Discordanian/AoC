use regex::Regex;

// permute k elements of arr
// Heap's algorithm
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
pub fn permute(p: &mut Vec<Vec<u32>>, k: usize, arr: Vec<u32>) {
    let mut a: Vec<u32> = arr.clone();
    if k == 1 {
        p.push(arr.to_vec());
    } else {
        permute(p, k - 1, a.clone());
        let k_even = k % 2 == 0;

        for idx in 0..(k - 1) {
            if k_even {
                let t = a[idx];
                a[idx] = a[k - 1];
                a[k - 1] = t;
            } else {
                let t = a[0];
                a[0] = a[k - 1];
                a[k - 1] = t;
            }
            permute(p, k - 1, a.clone());
        }
    }
}

// Ignores everything else in the str slice and returns all positive integers found
pub fn parse_vec_u32(s: &str) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").expect("parse_vec_u32 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<u32>().unwrap());
    }

    retval
}

// Ignores everything else in the str slice and returns all integers found
pub fn parse_vec_i32(s: &str) -> Vec<i32> {
    let re = Regex::new(r"(-?\d+)").expect("parse_vec_i32 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<i32>().unwrap());
    }

    retval
}

// Ignores everything else in the str slice and returns all integers found
pub fn parse_vec_f32(s: &str) -> Vec<f32> {
    let re = Regex::new(r"((?:[+-]?\d+(?:\.\d*)?|\d\d+))").expect("parse_vec_f32 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<f32>().unwrap());
    }

    retval
}

#[derive(Ord, PartialOrd, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IPoint {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FPoint {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl IPoint {
    pub fn scale(self, z: i32) -> Self {
        Self {
            x: self.x * z,
            y: self.y * z,
        }
    }
}

impl std::ops::Add for IPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for IPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[allow(dead_code)]
impl FPoint {
    pub fn scale(self, z: f64) -> Self {
        Self {
            x: self.x * z,
            y: self.y * z,
        }
    }
}

impl std::ops::Add for FPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for FPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub fn input_to_map(input: &str) -> std::collections::BTreeMap<IPoint, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, val)| {
                (
                    IPoint {
                        x: x as i32,
                        y: y as i32,
                    },
                    val,
                )
            }) //move required for y value in closure
        })
        .collect()
}

pub fn input_to_vecs(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unsigned() {
        const INPUT: &str = "ass4dgj^23     45  -23 412349kurt 098 617NBompartAve";
        assert_eq!(parse_vec_u32(INPUT), vec![4, 23, 45, 23, 412349, 98, 617]);
    }

    #[test]
    fn parse_signed() {
        const INPUT: &str = "ass4dgj^23     45  -23 412349kurt 098 617NBompartAve";
        assert_eq!(parse_vec_i32(INPUT), vec![4, 23, 45, -23, 412349, 98, 617]);
    }

    #[test]
    fn parse_floats() {
        const INPUT: &str = "ass4dgj^23.2H     45  -23 412349.123kurt 0.98 617NBompartAve";
        assert_eq!(
            parse_vec_f32(INPUT),
            vec![4.0, 23.2, 45.0, -23.0, 412349.123, 0.98, 617.0]
        );
    }

    const INPUT: &str = "AB
CD";

    #[test]
    fn btree_from_input() {
        let result = input_to_map(INPUT);
        let mut bt = std::collections::BTreeMap::new();
        bt.insert(IPoint { x: 0, y: 0 }, 'A');
        bt.insert(IPoint { x: 1, y: 0 }, 'B');
        bt.insert(IPoint { x: 0, y: 1 }, 'C');
        bt.insert(IPoint { x: 1, y: 1 }, 'D');
        assert_eq!(result, bt);
    }

    #[test]
    fn vec_vec_from_input() {
        let result = input_to_vecs(INPUT);
        let vofv = vec![vec!['A', 'B'], vec!['C', 'D']];
        assert_eq!(result, vofv);
    }

    #[test]
    fn ipoint_test_add() {
        let a = IPoint { x: 1, y: 2 };
        let b = IPoint { x: 3, y: 4 };
        assert_eq!(a + b, IPoint { x: 4, y: 6 });
    }

    #[test]
    fn ipoint_test_sub() {
        let a = IPoint { x: 1, y: 2 };
        let b = IPoint { x: 3, y: 4 };
        assert_eq!(a - b, IPoint { x: -2, y: -2 });
    }

    #[test]
    fn ipoint_test_scale() {
        let a = IPoint { x: 1, y: 2 };
        assert_eq!(a.scale(10), IPoint { x: 10, y: 20 });
    }

    #[test]
    fn fpoint_test_add() {
        let a = FPoint { x: 1.0, y: 2.0 };
        let b = FPoint { x: 3.0, y: 4.0 };
        assert_eq!(a + b, FPoint { x: 4.0, y: 6.0 });
    }

    #[test]
    fn fpoint_test_sub() {
        let a = FPoint { x: 1.0, y: 2.0 };
        let b = FPoint { x: 3.0, y: 4.0 };
        assert_eq!(a - b, FPoint { x: -2.0, y: -2.0 });
    }

    #[test]
    fn fpoint_test_scale() {
        let a = FPoint { x: 1.0, y: 2.0 };
        assert_eq!(a.scale(10.0), FPoint { x: 10.0, y: 20.0 });
    }
}
