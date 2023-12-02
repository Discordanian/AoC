use std::io;

pub fn rgb_from_str(line: &str) -> (u32, u32, u32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut last_num = 0;
    let parts: Vec<&str> =  line.split(':').collect();
    let ball_part: &str = parts[1];
    for part in ball_part.split_whitespace() {
        match part {
            "blue;" => b += last_num,
            "blue," => b += last_num,
            "blue" => b += last_num,
            "red;" => r += last_num,
            "red," => r += last_num,
            "red" => r += last_num,
            "green;" => g += last_num,
            "green," => g += last_num,
            "green" => g += last_num,
            _ => last_num = part.parse().unwrap()
        }
    }
    (r,g,b)
}

pub fn process_part1(input: &str) -> String {
    let r = 12;
    let g = 13;
    let b = 14;
    for game in input.lines() {
        let tup = rgb_from_str(game);
        if r >= tup.0 && g >= tup.1 && b >= tup.2 {
            dbg!("Yes Game {:?} -> {:?}", game,tup);
        }
    }
    "1".to_string()
}

pub fn process_part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT : &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "8".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "15".to_string());
    }
}
