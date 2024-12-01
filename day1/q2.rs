use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io;

fn read_file(file_path: &str) -> io::Result<String> {
    let _file = File::open(file_path)?;
    fs::read_to_string(file_path)
}

fn create_counter(numbers: Vec<i32>) -> HashMap<i32, i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    numbers.iter().for_each(|&x| {
        let count = counter.entry(x).or_insert(0);
        *count += 1;
    });
    counter
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("File path is required");
        return;
    }

    let file_path = &args[1];
    let file_content = read_file(file_path).unwrap();
    let numbers: Vec<&str> = file_content.split_whitespace().collect();

    let (first_column, second_column): (Vec<i32>, Vec<i32>) =
        numbers
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

    let counter = create_counter(second_column.clone());

    let similarity: i32 = first_column
        .iter()
        .zip(second_column.iter())
        .map(|(a, _b)| {
            if counter.contains_key(a) {
                counter.get(a).unwrap() * a
            } else {
                0
            }
        })
        .sum();

    println!("{}", similarity);
}
