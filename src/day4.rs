use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, Result},
};

// utility
fn q2_helper(grid: &mut Vec<Vec<bool>>, count: &mut i32) {
    if grid.is_empty() || grid[0].is_empty() {
        return;
    }

    let r_cnt = grid.len();
    let c_cnt = grid[0].len();

    for r in 0..r_cnt {
        for c in 0..c_cnt {
            if grid[r][c] {
                // check boundary
                let top = if r == 0 { None } else { Some(grid[r - 1][c]) };
                let bottom = if r == r_cnt - 1 {
                    None
                } else {
                    Some(grid[r + 1][c])
                };
                let left = if c == 0 { None } else { Some(grid[r][c - 1]) };
                let right = if c == c_cnt - 1 {
                    None
                } else {
                    Some(grid[r][c + 1])
                };
                // diagonal
                let top_left = if r == 0 || c == 0 {
                    None
                } else {
                    Some(grid[r - 1][c - 1])
                };
                let top_right = if r == 0 || c == c_cnt - 1 {
                    None
                } else {
                    Some(grid[r - 1][c + 1])
                };
                let bottom_left = if r == r_cnt - 1 || c == 0 {
                    None
                } else {
                    Some(grid[r + 1][c - 1])
                };
                let bottom_right = if r == r_cnt - 1 || c == c_cnt - 1 {
                    None
                } else {
                    Some(grid[r + 1][c + 1])
                };

                let mut roll_cnt = 0i32;
                for neighbor in [
                    top,
                    bottom,
                    left,
                    right,
                    top_left,
                    top_right,
                    bottom_left,
                    bottom_right,
                ]
                .iter()
                {
                    if let Some(val) = neighbor {
                        if *val {
                            roll_cnt += 1;
                        }
                    }
                }

                if roll_cnt < 4 {
                    *count += 1;
                    grid[r][c] = false;
                }
            }
        }
    }
}

fn q1_helper(grid: &Vec<Vec<bool>>, count: &mut i32) {
    if grid.is_empty() || grid[0].is_empty() {
        return;
    }

    let r_cnt = grid.len();
    let c_cnt = grid[0].len();

    // println!("row cnt: {:?}", r_cnt);
    // println!("col cnt: {:?}", c_cnt);

    for r in 0..r_cnt {
        for c in 0..c_cnt {
            if grid[r][c] {
                // check boundary
                let top = if r == 0 { None } else { Some(grid[r - 1][c]) };
                let bottom = if r == r_cnt - 1 {
                    None
                } else {
                    Some(grid[r + 1][c])
                };
                let left = if c == 0 { None } else { Some(grid[r][c - 1]) };
                let right = if c == c_cnt - 1 {
                    None
                } else {
                    Some(grid[r][c + 1])
                };
                // diagonal
                let top_left = if r == 0 || c == 0 {
                    None
                } else {
                    Some(grid[r - 1][c - 1])
                };
                let top_right = if r == 0 || c == c_cnt - 1 {
                    None
                } else {
                    Some(grid[r - 1][c + 1])
                };
                let bottom_left = if r == r_cnt - 1 || c == 0 {
                    None
                } else {
                    Some(grid[r + 1][c - 1])
                };
                let bottom_right = if r == r_cnt - 1 || c == c_cnt - 1 {
                    None
                } else {
                    Some(grid[r + 1][c + 1])
                };

                let mut roll_cnt = 0i32;
                for neighbor in [
                    top,
                    bottom,
                    left,
                    right,
                    top_left,
                    top_right,
                    bottom_left,
                    bottom_right,
                ]
                .iter()
                {
                    if let Some(val) = neighbor {
                        if *val {
                            roll_cnt += 1;
                        }
                    }
                }

                if roll_cnt < 4 {
                    *count += 1;
                }
            }
        }
    }
}

pub fn solve_q1(file: &str) -> Result<i32> {
    if file.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File path cannot be empty",
        ));
    }

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in reader.lines() {
        // println!("read line: {:?}", line);
        let line = line?;
        let line = line.trim();
        let row: Vec<bool> = line
            .chars()
            .map(|c| match c {
                '@' => true,
                '.' => false,
                _ => false,
            })
            .collect();
        grid.push(row);
    }

    let mut total_rolls = 0i32;

    q1_helper(&grid, &mut total_rolls);

    Ok(total_rolls)
}

pub fn solve_q2(file: &str) -> Result<i32> {
    if file.is_empty() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "File path cannot be empty",
        ));
    }

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in reader.lines() {
        // println!("read line: {:?}", line);
        let line = line?;
        let line = line.trim();
        let row: Vec<bool> = line
            .chars()
            .map(|c| match c {
                '@' => true,
                '.' => false,
                _ => false,
            })
            .collect();
        grid.push(row);
    }

    let mut total_rolls = 0i32;

    q2_helper(&mut grid, &mut total_rolls);
    // println!("at the start: {:?}", total_rolls);
    let mut current_rolls = total_rolls;
    while current_rolls != 0 {
        current_rolls = 0;
        q2_helper(&mut grid, &mut current_rolls);
        // println!("Current Rolls: {:?}", current_rolls);
        total_rolls += current_rolls;
    }

    Ok(total_rolls)
}
