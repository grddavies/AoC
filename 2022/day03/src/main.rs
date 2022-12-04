use std::{
    collections::HashSet,
    convert::AsRef,
    io::{self, BufRead},
    vec::Vec,
};

fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => (c as u8 - 'a' as u8 + 1).into(),
        'A'..='Z' => (c as u8 - 'A' as u8 + 27).into(),
        _ => 0,
    }
}

fn find_common_char<T: AsRef<str>>(strings: &[T]) -> Option<char> {
    let charsets: Vec<HashSet<char>> = strings
        .iter()
        .map(|s| s.as_ref().chars().collect::<HashSet<char>>())
        .collect();
    let intersection = charsets.iter().skip(1).fold(charsets[0].clone(), |l, r| {
        l.intersection(r).cloned().collect()
    });
    intersection.into_iter().next()
}

fn main() {
    let stdin = io::stdin();

    let lines = stdin
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>();

    assert!(lines.len() % 3 == 0, "Input lines not a multiple of 3");
    let mut total = 0;
    lines
        .chunks_exact(3)
        .for_each(|chunk| match find_common_char(chunk) {
            Some(common_char) => total += get_priority(common_char),
            None => panic!("No common characters"),
        });
    println!("{}", total);
}
