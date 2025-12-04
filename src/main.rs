use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, Result},
};

fn solve_q2(path: &str) -> Result<i32> {
    if path.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File Path Not Present!",
        ));
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    const TOTAL: i32 = 100;
    let mut curr = 50;
    let mut password = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // https://docs.rs/syntect/latest/syntect/util/fn.split_at.html
        let (direction_str, steps_str) = line.split_at(1);
        let dir = direction_str.chars().next().unwrap();
        let steps: i32 = steps_str
            .trim()
            .parse()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

        let zero_cnt = match dir.to_ascii_uppercase() {
            'L' => {
                let distance_to_zero = if curr == 0 { TOTAL } else { curr };
                let hits = if steps < distance_to_zero {
                    0
                } else {
                    1 + (steps - distance_to_zero) / TOTAL
                };
                curr = (curr - steps).rem_euclid(TOTAL); // remainder operator
                hits
            }
            'R' => {
                let hits = (curr + steps) / TOTAL;
                curr = (curr + steps).rem_euclid(TOTAL);
                hits
            }
            _ => {
                continue;
            }
        };
        password += zero_cnt;
    }
    Ok(password)
}

fn solve_q1(file: &str) -> Result<i32> {
    if file.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File Path Not Present!",
        ));
    }

    // Creating File Handle
    let fd = File::open(file)?;
    let buf_reader = BufReader::new(fd);

    let mut curr_val: i32 = 50;
    let total = 100;
    let mut password = 0;

    for line_as_res in buf_reader.lines() {
        let line = line_as_res?;
        let direction = line.chars().next().unwrap().to_ascii_uppercase();
        let value = &line[1..].parse::<i32>().unwrap();
        match direction {
            'L' => {
                curr_val = (curr_val - value - total) % total;
            }
            'R' => {
                curr_val = (curr_val + value) % total;
            }
            _ => {
                continue;
            }
        }
        password += match curr_val {
            0 => 1,
            _ => 0,
        }
    }

    Ok(password)
}

fn main() {
    print!("Q1 => ");
    let res = solve_q1("input.txt");
    match res {
        Ok(pwd) => println!("Password is : {:?}", pwd),
        Err(_) => eprintln!("error in solving!"),
    }

    print!("Q2 => ");
    let res1 = solve_q2("test.txt");
    match res1 {
        Ok(pwd) => println!("Password is : {:?}", pwd),
        Err(_) => eprintln!("error in solving!"),
    }
}
