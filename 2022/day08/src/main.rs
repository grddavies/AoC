fn main() {
    let input = include_bytes!("../input.txt");
    let rows: Vec<&[u8]> = input
        .split(|&x| x == b'\n')
        .filter(|l| !l.is_empty())
        .collect();

    let mut count = 0;

    for (j, row) in rows.iter().enumerate() {
        // count all of first and last row
        if j == 0 || j == (rows.len() - 1) {
            count += rows[j].len();
            continue;
        };

        for (i, x) in row.iter().enumerate() {
            // count all of first col & last col
            if i == 0 || i == (row.len() - 1) {
                count += 1;
                continue;
            }
            // seen from left
            if row[0..i].iter().all(|&el| el < *x) {
                count += 1;
                continue;
            }
            // seen from right
            if row[(i + 1)..row.len()].iter().all(|&el| el < *x) {
                count += 1;
                continue;
            }
            // seen from top
            if (0..j).all(|j| rows[j][i] < *x) {
                count += 1;
                continue;
            }
            // seen from bottom
            if ((j + 1)..rows.len()).all(|j| rows[j][i] < *x) {
                count += 1;
                continue;
            }
        }
    }
    println!("{count}")
}
