use clap::Parser;
use std::{fs, iter, os::macos::raw::stat, str::FromStr};

#[derive(Debug)]
struct MultiStack {
    stacks: Vec<Vec<char>>,
}

impl MultiStack {
    fn apply(&mut self, action: Move) {
        let src = &mut self.stacks[action.src];
        let removed: Vec<char> = src.drain(src.len() - action.qty..src.len()).rev().collect();
        let dst = &mut self.stacks[action.dst];
        dst.extend(removed);
    }
}

impl FromStr for MultiStack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        // Parse number of columns
        if let Some(x) = &lines
            .next()
            .and_then(|s| s.split_whitespace().last().unwrap().parse::<usize>().ok())
        {
            // Initialize stacks
            let mut stacks: Vec<Vec<char>> = iter::repeat_with(|| vec![]).take(*x).collect();
            lines.for_each(|line| {
                let cols = line.chars().collect::<Vec<char>>();
                cols.chunks(4).enumerate().for_each(|(i, chunk)| {
                    let c = chunk[1];
                    if c != ' ' {
                        stacks[i].push(c);
                    }
                });
            });
            Ok(MultiStack { stacks })
        } else {
            Err(format!("[ERROR]: Bad input:\n{}", s))
        }
    }
}

#[derive(Debug)]
struct Move {
    src: usize,
    dst: usize,
    qty: usize,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() != 6 {
            return Err(format!("[ERROR]: Bad input {s:?}"));
        }
        let digits: Vec<usize> = words.iter().filter_map(|x| x.parse().ok()).collect();
        if digits.len() != 3 {
            return Err(format!("[ERROR]: Bad input {s:?}"));
        }
        Ok(Move {
            qty: digits[0],
            // Input strings use 1-indexed columns
            src: digits[1] - 1,
            dst: digits[2] - 1,
        })
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // File to read input from
    input: String,
}

fn main() {
    let cli = Cli::parse();
    match fs::read_to_string(cli.input) {
        Ok(input_string) => part_1(input_string),
        Err(e) => println!("[ERROR]: {}", e),
    }
}

fn part_1(input_string: String) {
    if let Some((state, instructions)) = input_string.split_once("\n\n") {
        match state.parse::<MultiStack>() {
            Ok(mut state) => {
                instructions
                    .lines()
                    .for_each(|line| match line.parse::<Move>() {
                        Ok(m) => {
                            state.apply(m);
                        }
                        Err(e) => println!("[ERROR]: Bad instruction: '{e}'"),
                    });
                let tops: String = state.stacks.iter().filter_map(|v| v.last()).collect();
                assert!(tops.len() == state.stacks.len());
                println!("{tops}");
            }
            Err(e) => println!("[ERROR]: Bad crate diagram:\n{e}"),
        }
    } else {
        println!("[ERROR]: Bad input string:\n{input_string}");
    }
}
