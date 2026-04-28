#[derive(Debug, Clone, Copy)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    action: Action,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_coords(coords: &str) -> (usize, usize) {
    let (x, y) = coords
        .split_once(',')
        .expect("coordinate should be in x,y format");
    (
        x.parse::<usize>().expect("x must be a number"),
        y.parse::<usize>().expect("y must be a number"),
    )
}

fn parse_instruction(line: &str) -> Instruction {
    let (action, remainder) = if let Some(rest) = line.strip_prefix("turn on ") {
        (Action::TurnOn, rest)
    } else if let Some(rest) = line.strip_prefix("turn off ") {
        (Action::TurnOff, rest)
    } else if let Some(rest) = line.strip_prefix("toggle ") {
        (Action::Toggle, rest)
    } else {
        panic!("unknown instruction: {line}");
    };

    let (start, end) = remainder
        .split_once(" through ")
        .expect("instruction should include `through`");

    Instruction {
        action,
        start: parse_coords(start),
        end: parse_coords(end),
    }
}

pub fn process_part1(input: &str) -> String {
    let mut lights = vec![false; 1_000_000];

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let instruction = parse_instruction(line);
        for y in instruction.start.1..=instruction.end.1 {
            for x in instruction.start.0..=instruction.end.0 {
                let index = y * 1000 + x;
                match instruction.action {
                    Action::TurnOn => lights[index] = true,
                    Action::TurnOff => lights[index] = false,
                    Action::Toggle => lights[index] = !lights[index],
                }
            }
        }
    }

    lights
        .iter()
        .filter(|&&is_lit| is_lit)
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut lights = vec![0u32; 1_000_000];

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let instruction = parse_instruction(line);
        for y in instruction.start.1..=instruction.end.1 {
            for x in instruction.start.0..=instruction.end.0 {
                let index = y * 1000 + x;
                match instruction.action {
                    Action::TurnOn => lights[index] += 1,
                    Action::TurnOff => lights[index] = lights[index].saturating_sub(1),
                    Action::Toggle => lights[index] += 2,
                }
            }
        }
    }

    lights
        .iter()
        .map(|&brightness| u64::from(brightness))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{process_part1, process_part2};

    #[test]
    fn all_lights_on() {
        let input = "turn on 0,0 through 999,999";
        assert_eq!(process_part1(input), "1000000");
    }

    #[test]
    fn toggle_first_row() {
        let input = "toggle 0,0 through 999,0";
        assert_eq!(process_part1(input), "1000");
    }

    #[test]
    fn worked_example_sequence() {
        let input = "\
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
        assert_eq!(process_part1(input), "998996");
    }

    #[test]
    fn part2_single_light_on() {
        let input = "turn on 0,0 through 0,0";
        assert_eq!(process_part2(input), "1");
    }

    #[test]
    fn part2_toggle_entire_grid() {
        let input = "toggle 0,0 through 999,999";
        assert_eq!(process_part2(input), "2000000");
    }

    #[test]
    fn part2_turn_off_does_not_go_negative() {
        let input = "turn off 0,0 through 0,0";
        assert_eq!(process_part2(input), "0");
    }
}
