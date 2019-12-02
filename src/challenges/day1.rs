use math::round;
use std::io::{self, BufRead};

const TITLE: &str = "The Tyranny of the Rocket Equation";
const DAY: i32 = 1;

fn calculate_fuel(mass: i32) -> i32 {
    round::floor(mass as f64 / 3.0, 0) as i32 - 2
}

fn calculate_fuel_recursive(mass: i32) -> i32 {
    let current_fuel = calculate_fuel(mass);
    if current_fuel <= 0 {
        return 0;
    } else {
        return current_fuel + calculate_fuel_recursive(current_fuel);
    }
}

pub fn part1() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);
    let mut mass: i32;
    let mut fuel_counter: i32 = 0;
    let mut line: String;

    for input in io::stdin().lock().lines() {
        line = input.unwrap();

        if line.is_empty() {
            break;
        }

        mass = line.parse::<i32>().unwrap();
        fuel_counter += calculate_fuel(mass);
    }

    // Fuel needed is 3365459
    println!("Fuel needed is {}!", fuel_counter);  
}

pub fn part2() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);
    let mut mass: i32;
    let mut fuel_counter: i32 = 0;
    let mut line: String;

    for input in io::stdin().lock().lines() {
        line = input.unwrap();

        if line.is_empty() {
            break;
        }

        mass = line.parse::<i32>().unwrap();
        fuel_counter += calculate_fuel_recursive(mass);
    }

    // Fuel needed is 5045301!
    println!("Fuel needed is {}!", fuel_counter);
}
