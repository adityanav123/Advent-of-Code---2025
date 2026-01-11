use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

pub fn solve_q1(file: &str) -> io::Result<i64> {
    if file.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "file path is empty",
        ));
    }

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut values: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    // last row contains operators
    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();

        let parts: Vec<&str> = line.split_whitespace().collect();

        let is_operators = parts
            .iter()
            .any(|&token| matches!(token, "+" | "-" | "*" | "/"));

        if is_operators {
            operators = parts
                .into_iter()
                .map(|operator| operator.to_string())
                .collect();
        } else {
            let nums: Vec<i64> = parts
                .into_iter()
                .map(|token| token.parse::<i64>().unwrap())
                .collect();
            values.push(nums);
        }
    }
    // println!("values: {:?}", values);
    // println!("operators: {:?}", operators);

    let mut result: i64 = 0; // final result

    if values.is_empty() || operators.is_empty() {
        return Ok(0);
    }

    for (c, op) in operators.iter().enumerate() {
        let mut col_iter = values.iter().map(|row| row[c]);

        let first = col_iter
            .next()
            .ok_or_else(|| Error::new(io::ErrorKind::InvalidData, "no rows"))?;

        let col_value = match op.as_str() {
            "+" => col_iter.fold(first, |acc, v| acc + v),
            "-" => col_iter.fold(first, |acc, v| acc - v),
            "*" => col_iter.fold(first, |acc, v| acc * v),
            "/" => col_iter.fold(first, |acc, v| acc / v),
            _ => {
                return Err(Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid operator: {op}"),
                ));
            }
        };

        result += col_value;
    }

    Ok(result)
}
