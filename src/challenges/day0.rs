use std::io::{self, BufRead};

const DAY: i32 = 1;
const TITLE: &str = "Title goes here";

pub fn part1() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut line: String;

    for input in io::stdin().lock().lines() {
        line = input.unwrap();

        if line.is_empty() {
            break;
        }

        /*
            MAGIC goes here
        */
    }

    // Answer is XXX
    println!("print answer!!!");
}

pub fn part2() {
    println!("--- Day {} Part 2: {} ---", DAY, TITLE);

    let mut line: String;

    for input in io::stdin().lock().lines() {
        line = input.unwrap();

        if line.is_empty() {
            break;
        }

        /*
            MAGIC goes here
        */
    }

    // Answer is XXX
    println!("print answer!!!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // assert_eq!(4, add_two(2));
    }
}
