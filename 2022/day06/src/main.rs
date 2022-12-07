use clap::Parser;

use std::collections::HashSet;
use std::hash::Hash;
use std::io::Error;
use std::path::PathBuf;
use std::{fs, io};

fn items_unique<T: Hash + Eq>(iter: impl IntoIterator<Item = T>) -> bool {
    let mut seen = HashSet::new();
    iter.into_iter().all(move |x| seen.insert(x))
}

/// Find index position after the first block of n distinct items in an iterator
fn find_idx_after_n_distinct_items<T, I>(xs: I, n: usize) -> Option<usize>
where
    I: Iterator<Item = T> + Clone,
    T: Hash + Eq + Copy,
{
    let mut last: Vec<T> = xs.clone().take(n).collect();
    xs.enumerate()
        .skip(n)
        .skip_while(|(_, x)| {
            last.rotate_left(1);
            last[0] = *x;
            !items_unique(&last)
        })
        .next()
        .map(|(pos, _)| pos)
}

#[derive(Parser)]
#[command(name = "AoC-Day06")]
#[command(about = "Finds index after first block of size n of distinct characters in a string")]
struct Cli {
    /// Size of block to check for character uniqueness
    #[arg(value_parser = clap::value_parser!(u32).range(2..))]
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
            Ok(_) => Ok(input.trim_end().to_string()),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let n = cli.n as usize;

    match get_input(cli.file) {
        Ok(input) => {
            if input.len() <= n {
                println!(
                    "[ERROR]: Expected input length greater than block size. {} <= {}",
                    input.len(),
                    n
                );
                return;
            }
            match find_idx_after_n_distinct_items(input.chars(), n) {
                Some(pos) => println!("{}", pos + 1), // output 1-indexed
                None => {
                    println!("[WARN]: Could not find block of {n} distinct characters in input")
                }
            }
        }
        Err(e) => println!("[ERROR]: {e}"),
    }
}
