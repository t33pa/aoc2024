use std::env;
use std::fs::{self, File};
use std::io;

fn read_file(file_path: &str) -> io::Result<String> {
    let _file = File::open(file_path)?;
    fs::read_to_string(file_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("File path is required");
        return;
    }

    let file_content = read_file(&args[1]).unwrap();
    let numbers: Vec<&str> = file_content.split_whitespace().collect();

    let (mut first_column, mut second_column): (Vec<i32>, Vec<i32>) = numbers
        .iter()
        .enumerate()
        .fold((Vec::new(), Vec::new()), |mut acc, (idx, num)| {
            if idx % 2 == 0 {
                acc.0.push(num.parse().unwrap());
            } else {
                acc.1.push(num.parse().unwrap());
            }
            acc
        });

    first_column.sort();
    second_column.sort();

    let sum: i32 = first_column
        .iter()
        .zip(second_column.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("{}", sum);
}
