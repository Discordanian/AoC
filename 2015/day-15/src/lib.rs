#[derive(Clone, Copy)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_line(line: &str) -> Ingredient {
    let (_, props) = line.split_once(": ").expect("line must contain ': '");
    let mut nums = Vec::new();
    for tok in props.split(|c: char| c == ',' || c.is_whitespace()) {
        let t = tok.trim();
        if t.is_empty() {
            continue;
        }
        if let Ok(n) = t.parse::<i32>() {
            nums.push(n);
        }
    }
    assert_eq!(nums.len(), 5, "expected 5 integers per ingredient line");
    Ingredient {
        capacity: nums[0],
        durability: nums[1],
        flavor: nums[2],
        texture: nums[3],
        calories: nums[4],
    }
}

fn cookie_score(amounts: &[i32], ingredients: &[Ingredient]) -> i64 {
    let mut cap = 0i64;
    let mut dur = 0i64;
    let mut flav = 0i64;
    let mut tex = 0i64;
    for (&amt, ing) in amounts.iter().zip(ingredients) {
        let a = amt as i64;
        cap += a * ing.capacity as i64;
        dur += a * ing.durability as i64;
        flav += a * ing.flavor as i64;
        tex += a * ing.texture as i64;
    }
    cap = cap.max(0);
    dur = dur.max(0);
    flav = flav.max(0);
    tex = tex.max(0);
    cap * dur * flav * tex
}

fn total_calories(amounts: &[i32], ingredients: &[Ingredient]) -> i64 {
    amounts
        .iter()
        .zip(ingredients)
        .map(|(&a, ing)| a as i64 * ing.calories as i64)
        .sum()
}

fn max_cookie_score(ingredients: &[Ingredient], teaspoons: i32, calorie_target: Option<i64>) -> i64 {
    let n = ingredients.len();
    assert!(n >= 1);
    let mut amounts = vec![0i32; n];
    let mut best = 0i64;

    fn dfs(
        ingredients: &[Ingredient],
        amounts: &mut [i32],
        idx: usize,
        remaining: i32,
        calorie_target: Option<i64>,
        best: &mut i64,
    ) {
        if idx == ingredients.len() - 1 {
            amounts[idx] = remaining;
            if let Some(target) = calorie_target {
                if total_calories(amounts, ingredients) != target {
                    return;
                }
            }
            *best = (*best).max(cookie_score(amounts, ingredients));
            return;
        }
        for x in 0..=remaining {
            amounts[idx] = x;
            dfs(
                ingredients,
                amounts,
                idx + 1,
                remaining - x,
                calorie_target,
                best,
            );
        }
    }

    dfs(
        ingredients,
        &mut amounts,
        0,
        teaspoons,
        calorie_target,
        &mut best,
    );
    best
}

pub fn process_part1(input: &str) -> String {
    let ingredients: Vec<Ingredient> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect();

    max_cookie_score(&ingredients, 100, None).to_string()
}

pub fn process_part2(input: &str) -> String {
    let ingredients: Vec<Ingredient> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect();

    max_cookie_score(&ingredients, 100, Some(500)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_ingredients() {
        let input = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";
        let ingredients: Vec<Ingredient> = input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(parse_line)
            .collect();
        assert_eq!(max_cookie_score(&ingredients, 100, None), 62842880);
    }

    #[test]
    fn example_part2_two_ingredients_500_calories() {
        let input = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";
        let ingredients: Vec<Ingredient> = input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(parse_line)
            .collect();
        assert_eq!(max_cookie_score(&ingredients, 100, Some(500)), 57600000);
    }
}
