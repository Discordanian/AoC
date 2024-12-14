use regex::Regex;

pub fn wrapped_coordinates(
    pos: (usize, usize),
    v: (i32, i32),
    height: usize,
    width: usize,
) -> (usize, usize) {
    let x: i32 = (pos.0 as i32 + v.0).rem_euclid(width as i32);
    let y: i32 = (pos.1 as i32 + v.1).rem_euclid(height as i32);
    (x as usize, y as usize)
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

pub fn bot_from_line(line: &str) -> Bot {
    let numbers = parse_vec_i32(line);
    let pos_x = numbers[0].unsigned_abs() as usize;
    let pos_y = numbers[1].unsigned_abs() as usize;
    let vel_x = numbers[2];
    let vel_y = numbers[3];

    Bot {
        position: (pos_x, pos_y),
        velocity: (vel_x, vel_y),
    }
}

impl Bot {
    pub fn step(&mut self, height: usize, width: usize) {
        self.position = wrapped_coordinates(self.position, self.velocity, height, width);
    }
}

#[derive(Debug)]
pub struct Bot {
    position: (usize, usize),
    velocity: (i32, i32),
}

pub fn process_part1(input: &str, height: usize, width: usize) -> u32 {
    let mut bots: Vec<Bot> = input.lines().map(bot_from_line).collect();

    let mid_x: usize = (width - 1) / 2;
    let mid_y: usize = (height - 1) / 2;

    dbg!((mid_x, mid_y));

    for _ in 0..100 {
        for bot in bots.iter_mut() {
            bot.step(height, width);
        }
    }

    let mut up_left = 0;
    let mut up_right = 0;
    let mut down_left = 0;
    let mut down_right = 0;
    for bot in bots.iter() {
        let left = bot.position.0 < mid_x;
        let right = bot.position.0 > mid_x;
        let up = bot.position.1 < mid_y;
        let down = bot.position.1 > mid_y;

        if left && up {
            up_left += 1;
        }
        if left && down {
            down_left += 1;
        }
        if right && up {
            up_right += 1;
        }
        if right && down {
            down_right += 1;
        }
    }

    up_left * up_right * down_left * down_right
}

pub fn process_part2(input: &str, height: usize, width: usize) -> u32 {
    let mut bots: Vec<Bot> = input.lines().map(bot_from_line).collect();

    let mid_x: usize = (width - 1) / 2;
    let mid_y: usize = (height - 1) / 2;

    dbg!((mid_x, mid_y));

    for _ in 0..100 {
        for bot in bots.iter_mut() {
            bot.step(height, width);
        }
    }

    let mut up_left = 0;
    let mut up_right = 0;
    let mut down_left = 0;
    let mut down_right = 0;
    for bot in bots.iter() {
        let left = bot.position.0 < mid_x;
        let right = bot.position.0 > mid_x;
        let up = bot.position.1 < mid_y;
        let down = bot.position.1 > mid_y;

        if left && up {
            up_left += 1;
        }
        if left && down {
            down_left += 1;
        }
        if right && up {
            up_right += 1;
        }
        if right && down {
            down_right += 1;
        }
    }

    up_left * up_right * down_left * down_right
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    const HEIGHT: usize = 7;
    const WIDTH: usize = 11;

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT, HEIGHT, WIDTH);
        assert_eq!(result, 12);
    }

    #[test]
    fn wrapped_movement() {
        let result = wrapped_coordinates((0, 0), (-1, -2), 5, 5);
        assert_eq!(result, (4, 3));
    }
}
