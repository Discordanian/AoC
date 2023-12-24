#[derive(Debug)]
struct Rock {
    x: i128,
    y: i128,
    z: i128,
    xv: i128,
    yv: i128,
    zv: i128,
}

fn input_2_rock(input: &str) -> Vec<Rock> {
    input
        .lines()
        .map(|line| {
            /*   Learn how to really parse this stuff */
            let simple_line: String = line.chars().filter(|&c| c != ',').collect();

            let mut line_iter = simple_line.split_whitespace();
            let x = line_iter.next().unwrap().parse::<i128>().unwrap();
            let y = line_iter.next().unwrap().parse::<i128>().unwrap();
            let z = line_iter.next().unwrap().parse::<i128>().unwrap();
            let _ = line_iter.next();
            let xv = line_iter.next().unwrap().parse::<i128>().unwrap();
            let yv = line_iter.next().unwrap().parse::<i128>().expect("A number");
            let zv = line_iter.next().unwrap().parse::<i128>().unwrap();
            Rock {
                x,
                y,
                z,
                xv,
                yv,
                zv,
            }
        })
        .collect()
}

pub fn process_part1(input: &str) -> u64 {
    let rocks = input_2_rock(input);
    let rock_cnt = rocks.len();

    let range_low = 200000000000000;
    let range_high = 400000000000000;

    let mut ans = 0;

    for i in 0..rock_cnt {
        for j in (i + 1)..rock_cnt {
            let rocka_x = rocks[i].x;
            let rocka_x_delta = rocks[i].x + rocks[i].xv;
            let rockb_x = rocks[j].x;
            let rockb_x_delta = rocks[j].x + rocks[j].xv;

            let rocka_y = rocks[i].y;
            let rocka_y_delta = rocks[i].y + rocks[i].yv;
            let rockb_y = rocks[j].y;
            let rockb_y_delta = rocks[j].y + rocks[j].yv;

            let denom = (rocka_x - rocka_x_delta) * (rockb_y - rockb_y_delta)
                - (rocka_y - rocka_y_delta) * (rockb_x - rockb_x_delta);
            if denom != 0 {
                let px = ((rocka_x * rocka_y_delta - rocka_y * rocka_x_delta)
                    * (rockb_x - rockb_x_delta)
                    - (rocka_x - rocka_x_delta)
                        * (rockb_x * rockb_y_delta - rockb_y * rockb_x_delta))
                    / ((rocka_x - rocka_x_delta) * (rockb_y - rockb_y_delta)
                        - (rocka_y - rocka_y_delta) * (rockb_x - rockb_x_delta));
                let py = ((rocka_x * rocka_y_delta - rocka_y * rocka_x_delta)
                    * (rockb_y - rockb_y_delta)
                    - (rocka_y - rocka_y_delta)
                        * (rockb_x * rockb_y_delta - rockb_y * rockb_x_delta))
                    / ((rocka_x - rocka_x_delta) * (rockb_y - rockb_y_delta)
                        - (rocka_y - rocka_y_delta) * (rockb_x - rockb_x_delta));

                let test1 = (px > rocka_x) == (rocka_x_delta > rocka_x); // if px>rocka_x then rocka_x_delta has to be greater than rocka_x.
                                                                         // If not, both have to be false
                let test2 = (px > rockb_x) == (rockb_x_delta > rockb_x);

                let test3 = range_low <= px && px <= range_high;
                let test4 = range_low <= py && py <= range_high;

                if test1 && test2 && test3 && test4 {
                    ans += 1;
                }
            }
        }
    }

    ans
}

pub fn process_part2(input: &str) -> u64 {
    let mut ans = 0;
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, 15);
    }
}
