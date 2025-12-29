use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, Result},
};

pub fn solve_q1(file: &str) -> Result<i32> {
    if file.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File Path Not Present!",
        ));
    }

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut total_op_joltage: i32 = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut max_value = 0;
        let chars: Vec<char> = line.chars().collect();

        for i in 0..chars.len() {
            if let Some(first_digit) = chars[i].to_digit(10) {
                let first = first_digit as i32;

                // Look for the best second digit after position i
                for j in (i + 1)..chars.len() {
                    if let Some(second_digit) = chars[j].to_digit(10) {
                        let second = second_digit as i32;
                        let value = first * 10 + second;

                        if value > max_value {
                            max_value = value;
                        }
                    }
                }
            }
        }

        println!("line [{:?}] joltage => {:?}", line, max_value);
        total_op_joltage += max_value;
    }

    Ok(total_op_joltage)
}

fn max_num_from_digits_upto_k(line: &str, k: usize) -> Result<i64> {
    let mut pow10 = vec![1_i64; k];
    for i in 1..k {
        pow10[i] = pow10[i - 1] * 10;
    }

    const NOT_POSSIBLE: i64 = i64::MIN / 2;

    let bytes = line.as_bytes();
    let mut dp_next = vec![NOT_POSSIBLE; k + 1];
    let mut dp_curr = vec![NOT_POSSIBLE; k + 1];

    dp_next[0] = 0; // no digit included

    for &b in bytes.iter().rev() {
        if !(b'0'..=b'9').contains(&b) {
            continue;
        }
        let digit = (b - b'0') as i64;

        dp_curr[0] = 0;
        for t in 1..=k {
            let skip_it = dp_next[t];
            let include_it = if dp_next[t - 1] == NOT_POSSIBLE {
                NOT_POSSIBLE
            } else {
                digit * pow10[t - 1] + dp_next[t - 1]
            };

            dp_curr[t] = skip_it.max(include_it);
        }
        std::mem::swap(&mut dp_curr, &mut dp_next);
    }

    Ok(dp_next.into_iter().max().unwrap_or(0))
}

fn solve_q2_helper(
    jolts: &Vec<i32>,
    idx: usize,
    curr_num: i64,
    curr_size: usize,
    max_size: usize,
) -> i64 {
    if idx >= jolts.len() {
        return curr_num;
    }

    if curr_size >= max_size {
        return curr_num;
    }

    let include_it = solve_q2_helper(
        jolts,
        idx + 1,
        curr_num * 10 + jolts[idx] as i64,
        curr_size + 1,
        max_size,
    );

    let exclude_it = solve_q2_helper(jolts, idx + 1, curr_num, curr_size, max_size);

    // Return max
    include_it.max(exclude_it)
}

pub fn solve_q2(file: &str) -> Result<i64> {
    if file.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File Path Not Present!",
        ));
    }

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut total_jolt_voltage: i64 = 0;

    const TOTAL_JOLT_DIGITS: usize = 12;
    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let res = max_num_from_digits_upto_k(line, TOTAL_JOLT_DIGITS)?;
        total_jolt_voltage += res;
    }

    Ok(total_jolt_voltage)
}
