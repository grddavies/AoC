use std::hash::Hash;
use std::{collections::HashSet, io};

fn unique_chars<T>(arr: &[T]) -> bool
where
    T: Hash + Eq + Clone,
{
    let uniques: HashSet<T> = HashSet::from_iter(arr.iter().cloned());
    uniques.len() == arr.len()
}

fn get_pos_of_first_four_distinct_chars(input: &String) -> Option<usize> {
    let chars = input.chars();
    let mut last = [' '; 4];
    chars
        .to_owned()
        .take(4)
        .enumerate()
        .for_each(|(i, c)| last[i] = c);
    for (pos, ch) in chars.enumerate() {
        if unique_chars(&last) {
            return Some(pos);
        }
        last.rotate_left(1);
        last[0] = ch;
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    match stdin.read_line(&mut buffer) {
        Ok(_) => match get_pos_of_first_four_distinct_chars(&buffer) {
            Some(pos) => println!("{pos}"),
            None => println!("[ERROR]: No start marker found"),
        },
        Err(e) => println!("[ERROR]: {e}"),
    }
}
