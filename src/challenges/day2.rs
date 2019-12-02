use std::io;

const DAY: i32 = 2;
const TITLE: &str = "1202 Program Alarm";

const EXIT: usize = 99;
const ADD: usize = 1;
const MUL: usize = 2;

const TARGET: usize = 19690720;

pub fn part1() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let mut tokens: Vec<usize> = input
        .split(',')
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut index = 0;

    loop {
        let operator = tokens[index];

        if operator == EXIT {
            break;
        }

        let op1 = tokens[index + 1];
        let op2 = tokens[index + 2];
        let dest = tokens[index + 3];

        tokens[dest] = match operator {
            ADD => tokens[op1] + tokens[op2],
            MUL => tokens[op1] * tokens[op2],
            _ => break,
        };
        index += 4;
    }

    // Answer is 3101844
    println!("Answer is {}", tokens[0]);
}

pub fn part2() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let original: Vec<usize> = input
        .split(',')
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut index = 0;
    let mut tokens;

    for noun in 0..100 {
        for verb in 0..100 {
            index = 0;
            tokens = original.clone();
            tokens[1] = noun;
            tokens[2] = verb;

            'inner: loop {
                let operator = tokens[index];

                if operator == EXIT {
                    if tokens[0] == TARGET {
                        println!("Answer is ({})", 100 * noun + verb);
                    }
                    break 'inner;
                }

                let op1 = tokens[index + 1];
                let op2 = tokens[index + 2];
                let dest = tokens[index + 3];

                tokens[dest] = match operator {
                    ADD => tokens[op1] + tokens[op2],
                    MUL => tokens[op1] * tokens[op2],
                    _ => break,
                };
                index += 4;
            }
        }
    }
}
