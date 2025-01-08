pub fn fuel_required(mass: i32) -> i32 {
    (mass / 3) - 2
}

pub fn fuel_required_recursive(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + fuel_required_recursive(fuel)
    }
}

pub fn process_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|mass| fuel_required(mass.parse::<i32>().expect("Able to parse to i32")))
        .sum()
}

pub fn process_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|mass| fuel_required_recursive(mass.parse::<i32>().expect("Able to parse to i32")))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1a_works() {
        let result = process_part1("12");
        assert_eq!(result, 2);
    }

    #[test]
    fn part1b_works() {
        let result = process_part1("14");
        assert_eq!(result, 2);
    }

    #[test]
    fn part1c_works() {
        let result = process_part1("1969");
        assert_eq!(result, 654);
    }

    #[test]
    fn part1d_works() {
        let result = process_part1("100756");
        assert_eq!(result, 33583);
    }

    #[test]
    fn part2a_works() {
        let result = process_part2("14");
        assert_eq!(result, 2);
    }

    #[test]
    fn part2b_works() {
        let result = process_part2("1969");
        assert_eq!(result, 966);
    }

    #[test]
    fn part2c_works() {
        let result = process_part2("100756");
        assert_eq!(result, 50346);
    }
}
