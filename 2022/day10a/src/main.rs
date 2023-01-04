use std::str::FromStr;

fn main() {
    let mut cpu = CPU { cycle: 0, x_reg: 1 };
    let input = include_str!("../input.txt");
    let program = input.lines().filter_map(|l| l.parse::<Instruction>().ok());
    let mut i = 0;
    let mut values = [0; 6];

    for inst in program {
        for _ in 0..inst.cycles() {
            cpu.cycle += 1;
            if (cpu.cycle as i32 - 20) % 40 == 0 {
                values[i] = cpu.signal();
                i += 1;
            }
        }
        match inst {
            Instruction::AddX(v) => cpu.x_reg += v,
            _ => {}
        }
        if cpu.cycle > 220 {
            break
        }
    }
    println!("{}", values.iter().sum::<i32>());
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match &self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splt = s.split_whitespace();
        match splt.next() {
            Some("noop") => Ok(Instruction::Noop),
            Some("addx") => match splt.next() {
                Some(num) => Ok(Instruction::AddX(num.parse().unwrap())),
                None => Err("Missing value for addx instruction".to_string()),
            },
            Some(&_) | None => Err(format!("Could not parse instruction from '{}'", s)),
        }
    }
}

struct CPU {
    cycle: usize,
    x_reg: i32,
}

impl CPU {
    fn signal(&self) -> i32 {
        self.cycle as i32 * self.x_reg
    }
}
