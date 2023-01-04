use std::str::FromStr;

const SCREEN_WIDTH: usize = 40;

fn main() {
    let mut cpu = CPU { cycle: 0, x_reg: 1 };
    let input = include_str!("../input.txt");
    let program = input.lines().filter_map(|l| l.parse::<Instruction>().ok());

    let mut p_sprite: usize;
    let mut pixels = ['.'; 240];

    for inst in program {
        for _ in 0..inst.cycles() {
            cpu.cycle += 1;
            p_sprite = (cpu.x_reg + 1) as usize;
            if p_sprite.abs_diff(cpu.cycle % SCREEN_WIDTH) < 2 {
                pixels[cpu.cycle - 1] = '#';
            }
        }
        match inst {
            Instruction::AddX(v) => cpu.x_reg += v,
            _ => {}
        }
    }

    print_screen(pixels);
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

fn print_screen(pixels: [char; 240]) {
    pixels
        .chunks(SCREEN_WIDTH)
        .for_each(|c| println!("{}", c.iter().collect::<String>()))
}
