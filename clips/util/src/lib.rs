use regex::Regex;

// Ignores everything else in the str slice and returns all positive integers found
pub fn parse_vec_u32(s: &str) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").expect("parse_vec_u32 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<u32>().unwrap());
    }

    retval
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

// Ignores everything else in the str slice and returns all integers found
pub fn parse_vec_f32(s: &str) -> Vec<f32> {
    let re = Regex::new(r"((?:[+-]?\d+(?:\.\d*)?|\d\d+))").expect("parse_vec_f32 regex failure");
    let mut retval = vec![];

    for (_, [x]) in re.captures_iter(s).map(|c| c.extract()) {
        retval.push(x.parse::<f32>().unwrap());
    }

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unsigned() {
        const INPUT: &str = "ass4dgj^23     45  -23 412349kurt 098 617NBompartAve";
        assert_eq!(parse_vec_u32(INPUT), vec![4, 23, 45, 23, 412349, 98, 617]);
    }

    #[test]
    fn parse_signed() {
        const INPUT: &str = "ass4dgj^23     45  -23 412349kurt 098 617NBompartAve";
        assert_eq!(parse_vec_i32(INPUT), vec![4, 23, 45, -23, 412349, 98, 617]);
    }

    #[test]
    fn parse_floats() {
        const INPUT: &str = "ass4dgj^23.2H     45  -23 412349.123kurt 0.98 617NBompartAve";
        assert_eq!(
            parse_vec_f32(INPUT),
            vec![4.0, 23.2, 45.0, -23.0, 412349.123, 0.98, 617.0]
        );
    }
}
