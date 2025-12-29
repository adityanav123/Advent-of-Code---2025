use std::{
    fs,
    io::{self, Error, Result},
};

fn is_invalid_id(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    let (first, second) = s.split_at(half);

    first == second
}

fn is_invalid_id_2(n: i64) -> bool {
    // have to try all possible block sizes
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    for d in 1..=len / 2 {
        if len % d != 0 {
            continue;
        }

        let chunk = &bytes[..d];
        let mut i = d;
        let mut chunk_present = true;

        while i < len {
            if &bytes[i..i + d] != chunk {
                chunk_present = false;
                break;
            }
            i += d;
        }

        if chunk_present {
            return true;
        }
    }

    false
}

fn check_range(range: &str) -> Result<i64> {
    // println!("Checking Range: {:?}", range);

    let (start_str, end_str) = range
        .trim()
        .split_once('-')
        .ok_or_else(|| Error::new(io::ErrorKind::InvalidInput, "Invalid range format"))?;

    let start: i64 = start_str.parse().map_err(|e| {
        Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid start value: {}", e),
        )
    })?;

    let end: i64 = end_str.parse().map_err(|e| {
        Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid end value: {}", e),
        )
    })?;

    // println!("range : ({:?} -to- {:?})", start_str, end_str);

    let mut sum_in_range: i64 = 0;

    for id in start..=end {
        if is_invalid_id(id) {
            // println!("Found Invalid ! = {:?}", id);
            sum_in_range += id as i64;
        }
    }

    Ok(sum_in_range)
}

fn check_range2(range: &str) -> Result<i64> {
    // println!("Checking Range: {:?}", range);

    let (start_str, end_str) = range
        .trim()
        .split_once('-')
        .ok_or_else(|| Error::new(io::ErrorKind::InvalidInput, "Invalid range format"))?;

    let start: i64 = start_str.parse().map_err(|e| {
        Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid start value: {}", e),
        )
    })?;

    let end: i64 = end_str.parse().map_err(|e| {
        Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid end value: {}", e),
        )
    })?;

    // println!("range : ({:?} -to- {:?})", start_str, end_str);

    let mut sum_in_range: i64 = 0;

    for id in start..=end {
        if is_invalid_id_2(id) {
            // println!("Found Invalid ! = {:?}", id);
            sum_in_range += id as i64;
        }
    }

    Ok(sum_in_range)
}

pub fn solve_q1(path: &str) -> Result<i64> {
    if path.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File Path Not Present!",
        ));
    }

    let input = fs::read_to_string(path)?;
    let values = input.trim().split(',');

    let mut invalid_res_value = 0;

    for part in values {
        let res = check_range(part.trim());
        match res {
            Ok(val) => invalid_res_value += val,
            Err(_) => continue, // Invalid Range
        }
    }

    Ok(invalid_res_value)
}

pub fn solve_q2(path: &str) -> Result<i64> {
    if path.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File Path Not Present!",
        ));
    }

    let input = fs::read_to_string(path)?;
    let values = input.trim().split(',');

    let mut invalid_res_value = 0;

    for part in values {
        let res = check_range2(part.trim());
        match res {
            Ok(val) => invalid_res_value += val,
            Err(_) => continue, // Invalid Range
        }
    }

    Ok(invalid_res_value)
}
