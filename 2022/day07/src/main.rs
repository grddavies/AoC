use std::{collections::BTreeMap, path::PathBuf};

fn main() {
    let input = include_str!("../input.txt");
    let mut cwd = PathBuf::new();
    let mut filetree = BTreeMap::new();
    for (i, line) in input.split('\n').enumerate() {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("$") => match words.next() {
                Some("cd") => match words.next() {
                    Some("..") => {
                        cwd.pop();
                        continue;
                    }
                    Some(dirname) => cwd.push(dirname),
                    None => panic!("Expected dirname, found nothing --> line {}", i + 1),
                },
                Some("ls") => continue,
                Some(x) => panic!("Unexpected word: {x} --> line {}", i + 1),
                None => panic!("Expected command --> line {}", i + 1),
            },
            Some("dir") => match words.next() {
                Some(dir) => {
                    filetree.insert(cwd.join(dir).to_path_buf(), 0);
                }
                None => panic!("Expected dirname, found nothing --> line {}", i + 1),
            },
            Some(number) => match number.parse::<i32>() {
                Ok(size) => {
                    filetree
                        .entry(cwd.clone())
                        .and_modify(|used| *used += size)
                        .or_insert(size);
                }
                Err(e) => panic!("Failed to parse filesize at line {}: {e}", i + 1),
            },
            None => {}
        }
    }
    // Copy filetree to appease borrow checker :(
    let mut nested = filetree.clone();
    for path in filetree.keys().rev().collect::<Vec<&PathBuf>>() {
        path.ancestors().skip(1).for_each(|p| {
            nested
                .entry(p.to_path_buf())
                .and_modify(|x| *x += filetree[path]);
        });
    }

    let total: i32 = nested.values().filter(|x| **x <= 100000).sum();
    println!("{}", total)
}
