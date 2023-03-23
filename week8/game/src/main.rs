use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    let reader = io::BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;
    let mut characters = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        lines += 1;
        words += line.split_whitespace().count();
        characters += line.chars().count();
    }

    println!("Lines: {}", lines);
    println!("Words: {}", words);
    println!("Characters: {}", characters);
}