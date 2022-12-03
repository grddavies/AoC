use std::io::{self, BufRead};

fn get_priority(c: char) -> i32 {
    match c.is_uppercase() {
        true => c as i32 - 65 + 27,
        false => c as i32 - 97 + 1,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut total = 0;
    for res in stdin.lock().lines() {
        if let Ok(line) = res {
            let (left, right) = line.split_at(line.len() / 2);
            let mut subtotal = 0;
            for lc in left.chars() {
                if right.chars().any(|rc| rc == lc) {
                    subtotal += get_priority(lc);
                    break; // only one char will be repeated
                }
            }
            total += subtotal;
        }
    }
    println!("{}", total)
}
