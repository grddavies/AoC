use std::{
    cmp,
    io::{stdin, BufRead},
    str::FromStr,
};

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn union(&self, other: &Self) -> Self {
        Range {
            start: (cmp::min(self.start, other.start)),
            end: cmp::max(self.end, other.end),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.union(other) == *self
    }
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((a, b)) => {
                let start = a.parse::<u32>();
                let end = b.parse::<u32>();
                if start.is_err() || end.is_err() {
                    return Err(format!("Invalid range values: {}", s));
                }
                Ok(Range {
                    start: start.unwrap(),
                    end: end.unwrap(),
                })
            }
            None => Err(format!("Invalid range format: '{}'", s)),
        }
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

fn main() {
    let stdin = stdin();
    let mut count = 0;
    stdin.lock().lines().for_each(|res| match res {
        Ok(line) => {
            let strings = line.split(',');
            let ranges: Vec<Range> = strings.filter_map(|s| s.parse::<Range>().ok()).collect();
            if let [a, b] = &ranges[..] {
                if a.contains(b) || b.contains(a) {
                    count += 1;
                }
            }
        }
        Err(e) => panic!("Could not read line: {}", e),
    });
    println!("{}", count);
}
