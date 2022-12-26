fn main() {
    let input = include_bytes!("../input.txt");
    let rows: Vec<&[u8]> = input
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .collect();
    let mut max = 0;
    for (j, row) in rows[0..rows.len()].iter().enumerate() {
        if j == 0 || j == rows.len() - 1 {
            continue;
        }
        for (i, el) in row[0..row.len()].iter().enumerate() {
            if i == 0 || i == row.len() - 1 {
                continue;
            }
            // Score em up
            let score = lscore(&el, i, &row)
                * rscore(&el, i, &row)
                * uscore(&el, i, j, &rows)
                * dscore(&el, i, j, &rows);
            if score > max {
                max = score
            }
        }
    }
    println!("{}", max)
}

fn lscore(el: &u8, i: usize, row: &[u8]) -> usize {
    let mut idx = i;
    let mut score = 0;
    while let Some(i) = idx.checked_sub(1) {
        if &row[i] >= el {
            score += 1;
            break;
        }
        score += 1;
        idx -= 1;
    }
    score
}

fn rscore(el: &u8, i: usize, row: &[u8]) -> usize {
    let mut idx = i;
    let mut score = 0;
    while let Some(x) = row.get(idx + 1) {
        if x >= el {
            score += 1;
            break;
        }
        score += 1;
        idx += 1;
    }
    score
}

fn uscore(el: &u8, i: usize, j: usize, rows: &Vec<&[u8]>) -> usize {
    let mut idx = j;
    let mut score = 0;
    while let Some(j) = idx.checked_sub(1) {
        if &rows[j][i] >= el {
            score += 1;
            break;
        }
        score += 1;
        idx -= 1;
    }
    score
}

fn dscore(el: &u8, i: usize, j: usize, rows: &Vec<&[u8]>) -> usize {
    let mut idx = j;
    let mut score = 0;
    while let Some(xs) = rows.get(idx + 1) {
        if &xs[i] >= el {
            score += 1;
            break;
        }
        score += 1;
        idx += 1;
    }
    score
}
