pub fn process_part1(input: &str) -> String {
    let mut diff: usize = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        diff += line.len() - memory_char_count(line);
    }
    diff.to_string()
}

/// Characters in the in-memory string value for one line (a quoted literal).
fn memory_char_count(line: &str) -> usize {
    let bytes = line.as_bytes();
    if bytes.len() < 2 || bytes[0] != b'"' || bytes[bytes.len() - 1] != b'"' {
        return 0;
    }
    let inner: &[u8] = &bytes[1..bytes.len() - 1];
    let mut i: usize = 0;
    let mut count: usize = 0;
    while i < inner.len() {
        if inner[i] != b'\\' {
            i += 1;
            count += 1;
        } else {
            i += 1;
            if i >= inner.len() {
                break;
            }
            match inner[i] {
                b'\\' | b'"' => {
                    i += 1;
                    count += 1;
                }
                b'x' => {
                    i += 3;
                    count += 1;
                }
                _ => {
                    i += 1;
                    count += 1;
                }
            }
        }
    }
    count
}

/// Part 2: encode each line’s literal as a new string (surrounding `"`, and `\` before every
/// `"` and `\` in the original), then return the sum of (encoded length − original line length).
/// Per line that increase is `2` (new outer quotes) plus one extra character for each `"` and `\`
/// in the source line.
pub fn process_part2(input: &str) -> String {
    let mut diff: usize = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let quote_or_backslash = line.chars().filter(|&c| c == '"' || c == '\\').count();
        diff += 2 + quote_or_backslash;
    }
    diff.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// AoC 2015 Day 8 example (four string literals).
    const EXAMPLE: &str = concat!("\"\"\n", "\"abc\"\n", "\"aaa\\\"aaa\"\n", "\"\\x27\"");

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(EXAMPLE), "12");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(EXAMPLE), "19");
    }
}
