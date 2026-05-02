use std::collections::{HashMap, HashSet};

fn distance_matrix(input: &str) -> Vec<Vec<u32>> {
    let mut cities: HashSet<String> = HashSet::new();
    let mut pairs: Vec<(String, String, u32)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (left, dist_str) = line.split_once(" = ").expect("line with ' = '");
        let dist: u32 = dist_str.parse().expect("distance");
        let (a, b) = left.split_once(" to ").expect("line with ' to '");
        cities.insert(a.to_string());
        cities.insert(b.to_string());
        pairs.push((a.to_string(), b.to_string(), dist));
    }

    let mut city_list: Vec<String> = cities.into_iter().collect();
    city_list.sort();
    let n = city_list.len();
    let idx: HashMap<String, usize> = city_list
        .iter()
        .enumerate()
        .map(|(i, s)| (s.clone(), i))
        .collect();

    let mut mat = vec![vec![0u32; n]; n];
    for (a, b, d) in pairs {
        let ia = idx[&a];
        let ib = idx[&b];
        mat[ia][ib] = d;
        mat[ib][ia] = d;
    }

    mat
}

pub fn process_part1(input: &str) -> String {
    let mat = distance_matrix(input);
    let mut best = u32::MAX;
    let mut used = vec![false; mat.len()];
    shortest_hamiltonian_path(&mat, &mut used, None, 0, 0, &mut best);
    best.to_string()
}

fn shortest_hamiltonian_path(
    mat: &[Vec<u32>],
    used: &mut [bool],
    last: Option<usize>,
    visited: usize,
    dist: u32,
    best: &mut u32,
) {
    let n = mat.len();
    if visited == n {
        *best = (*best).min(dist);
        return;
    }
    for next in 0..n {
        if used[next] {
            continue;
        }
        let add = match last {
            None => 0,
            Some(i) => mat[i][next],
        };
        let new_dist = dist + add;
        if new_dist >= *best {
            continue;
        }
        used[next] = true;
        shortest_hamiltonian_path(mat, used, Some(next), visited + 1, new_dist, best);
        used[next] = false;
    }
}

fn longest_hamiltonian_path(
    mat: &[Vec<u32>],
    used: &mut [bool],
    last: Option<usize>,
    visited: usize,
    dist: u32,
    best: &mut u32,
) {
    let n: usize = mat.len();
    if visited == n {
        *best = (*best).max(dist);
        return;
    }
    for next in 0..n {
        if used[next] {
            continue;
        }
        let add = match last {
            None => 0,
            Some(i) => mat[i][next],
        };
        used[next] = true;
        longest_hamiltonian_path(mat, used, Some(next), visited + 1, dist + add, best);
        used[next] = false;
    }
}

pub fn process_part2(input: &str) -> String {
    let mat: Vec<Vec<u32>> = distance_matrix(input);
    let mut best: u32 = 0u32;
    let mut used: Vec<bool> = vec![false; mat.len()];
    longest_hamiltonian_path(&mat, &mut used, None, 0, 0, &mut best);
    best.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn part1_works() {
        let result: String = process_part1(EXAMPLE);
        assert_eq!(result, "605".to_string());
    }

    #[test]
    fn part2_works() {
        let result: String = process_part2(EXAMPLE);
        assert_eq!(result, "982".to_string());
    }
}
