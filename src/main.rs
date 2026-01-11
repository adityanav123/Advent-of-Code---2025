mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn solve_day1() {
    println!("day1::");
    print!("Q1 => ");
    let res = day1::solve_q1("input.txt");
    match res {
        Ok(pwd) => println!("Password is : {:?}", pwd),
        Err(_) => eprintln!("error in solving!"),
    }

    print!("Q2 => ");
    let res1 = day1::solve_q2("test.txt");
    match res1 {
        Ok(pwd) => println!("Password is : {:?}", pwd),
        Err(_) => eprintln!("error in solving!"),
    }
}

fn solve_day2() {
    println!("day2::");
    print!("Q1 => ");
    let res = day2::solve_q1("input.txt");
    match res {
        Ok(pwd) => println!("Invalid Data : {:?}", pwd),
        Err(_) => eprintln!("error in solving!"),
    }

    print!("Q2 => ");
    let res1 = day2::solve_q2("input.txt");
    match res1 {
        Ok(pwd) => println!("Invalid Data : {:?}", pwd),
        Err(_) => eprintln!("error in solving!"),
    }
}

fn solve_day3() {
    print!("day3::");
    // println!("Q1 => ");
    // let res = day3::solve_q1("input.txt");
    // match res {
    //     Ok(joltage) => println!("Total Output Joltage : {:?}", joltage),
    //     Err(_) => eprintln!("error in solving!"),
    // }

    println!("Q2 => ");
    let res1 = day3::solve_q2("input.txt");
    match res1 {
        Ok(joltage) => println!("Total Output Joltage : {:?}", joltage),
        Err(_) => eprintln!("error in solving!"),
    }
}

fn solve_day4() {
    print!("day4::");
    // println!("Q1 => ");
    // let res = day4::solve_q1("input.txt");
    // match res {
    //     Ok(count) => println!("Total rolls : {:?}", count),
    //     Err(_) => eprintln!("error in solving!"),
    // }

    println!("Q2 => ");
    let res1 = day4::solve_q2("input.txt");
    match res1 {
        Ok(count) => println!("Total rolls : {:?}", count),
        Err(_) => eprintln!("error in solving!"),
    }
}

fn solve_day5() {
    print!("day5::");
    // println!("Q1 => ");
    // let res = day5::solve_q1("input.txt");
    // match res {
    //     Ok(count) => println!("Total Fresh Ingredients : {:?}", count),
    //     Err(_) => eprintln!("error in solving!"),
    // }

    println!("Q2 => ");
    let res1 = day5::solve_q2("input.txt");
    match res1 {
        Ok(count) => println!("Only Fresh Ingredients : {:?}", count),
        Err(_) => eprintln!("error in solving!"),
    }
}

fn solve_day6() {
    print!("day6::");
    println!("Q1 => ");
    let res = day6::solve_q1("input.txt");
    match res {
        Ok(result) => println!("solution = {:?}", result),
        Err(_) => eprintln!("error in solving!")
    };
}

fn main() {
    // solve_day1();
    // solve_day2();
    // solve_day3
    // solve_day4();
    // solve_day5();
    solve_day6();
}
