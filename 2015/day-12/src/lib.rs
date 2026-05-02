use serde_json::Value;

fn sum_all_numbers(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n
            .as_i64()
            .or_else(|| n.as_u64().map(|u| u as i64))
            .unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
        Value::Array(items) => items.iter().map(sum_all_numbers).sum(),
        Value::Object(map) => map.values().map(sum_all_numbers).sum(),
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
    }
}

pub fn process_part1(input: &str) -> String {
    let root: Value = serde_json::from_str(input.trim()).expect("valid JSON");
    sum_all_numbers(&root).to_string()
}

fn sum_all_numbers_skip_red_objects(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n
            .as_i64()
            .or_else(|| n.as_u64().map(|u| u as i64))
            .unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
        Value::Array(items) => items.iter().map(sum_all_numbers_skip_red_objects).sum(),
        Value::Object(map) => {
            if map.values().any(|val| val.as_str() == Some("red")) {
                return 0;
            }
            map.values()
                .map(sum_all_numbers_skip_red_objects)
                .sum()
        }
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
    }
}

pub fn process_part2(input: &str) -> String {
    let root: Value = serde_json::from_str(input.trim()).expect("valid JSON");
    sum_all_numbers_skip_red_objects(&root).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(process_part1("[1,2,3]"), "6");
        assert_eq!(process_part1(r#"{"a":2,"b":4}"#), "6");
        assert_eq!(process_part1("[[[3]]]"), "3");
        assert_eq!(process_part1(r#"{"a":{"b":4},"c":-1}"#), "3");
        assert_eq!(process_part1(r#"{"a":[-1,1]}"#), "0");
        assert_eq!(process_part1(r#"[-1,{"a":1}]"#), "0");
        assert_eq!(process_part1("[]"), "0");
        assert_eq!(process_part1("{}"), "0");
    }

    #[test]
    fn part2_examples() {
        assert_eq!(process_part2("[1,2,3]"), "6");
        assert_eq!(process_part2(r#"[1,{"c":"red","b":2},3]"#), "4");
        assert_eq!(
            process_part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#),
            "0"
        );
        assert_eq!(process_part2(r#"[1,"red",5]"#), "6");
    }
}
