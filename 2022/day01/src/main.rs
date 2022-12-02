use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut max = 0;
    let mut agg = 0;
    for line in stdin.lock().lines() {
        match line.unwrap().as_str() {
            "" => {
                if agg > max {
                    max = agg;
                }
                agg = 0
            }
            line => agg += line.parse::<i32>().unwrap(),
        }
    }
    println!("{}", max);
    Ok(())
}
