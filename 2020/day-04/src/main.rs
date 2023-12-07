use std::env;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve(infile: String) -> String {
    let lines = read_lines(infile.as_str());
    // dbg!(lines.clone());
    format!("{}", lines.len())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => println!("Pass in filename to solve"),
        _ => println!(
            "Solution for {} : {}",
            args[1].clone(),
            solve(args[1].clone())
        ),
    }
}
