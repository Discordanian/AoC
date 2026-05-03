/// Presents at house `n` equal ten times the sum of divisors of `n`
/// (Elf `d` visits iff `d | n`, delivering `10 * d` presents).
pub fn process_part1(input: &str) -> String {
    let target: u64 = input.trim().parse().expect("puzzle input should be a positive integer");
    let mut upper: usize = 50_000;
    loop {
        let sigma = sum_of_divisors_sieve(upper);
        if let Some(n) = (1..=upper).find(|&n| sigma[n] * 10 >= target) {
            return n.to_string();
        }
        upper *= 2;
    }
}

fn sum_of_divisors_sieve(limit: usize) -> Vec<u64> {
    let mut sigma = vec![0u64; limit + 1];
    for d in 1..=limit {
        let mut m = d;
        while m <= limit {
            sigma[m] += d as u64;
            m += d;
        }
    }
    sigma
}

/// Elf `e` delivers `11 * e` to houses `e, 2e, …, 50e` only.
/// House `h` gets `11 * e` from each divisor `e` of `h` with `h / e <= 50`.
pub fn process_part2(input: &str) -> String {
    let target: u64 = input.trim().parse().expect("puzzle input should be a positive integer");
    let mut upper: usize = 50_000;
    loop {
        let presents = presents_part2_sieve(upper);
        if let Some(n) = (1..=upper).find(|&n| presents[n] >= target) {
            return n.to_string();
        }
        upper *= 2;
    }
}

fn presents_part2_sieve(limit: usize) -> Vec<u64> {
    let mut p = vec![0u64; limit + 1];
    for e in 1..=limit {
        let add = (e as u64) * 11;
        for k in 1..=50 {
            let h = e * k;
            if h > limit {
                break;
            }
            p[h] += add;
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_houses() {
        let sigma = sum_of_divisors_sieve(9);
        let presents: Vec<u64> = (1..=9).map(|n| sigma[n] * 10).collect();
        assert_eq!(presents, vec![10, 30, 40, 70, 60, 120, 80, 150, 130]);
    }

    #[test]
    fn puzzle_input() {
        assert_eq!(process_part1("33100000"), "776160");
    }

    #[test]
    fn part2_house_4() {
        let p = presents_part2_sieve(10);
        // Elves 1, 2, 4 each within 50 stops; 11 * (1 + 2 + 4) = 77
        assert_eq!(p[4], 77);
    }

    #[test]
    fn puzzle_input_part2() {
        assert_eq!(process_part2("33100000"), "786240");
    }
}
