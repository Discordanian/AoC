pub fn process_part1(input: &str) -> String {
    let mut first = true;
    let mut sum = 0;
    let mut last_char = '0';
    for c in input.chars() {
        if c == '\n' {
            first = true;
            sum += last_char.to_digit(10).unwrap()
        }
        if c.is_digit(10) && !first {
            last_char = c;
        }
        if c.is_digit(10) && first {
            sum += c.to_digit(10).unwrap() * 10;
            last_char = c;
            first = false;
        }
    }
    format!("{}",sum)
}

pub fn process_part2(input: &str) -> String {
    let mut new_input : String = String::from(input);
    new_input = new_input.replace("zerone","0");
    new_input = new_input.replace("nineight","9");
    new_input = new_input.replace("sevenine","7");
    new_input = new_input.replace("fiveight","5");
    new_input = new_input.replace("threeight","3");
    new_input = new_input.replace("eightwo","8");
    new_input = new_input.replace("eighthree","8");
    new_input = new_input.replace("twone","2");
    new_input = new_input.replace("oneight","1");
    new_input = new_input.replace("one","1");
    new_input = new_input.replace("two","2");
    new_input = new_input.replace("three","3");
    new_input = new_input.replace("four","4");
    new_input = new_input.replace("five","5");
    new_input = new_input.replace("six","6");
    new_input = new_input.replace("seven","7");
    new_input = new_input.replace("eight","8");
    new_input = new_input.replace("nine","9");
    // new_input = new_input.replace("zero","0");
    println!("{}", new_input);

    let mut first = true;
    let mut sum = 0;
    let mut last_char = '0';
    let mut first_char = '0';
    for c in new_input.as_str().chars() {
        if c == '\n' {
            first = true;
            sum += last_char.to_digit(10).unwrap();
            println!("Newline, add character {}{}", first_char,last_char);
        }
        if c.is_digit(10) && !first {
            last_char = c;
        }
        if c.is_digit(10) && first {
            // println!("First digit add 10 * character {}", c);
            first_char = c;
            sum += c.to_digit(10).unwrap() * 10;
            last_char = c;
            first = false;
        }
    }
    format!("{}",sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1 : &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "142".to_string());
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "281".to_string());
    }
}
