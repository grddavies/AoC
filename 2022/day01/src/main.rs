use std::io::{self, BufRead};

fn insert_in_place<T>(arr: &mut [T], value: T, idx: usize) {
    arr[idx..].rotate_right(1);
    arr[idx] = value;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut maxes = [0, 0, 0];
    let mut agg = 0;
    for line in stdin.lock().lines() {
        match line.unwrap().as_str() {
            "" => {
                for i in 0..maxes.len() {
                    if &agg > &maxes[i] {
                        insert_in_place(&mut maxes, agg, i);
                        break
                    }
                }
                agg = 0
            }
            line => agg += line.parse::<i32>().unwrap(),
        }
    }
    println!("{}", maxes.iter().sum::<i32>());
    Ok(())
}
