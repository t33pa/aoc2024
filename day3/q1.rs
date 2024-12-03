extern crate regex;
use std::env;
use std::fs;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches: Vec<(i32, i32)> = regex
        .captures_iter(INPUT)
        .filter_map(|cap| {
            let a = cap[1].parse::<i32>().ok()?;
            let b = cap[2].parse::<i32>().ok()?;
            Some((a, b))
        })
        .collect();

    if matches.is_empty() {
        println!("No matches found");
        std::process::exit(1);
    }

    let result: i32 = matches
        .iter()
        .map(|(a, b)| a * b)
        .sum();

    println!("{:?}", matches); 
    println!("{}", result);
}

