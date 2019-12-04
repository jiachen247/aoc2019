const DAY: i32 = 4;
const TITLE: &str = "Secure Container";

const RANGE_START: i32 = 353096;
const RANGE_END: i32 = 843212;

fn is_valid(num: i32) -> bool {
    let mut has_duplicate_neigbour = false;
    let mut is_increasing = true;

    let num_vec: Vec<i32> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect();

    if num_vec.len() < 2 {
        return false;
    }

    for i in 1..num_vec.len() {
        let prev = num_vec[i - 1];
        let curr = num_vec[i];

        if prev == curr {
            has_duplicate_neigbour = true;
        }

        if prev > curr {
            is_increasing = false;
        }
    }

    if has_duplicate_neigbour && is_increasing {
        println!("valid={}", num);
    }

    has_duplicate_neigbour && is_increasing
}

fn is_valid2(num: i32) -> bool {
    let mut has_duplicate_neigbour = false;
    let mut is_increasing = true;

    let num_vec: Vec<i32> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect();

    if num_vec.len() < 2 {
        return false;
    }

    for i in 1..num_vec.len() {
        let prev = num_vec[i - 1];
        let curr = num_vec[i];

        let prev_group = i > 1 && num_vec[i - 2] == curr;
        let forward_group = i < 5 && num_vec[i + 1] == curr;

        if prev == curr {
            if !prev_group && !forward_group {
                has_duplicate_neigbour = true;
            }
        }

        if prev > curr {
            is_increasing = false;
        }
    }

    if has_duplicate_neigbour && is_increasing {
        println!("valid={}", num);
    }

    has_duplicate_neigbour && is_increasing
}

pub fn part1() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);
    let mut count = 0;

    for i in RANGE_START..=RANGE_END {
        if is_valid(i) {
            count += 1;
        }
    }
    // Answer is 579
    println!("# of valid passwords = {}", count);
}

pub fn part2() {
    println!("--- Day {} Part 2: {} ---", DAY, TITLE);

    let mut count = 0;

    for i in RANGE_START..=RANGE_END {
        if is_valid2(i) {
            count += 1;
        }
    }
    // Answer is 358
    println!("# of valid passwords = {}", count);
}
