use std::env;
use std::fs::{self, File};
use std::io;

fn read_file(file_path: &str) -> io::Result<String> {
    let _file = File::open(file_path)?;
    fs::read_to_string(file_path)
}

fn is_safe_report(numbers: Vec<i32>) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let is_increasing = numbers[0] < numbers[1];

    for i in 0..numbers.len() - 1 {
        let diff = numbers[i] - numbers[i + 1];
        if diff.abs() > 3 || diff == 0 {
            return false;
        }
        if is_increasing && diff > 0 {
            return false;
        }
        if !is_increasing && diff < 0 {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("File path is required");
        return;
    }

    let file_content = read_file(&args[1]).unwrap();
    let lines: Vec<&str> = file_content
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .collect();

    let safe_reports = lines
        .iter()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            is_safe_report(numbers)
        })
        .filter(|&is_safe| is_safe) 
        .count(); 

    println!("Safe reports: {}", safe_reports);
}
