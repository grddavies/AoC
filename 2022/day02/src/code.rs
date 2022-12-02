use std::str::FromStr;

use crate::{sign::{Sign}, Error};

#[derive(Debug)]
pub enum LeftCode {
    A,
    B,
    C,
}

impl LeftCode {
    pub fn to_sign(&self) -> Sign {
        match self {
            LeftCode::A => Sign::Rock,
            LeftCode::B => Sign::Paper,
            LeftCode::C => Sign::Scissors,
        }
    }
}

impl FromStr for LeftCode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(LeftCode::A),
            "B" => Ok(LeftCode::B),
            "C" => Ok(LeftCode::C),
            _ => Err(Error::ParseError),
        }
    }
}

#[derive(Debug)]
pub enum RightCode {
    X,
    Y,
    Z,
}

impl RightCode {
    pub fn to_sign(&self) -> Sign {
        match self {
            RightCode::X => Sign::Rock,
            RightCode::Y => Sign::Paper,
            RightCode::Z => Sign::Scissors,
        }
    }
}

impl FromStr for RightCode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RightCode::X),
            "Y" => Ok(RightCode::Y),
            "Z" => Ok(RightCode::Z),
            _ => Err(Error::ParseError),
        }
    }
}