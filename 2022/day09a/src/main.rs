use std::{collections::HashSet, str::FromStr, usize};

#[derive(Debug)]
struct Motion {
    d: usize,
    n: i16,
}

impl FromStr for Motion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((d, n)) = s.split_once(|c| c == ' ') {
            let n: i16 = n.parse().unwrap();
            match d {
                "R" => Ok(Motion { d: 0, n }),
                "L" => Ok(Motion { d: 0, n: -n }),
                "U" => Ok(Motion { d: 1, n }),
                "D" => Ok(Motion { d: 1, n: -n }),
                _ => Err(format!("Unexpected direction '{}'", d)),
            }
        } else {
            Err(format!("Invalid motion string '{}'", s))
        }
    }
}

fn move_tail(head: &[i16; 2], tail: &mut [i16; 2]) {
    let dx = head[0] - tail[0];
    let dy = head[1] - tail[1];
    if dx.pow(2) + dy.pow(2) > 2 {
        tail[0] += 1 * dx.signum();
        tail[1] += 1 * dy.signum();
    }
}

fn print_positions(positions: &HashSet<[i16; 2]>) {
    // Make grid
    let mut xmin = 0;
    let mut xmax = 0;
    let mut ymin = 0;
    let mut ymax = 0;
    positions.iter().for_each(|&[x, y]| {
        if x < xmin {
            xmin = x;
        } else if x > xmax {
            xmax = x;
        }

        if y < ymin {
            ymin = y;
        } else if y > ymax {
            ymax = y;
        }
    });
    let rows = vec!['-'; (1 + xmax - xmin) as usize];
    let mut cols = vec![rows.clone(); (1 + ymax - ymin) as usize];

    // Add positions
    for [x, y] in positions.iter() {
        let x = *x - xmin;
        let y = *y - ymin;
        cols[y as usize][x as usize] = '#';
    }

    // Add origin
    cols[(0 - ymin) as usize][(0 - xmin) as usize] = 's';

    // print
    for row in cols.iter().rev() {
        for c in row.iter() {
            print!("{c} ");
        }
        print!("\n")
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut hpos = [0, 0];
    let mut tpos = [0, 0];
    let mut positions = HashSet::new();

    input
        .split(|c| c == '\n')
        .filter(|x| !x.is_empty())
        .for_each(|line| match line.parse::<Motion>() {
            Ok(motion) => {
                for _ in 0..motion.n.abs() {
                    hpos[motion.d] += motion.n.signum();
                    move_tail(&hpos, &mut tpos);
                    positions.insert(tpos);
                }
            }
            Err(e) => panic!("Error: {}", e),
        });

    // print_positions(&positions);
    println!("{}", positions.len())
}
