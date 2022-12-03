use crate::Outcome;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl Sign {
    pub fn score(&self) -> i32 {
        match self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissors => 3,
        }
    }

    pub fn compete(&self, other: &Self) -> Outcome {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => Outcome::Draw,
            Some(Ordering::Greater) => Outcome::Win,
            Some(Ordering::Less) => Outcome::Loss,
            None => panic!(),
        }
    }

    pub fn inverse_compete(&self, outcome: Outcome) -> &Self {
        match outcome {
            Outcome::Draw => &self,
            Outcome::Win => match self {
                Sign::Rock => &Sign::Paper,
                Sign::Scissors => &Sign::Rock,
                Sign::Paper => &Sign::Scissors,
            },
            Outcome::Loss => match self {
                Sign::Rock => &Sign::Scissors,
                Sign::Scissors => &Sign::Paper,
                Sign::Paper => &Sign::Rock,
            },
        }
    }
}

impl PartialOrd for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Sign::Rock => match other {
                Sign::Paper => Some(Ordering::Less),
                Sign::Rock => Some(Ordering::Equal),
                Sign::Scissors => Some(Ordering::Greater),
            },
            Sign::Paper => match other {
                Sign::Scissors => Some(Ordering::Less),
                Sign::Paper => Some(Ordering::Equal),
                Sign::Rock => Some(Ordering::Greater),
            },
            Sign::Scissors => match other {
                Sign::Rock => Some(Ordering::Less),
                Sign::Scissors => Some(Ordering::Equal),
                Sign::Paper => Some(Ordering::Greater),
            },
        }
    }
}
