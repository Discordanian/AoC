const MULT: u64 = 252_533;
const MOD: u64 = 33_554_393;
const START: u64 = 20_151_125;

/// 1-based index along the diagonal fill order for `(row, col)`.
fn diagonal_index(row: u64, col: u64) -> u64 {
    let s = row + col;
    (s - 2) * (s - 1) / 2 + col
}

fn code_at_diagonal_index(index_1based: u64) -> u64 {
    let mut v = START;
    for _ in 1..index_1based {
        v = v.wrapping_mul(MULT) % MOD;
    }
    v
}

fn first_number_after(haystack: &str, key: &str) -> u64 {
    let i = haystack.find(key).expect("expected keyword in puzzle input") + key.len();
    let tail = &haystack[i..];
    let end = tail
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(tail.len());
    tail[..end].parse().expect("expected digits")
}

pub fn process_part1(input: &str) -> String {
    let row = first_number_after(input, "row ");
    let col = first_number_after(input, "column ");
    let idx = diagonal_index(row, col);
    code_at_diagonal_index(idx).to_string()
}

pub fn process_part2(input: &str) -> String {
    input.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_example_position_12_is_row4_col2() {
        assert_eq!(diagonal_index(4, 2), 12);
        assert_eq!(code_at_diagonal_index(12), 32_451_966);
    }

    #[test]
    fn manual_corner_values() {
        assert_eq!(code_at_diagonal_index(1), 20_151_125);
        assert_eq!(code_at_diagonal_index(diagonal_index(6, 6)), 27_995_004);
    }
}
