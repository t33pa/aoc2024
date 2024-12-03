use regex::Regex;
use std::env;
use std::fs;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let matches: Vec<String> = regex
        .find_iter(INPUT)
        .map(|mat| mat.as_str().to_string())
        .collect();

    if matches.is_empty() {
        println!("No matches found");
        std::process::exit(1);
    }

    let mut is_do = true;

    let result = matches.iter().fold(0, |acc, match_str| {
        if match_str == "do()" {
            is_do = true;
            acc
        } else if match_str == "don't()" {
            is_do = false;
            acc
        } else if is_do {
            let nums: Vec<&str> = match_str[4..match_str.len() - 1].split(',').collect();
            let a = nums[0].parse::<i32>().unwrap_or(0);
            let b = nums[1].parse::<i32>().unwrap_or(0);
            acc + a * b
        } else {
            acc
        }
    });

    println!("{}", result);
}

