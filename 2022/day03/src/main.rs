use std::{
    io::{self, BufRead},
    vec::Vec,
};

fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => (c as u8 - 'a' as u8 + 1).into(),
        'A'..='Z' => (c as u8 - 'A' as u8 + 27).into(),
        _ => 0
    }
}

fn main() {
    let stdin = io::stdin();

    let lines = stdin
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>();

    let mut total = 0;
    lines.chunks(3).for_each(|thing| {
        match thing {
            [first, second, third] => {
                match first
                    .chars()
                    .find(|c| second.contains(*c) && third.contains(*c)) {
                        Some(common_char) => total += get_priority(common_char),
                        None => panic!("No common characters in these groups:\n{}\n{}\n{}", first, second, third )
                    }
            },
            _ => panic!("Didn't expect to get here! Are you feeding me lines in threes?")
        }            
    });
    println!("{}", total);
}
