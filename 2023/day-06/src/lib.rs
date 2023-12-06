#[derive(Clone, Debug)]
struct Race {
    time: u32,
    distance: u32,
}

/*
impl From<&str> for Conversion {
    fn from(line: &str) -> Self {

        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Conversion {
            dest_start: parts[0],
            source_start: parts[1],
            length: parts[2],
        }
    }
}

*/
impl Race {
    fn wins(&self) -> u32 {
        (1..self.time)
            .filter(|x| self.expected(*x) > self.distance)
            .count() as u32
    }
    fn expected(&self, x: u32) -> u32 {
        // note that this forms a parabola facing down with an apex at time/2
        x * (self.time - x)
    }
}

pub fn process_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let time: u64 = lines[0][5..] // Get rid of "Time:"
        .chars()
        .filter(|x| *x != ' ')
        .fold(0_u64, |mut acc, x| {
            acc *= 10;
            acc += x.to_digit(10).expect("Not a number?") as u64;
            acc
        });
    let distance: u64 = lines[1][9..] // Get rid of "Distance:"
        .chars()
        .filter(|x| *x != ' ')
        .fold(0_u64, |mut acc, x| {
            acc *= 10;
            acc += x.to_digit(10).expect("Not a number?") as u64;
            acc
        });
    (1..time)
        .filter(|x| x * (time - x) > distance)
        .count()
        .to_string()
}

pub fn process_part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<u32> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let distances: Vec<u32> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let mut races: Vec<Race> = vec![];
    for x in 0..times.len() {
        races.push(Race {
            time: times[x],
            distance: distances[x],
        });
    }
    races
        .iter()
        .map(|race| race.wins())
        .product::<u32>()
        .to_string()
}




#[cfg(test)]
mod tests {
    use super::*;

    const INPUT : &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "288".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "71503".to_string());
    }
}
