use std::io::{self, BufRead};
use std::str::FromStr;

use crate::code::*;

mod code;
mod sign;

#[derive(Debug)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    pub fn score(&self) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError,
    InputError,
}

fn parse_line(line: &str) -> Result<(LeftCode, RightCode), Error> {
    let (left_str, right_str) = line.split_once(' ').unwrap();
    Ok((
        LeftCode::from_str(&left_str)?,
        RightCode::from_str(&right_str)?,
    ))
}

fn score_round(left: LeftCode, right: RightCode) -> i32 {
    let outcome = right.to_outcome();
    outcome.score() + left.to_sign().inverse_compete(outcome).score()
}

fn main() {
    let stdin = io::stdin();
    let mut total = 0;

    for result in stdin.lock().lines() {
        if let Ok(line) = result {
            if let Ok((left, right)) = parse_line(&line) {
                let score = score_round(left, right);
                total += score;
            }
        }
    }
    println!("{}", total);
}
