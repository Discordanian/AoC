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

pub fn max_rgb_from_str(line: &str) -> (u32, u32, u32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut last_num = 0;
    let parts: Vec<&str> =  line.split(':').collect();
    let games: Vec<&str> = parts[1].split(';').collect();
    for game in games {
        for part in game.split_whitespace() {
            match part {
                "blue," => b = last_num.max(b),
                "blue" => b = last_num.max(b),
                "red," => r = last_num.max(r),
                "red" => r = last_num.max(r),
                "green," => g = last_num.max(g),
                "green" => g = last_num.max(g),
                _ => last_num = part.trim().parse().unwrap()
            }
        }
    }
    (r,g,b)
}

pub fn game_id(line: &str) -> u32 {
    let parts: Vec<&str> =  line.split(':').collect();
    let game_p: Vec<&str> = parts[0].split("Game ").collect();
    game_p[1].parse().unwrap()
}

pub fn process_part1(input: &str) -> String {
    println!("process_part1");
    let r = 12;
    let g = 13;
    let b = 14;
    let mut sum = 0;
    for game in input.lines() {
        let tup = max_rgb_from_str(game);
        if r >= tup.0 && g >= tup.1 && b >= tup.2 {
            let id = game_id(game);
            sum += id;
            println!("Yes Game {} {:?} -> {:?}",id, game,tup);
        } else {
            let id = game_id(game);
            println!("No Game {} {:?} -> {:?}",id, game,tup);
        }
    }
    format!("{}",sum)
}

pub fn process_part2(_input: &str) -> String {
    "1".to_string()
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
        assert_eq!(result, "1".to_string());
    }
}
