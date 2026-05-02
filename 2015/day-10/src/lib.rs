pub fn process_part1(input: &str) -> String {
    length_after_look_and_say_rounds(input, 40)
}

pub fn process_part2(input: &str) -> String {
    length_after_look_and_say_rounds(input, 50)
}

fn length_after_look_and_say_rounds(input: &str, rounds: usize) -> String {
    let mut s = input.trim().to_string();
    for _ in 0..rounds {
        s = look_and_say(&s);
    }
    s.len().to_string()
}

fn look_and_say(s: &str) -> String {
    let bytes: &[u8] = s.as_bytes();
    let mut out = String::with_capacity(s.len() * 2);
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let mut count = 1usize;
        i += 1;
        while i < bytes.len() && bytes[i] == b {
            count += 1;
            i += 1;
        }
        out.push_str(&count.to_string());
        out.push(b as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_chain() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    #[test]
    fn five_steps_from_one() {
        let mut s = "1".to_string();
        for _ in 0..5 {
            s = look_and_say(&s);
        }
        assert_eq!(s, "312211");
    }
}
