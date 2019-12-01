use std::io::{self, BufRead};

const TITLE: &str = "Title goes here";
const DAY: i32 = 1;

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
