pub fn process_part1(input: &str) -> String {
    next_valid_password(input.trim())
}

pub fn process_part2(input: &str) -> String {
    next_valid_password(&process_part1(input))
}

fn next_valid_password(current: &str) -> String {
    assert_eq!(current.len(), 8, "password must be exactly 8 letters");
    let mut pwd: [u8; 8] = current.as_bytes().try_into().expect("8 bytes");

    increment(&mut pwd);
    strip_forbidden(&mut pwd);

    while !is_valid(&pwd) {
        increment(&mut pwd);
        strip_forbidden(&mut pwd);
    }

    String::from_utf8(pwd.to_vec()).expect("ascii")
}

/// Like counting: increment rightmost; `z` wraps to `a` with carry.
fn increment(pwd: &mut [u8; 8]) {
    let mut i = 7_i32;
    while i >= 0 {
        let ui = i as usize;
        if pwd[ui] < b'z' {
            pwd[ui] += 1;
            return;
        }
        pwd[ui] = b'a';
        i -= 1;
    }
}

/// After any change, if `i`, `o`, or `l` appears, bump that letter and reset the suffix to `a`
/// (next lexicographic candidate that avoids forbidden letters in those positions).
fn strip_forbidden(pwd: &mut [u8; 8]) {
    loop {
        let mut fixed = false;
        for idx in 0..8 {
            if matches!(pwd[idx], b'i' | b'o' | b'l') {
                pwd[idx] += 1;
                for j in idx + 1..8 {
                    pwd[j] = b'a';
                }
                fixed = true;
                break;
            }
        }
        if !fixed {
            break;
        }
    }
}

fn has_straight(pwd: &[u8; 8]) -> bool {
    pwd.windows(3).any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
}

fn no_forbidden(pwd: &[u8; 8]) -> bool {
    !pwd
        .iter()
        .any(|&c| c == b'i' || c == b'o' || c == b'l')
}

/// At least two non-overlapping identical pairs (e.g. `aa` … `bb`, or `aaaa` → two `aa` pairs).
fn has_two_pairs(pwd: &[u8; 8]) -> bool {
    for i in 0..7 {
        if pwd[i] != pwd[i + 1] {
            continue;
        }
        for j in (i + 2)..7 {
            if pwd[j] == pwd[j + 1] {
                return true;
            }
        }
    }
    false
}

fn is_valid(pwd: &[u8; 8]) -> bool {
    has_straight(pwd) && no_forbidden(pwd) && has_two_pairs(pwd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_after_abcdefgh() {
        assert_eq!(process_part1("abcdefgh"), "abcdffaa");
    }

    #[test]
    fn example_after_ghijklmn() {
        assert_eq!(process_part1("ghijklmn"), "ghjaabcc");
    }

    #[test]
    fn part2_is_next_after_part1_abcdefgh() {
        assert_eq!(process_part2("abcdefgh"), "abcdffbb");
    }

    #[test]
    fn part2_is_next_after_part1_ghijklmn() {
        assert_eq!(process_part2("ghijklmn"), "ghjbbcdd");
    }
}
