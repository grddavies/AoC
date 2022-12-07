use clap::Parser;

use std::collections::HashSet;
use std::hash::Hash;
use std::io::Error;
use std::path::PathBuf;
use std::{fs, io};

fn unique_chars<T>(arr: &[T]) -> bool
where
    T: Hash + Eq + Clone,
{
    let uniques: HashSet<T> = HashSet::from_iter(arr.iter().cloned());
    uniques.len() == arr.len()
}

fn get_pos_of_first_n_distinct_chars(input: &String, n: usize) -> Option<usize> {
    let chars = input.chars();
    let mut last: Vec<char> = chars.to_owned().take(n).collect();
    for (pos, ch) in chars.enumerate() {
        if unique_chars(&last) {
            return Some(pos);
        }
        last.rotate_left(1);
        last[0] = ch;
    }
    None
}

#[derive(Parser)]
#[command(name = "AoC-Day06")]
#[command(about = "Finds index of first occurrance of a block of size n of distinct characters", long_about = None)]
struct Cli {
    /// Number of distinct characters to look for
    #[arg(value_parser = clap::value_parser!(u32).range(2..=26))]
    n: u32,

    #[arg(short, long)]
    /// Optional file to read input from. Reads from stdin by default
    file: Option<PathBuf>,
}

fn get_input(file: Option<PathBuf>) -> Result<String, Error> {
    if let Some(file) = file {
        fs::read_to_string(file)
    } else {
        let stdin = io::stdin();
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => Ok(input),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let n = cli.n as usize;

    match get_input(cli.file) {
        Ok(input) => match get_pos_of_first_n_distinct_chars(&input, n) {
            Some(pos) => println!("{pos}"),
            None => {
                println!("[ERROR]: Could not find block of {n} distinct characters in input")
            }
        },
        Err(e) => println!("[ERROR]: {e}"),
    }
}
