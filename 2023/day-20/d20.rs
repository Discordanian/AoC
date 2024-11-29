use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::process;

// A helper function to calculate the least common multiple (LCM)
fn lcm(numbers: &[usize]) -> usize {
    numbers.iter().fold(1, |acc, &x| (acc * x) / gcd(acc, x))
}

// Helper function to calculate the greatest common divisor (GCD)
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn adjust(y: &str, typ_map: &HashMap<String, char>) -> String {
    if let Some(&prefix) = typ_map.get(y) {
        format!("{}{}", prefix, y)
    } else {
        y.to_string()
    }
}

fn main() {
    // Read input from file specified in command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let data = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let lines: Vec<&str> = data.trim().split('\n').collect();

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut typ_map: HashMap<String, char> = HashMap::new();

    // Parse input and build rules and type mappings
    for line in &lines {
        let parts: Vec<&str> = line.split("->").collect();
        let src = parts[0].trim().to_string();
        let dest: Vec<String> = parts[1].trim().split(", ").map(|s| s.to_string()).collect();
        rules.insert(src.clone(), dest);

        let typ = src.chars().next().unwrap();
        typ_map.insert(src[1..].to_string(), typ);
    }

    // Adjust the destination items
    let mut adjusted_rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut from_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut inv_map: HashMap<String, Vec<String>> = HashMap::new();

    for (src, dests) in &rules {
        let adjusted_dests: Vec<String> = dests.iter().map(|dest| adjust(dest, &typ_map)).collect();
        adjusted_rules.insert(src.clone(), adjusted_dests.clone());

        for dest in adjusted_dests {
            if dest.starts_with('&') {
                from_map
                    .entry(dest.clone())
                    .or_insert_with(HashMap::new)
                    .insert(src.clone(), "lo".to_string());
            }
            inv_map
                .entry(dest)
                .or_insert_with(Vec::new)
                .push(src.clone());
        }
    }

    // Verify initial assumptions
    let rx_inv = inv_map.get("rx").expect("No entry for 'rx'");
    assert!(rx_inv.len() == 1);
    assert!(rx_inv[0].starts_with('&'));

    let watch = inv_map.get(&rx_inv[0]).expect("No watch list");

    let mut lo = 0;
    let mut hi = 0;
    let mut queue: VecDeque<(String, String, String)> = VecDeque::new();
    let mut on_set: HashSet<String> = HashSet::new();
    let mut prev_map: HashMap<String, usize> = HashMap::new();
    let mut count_map: HashMap<String, usize> = HashMap::new();
    let mut to_lcm: Vec<usize> = Vec::new();

    for t in 1.. {
        queue.push_back((
            "broadcaster".to_string(),
            "button".to_string(),
            "lo".to_string(),
        ));

        while let Some((x, from, typ)) = queue.pop_front() {
            if typ == "lo" {
                if let Some(prev_t) = prev_map.get(&x) {
                    if count_map.get(&x).unwrap_or(&0) == &2 && watch.contains(&x) {
                        to_lcm.push(t - prev_t);
                    }
                }
                prev_map.insert(x.clone(), t);
                *count_map.entry(x.clone()).or_insert(0) += 1;
            }

            if to_lcm.len() == watch.len() {
                println!("watch.len() equal : {}", lcm(&to_lcm));
                process::exit(0);
            }

            if x == "rx" && typ == "lo" {
                println!("rx and typ lo : {}", t + 1);
            }

            if typ == "lo" {
                lo += 1;
            } else {
                hi += 1;
            }

            if let Some(dests) = adjusted_rules.get(&x) {
                if x == "broadcaster" {
                    for dest in dests {
                        queue.push_back((dest.clone(), x.clone(), typ.clone()));
                    }
                } else if x.starts_with('%') {
                    if typ == "hi" {
                        continue;
                    }
                    if on_set.insert(x.clone()) {
                        let new_typ = "hi".to_string();
                        for dest in dests {
                            queue.push_back((dest.clone(), x.clone(), new_typ.clone()));
                        }
                    } else {
                        on_set.remove(&x);
                    }
                } else if x.starts_with('&') {
                    let from_map_entry = from_map.get_mut(&x).expect("Missing FROM entry");
                    from_map_entry.insert(from.clone(), typ.clone());
                    let new_typ = if from_map_entry.values().all(|v| v == "hi") {
                        "lo".to_string()
                    } else {
                        "hi".to_string()
                    };
                    for dest in dests {
                        queue.push_back((dest.clone(), x.clone(), new_typ.clone()));
                    }
                } else {
                    panic!("Invalid case: {}", x);
                }
            }
        }

        if t == 1000 {
            println!("t is 1000 : {}", lo * hi);
        }
    }
}
