use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Error, Result},
    iter::Map,
};

fn q1_helper(ranges: &Vec<(i64, i64)>, ingredients: &Vec<i64>) -> Result<i64> {
    let mut cnt = 0i64;
    for &val in ingredients.iter() {
        let mut found = false;
        for &(start, end) in ranges.iter() {
            if val >= start && val <= end {
                found = true;
                break;
            }
        }
        if found {
            cnt += 1i64;
        }
    }
    Ok(cnt)
}

fn q2_helper(ranges: &Vec<(i64, i64)>) -> Result<i64> {
    if ranges.is_empty() {
        return Ok(0);
    }

    let mut rs = ranges.clone();

    for (s, e) in rs.iter_mut() {
        if *s > *e {
            std::mem::swap(s, e);
        }
    }

    rs.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut total: i64 = 0;
    let mut cur_start = rs[0].0;
    let mut cur_end = rs[0].1;

    for &(start, end) in rs.iter().skip(1) {
        if start <= cur_end.saturating_add(1) {
            cur_end = cur_end.max(end);
        } else {
            total += cur_end - cur_start + 1;
            cur_start = start;
            cur_end = end;
        }
    }

    total += cur_end - cur_start + 1;

    Ok(total)
}

pub fn solve_q1(file: &str) -> Result<i64> {
    if file.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "file path is empty",
        ));
    }

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ingredients: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.contains('-') {
            let parts: Vec<i64> = line
                .split('-')
                .map(|val| val.parse::<i64>().unwrap())
                .collect();
            ranges.push((parts[0], parts[1]));
        } else {
            let curr = line.parse::<i64>().unwrap();
            ingredients.push(curr);
        }
    }

    q1_helper(&ranges, &ingredients)
}

pub fn solve_q2(file: &str) -> Result<i64> {
    if file.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "file path is empty",
        ));
    }

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.contains('-') {
            let parts: Vec<i64> = line
                .split('-')
                .map(|val| val.parse::<i64>().unwrap())
                .collect();
            ranges.push((parts[0], parts[1]));
        } else {
            break;
        }
    }

    q2_helper(&ranges)
}
