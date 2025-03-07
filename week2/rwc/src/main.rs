use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()


/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut vec = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        vec.push(line_str);
    }
    Ok(vec)
}

/// Counts the number of lines, words, and characters in a vector,
/// printed to command line.
fn get_wc(lines: &Vec<String>) {
    let mut char_count = 0;
    let mut word_count = 0;
    for line in lines {
        char_count += line.len();
        word_count += line.split_whitespace().count();
    }
    
    println!("Number of lines: {}", lines.len());
    println!("Number of words: {}", word_count);
    println!("Number of chars: {}", char_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let lines = match read_file_lines(filename) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };
    get_wc(&lines);
}
