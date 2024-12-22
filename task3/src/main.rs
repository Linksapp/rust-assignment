use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "file.txt";

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut rows: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        rows.push(fields);
    }

    if rows.is_empty() {
        return Ok(());
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    let mut transposed: Vec<Vec<String>> = vec![Vec::new(); num_cols];

    for i in 0..num_rows {
        for j in 0..num_cols {
            transposed[j].push(rows[i][j].clone());
        }
    }

    for row in transposed {
        println!("{}", row.join(" "));
    }

    Ok(())
}
