use clap::Parser;
use std::{fs, iter, str::FromStr};

#[derive(Debug)]
struct MultiStack {
    stacks: Vec<Vec<char>>,
}

impl MultiStack {
    fn apply(&mut self, action: &Move, part_2: bool) {
        let src = &mut self.stacks[action.src];
        let removed: Vec<char> = src.drain(src.len() - action.qty..src.len()).collect();
        let dst = &mut self.stacks[action.dst];
        if part_2 {
            dst.extend(removed);
            return;
        }
        dst.extend(removed.iter().rev());
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
            Err(format!("Could not parse move from\n{s}"))
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

    // Solve part 2 of the problem
    #[arg(long)]
    part_2: bool,
}

fn main() {
    let cli = Cli::parse();
    match fs::read_to_string(cli.input) {
        Ok(input_string) => match solve(input_string, cli.part_2) {
            Ok(out) => println!("{out}"),
            Err(e) => println!("[ERROR]: {e}"),
        },
        Err(e) => println!("[ERROR]: {e}"),
    }
}

fn parse_input<T: AsRef<str>>(input_string: T) -> Result<(MultiStack, Vec<Move>), String> {
    if let Some((state, instructions)) = input_string.as_ref().split_once("\n\n") {
        match state.parse::<MultiStack>() {
            Ok(state) => {
                let moves: Vec<Move> = instructions
                    .lines()
                    // NOTE: fails silently if an instruction cannot be parsed
                    .filter_map(|line| line.parse::<Move>().ok())
                    .collect();
                Ok((state, moves))
            }
            Err(e) => Err(e),
        }
    } else {
        Err(format!("Bad input:\n{}", input_string.as_ref()))
    }
}

fn solve(input_string: String, part_2: bool) -> Result<String, String> {
    match parse_input(input_string) {
        Ok((mut state, moves)) => {
            moves.iter().for_each(|m| state.apply(m, part_2));
            let tops: String = state.stacks.iter().filter_map(|v| v.last()).collect();
            assert!(tops.len() == state.stacks.len());
            Ok(tops)
        }
        Err(e) => Err(e),
    }
}
