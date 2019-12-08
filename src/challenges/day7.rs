use permutate::Permutator;
use std::io;

const DAY: i32 = 7;
const TITLE: &str = "Amplification Circuit";

const EXIT_CODE: i32 = 99;
const ADD_CODE: i32 = 1;
const MULTIPLY_CODE: i32 = 2;
const READ_CODE: i32 = 3;
const WRITE_CODE: i32 = 4;
const JMP_IF_TRUE_CODE: i32 = 5;
const JMP_IF_FALSE_CODE: i32 = 6;
const SET_LESS_THAN_CODE: i32 = 7;
const SET_IF_EQUALS: i32 = 8;

const POSITION_MODE: i32 = 0;
const IMM_MODE: i32 = 1;

struct Instruction {
    op_code: i32,
    mode1: i32,
    mode2: i32,
    mode3: i32,
}

fn eval_arithmetic(code: i32, operand1: i32, operand2: i32) -> i32 {
    match code {
        ADD_CODE => operand1 + operand2,
        MULTIPLY_CODE => operand1 * operand2,
        _ => -1,
    }
}

fn get_operand(memory: &Vec<i32>, starting_addr: i32, mode: i32) -> i32 {
    match mode {
        POSITION_MODE => memory[memory[starting_addr as usize] as usize],
        IMM_MODE => memory[starting_addr as usize],
        _ => memory[starting_addr as usize],
    }
}

fn decode(int_code: i32) -> Instruction {
    if int_code == EXIT_CODE {
        return Instruction {
            op_code: 99,
            mode1: -1,
            mode2: -1,
            mode3: -1,
        };
    }

    let instruction: String = format!("{:0>5}", int_code);
    Instruction {
        op_code: instruction[3..5].to_string().parse::<i32>().unwrap(),
        mode1: instruction[2..3].to_string().parse::<i32>().unwrap(),
        mode2: instruction[1..2].to_string().parse::<i32>().unwrap(),
        mode3: instruction[0..1].to_string().parse::<i32>().unwrap(),
    }
}

pub fn execute(program: String, mut input: Vec<i32>) -> i32 {
    let mut output: Vec<i32> = Vec::new();
    input.reverse();

    let mut eip: usize = 0;

    let mut memory: Vec<i32> = program
        .split(',')
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let mem_len = memory.len();

    while eip < mem_len {
        match decode(memory[eip]) {
            Instruction {
                op_code: EXIT_CODE, ..
            } => break,
            Instruction {
                op_code,
                mode1,
                mode2,
                ..
            } if op_code == ADD_CODE || op_code == MULTIPLY_CODE => {
                let op1 = get_operand(&memory, eip as i32 + 1, mode1);
                let op2 = get_operand(&memory, eip as i32 + 2, mode2);
                let dest = memory[eip + 3];

                memory[dest as usize] = eval_arithmetic(op_code, op1, op2);
                eip += 4;
            }
            Instruction {
                op_code: READ_CODE, ..
            } => {
                let dest = memory[eip as usize + 1];
                memory[dest as usize] = input.pop().unwrap();
                eip += 2;
            }
            Instruction {
                op_code: WRITE_CODE,
                ..
            } => {
                println!(
                    "writing to output {}",
                    memory[memory[eip + 1 as usize] as usize]
                );
                output.push(memory[memory[eip + 1 as usize] as usize]);
                eip += 2;
            }
            Instruction {
                op_code: JMP_IF_TRUE_CODE,
                mode1,
                mode2,
                ..
            } => {
                let op1 = get_operand(&memory, eip as i32 + 1, mode1);
                let op2 = get_operand(&memory, eip as i32 + 2, mode2);
                if op1 != 0 {
                    eip = op2 as usize;
                } else {
                    eip += 3;
                }
            }
            Instruction {
                op_code: JMP_IF_FALSE_CODE,
                mode1,
                mode2,
                ..
            } => {
                let op1 = get_operand(&memory, eip as i32 + 1, mode1);
                let op2 = get_operand(&memory, eip as i32 + 2, mode2);
                if op1 == 0 {
                    eip = op2 as usize;
                } else {
                    eip += 3;
                }
            }
            Instruction {
                op_code: SET_LESS_THAN_CODE,
                mode1,
                mode2,
                ..
            } => {
                let op1 = get_operand(&memory, eip as i32 + 1, mode1);
                let op2 = get_operand(&memory, eip as i32 + 2, mode2);
                let result = if op1 < op2 { 1 } else { 0 };
                let dest = memory[eip + 3];
                memory[dest as usize] = result;
                eip += 4;
            }
            Instruction {
                op_code: SET_IF_EQUALS,
                mode1,
                mode2,
                ..
            } => {
                let op1 = get_operand(&memory, eip as i32 + 1, mode1);
                let op2 = get_operand(&memory, eip as i32 + 2, mode2);
                let result = if op1 == op2 { 1 } else { 0 };
                let dest = memory[eip + 3];
                memory[dest as usize] = result;
                eip += 4;
            }
            _ => {
                break;
            }
        }
    }

    // Answer is 7408802
    // println!("output is {:?}", output);
    output.pop().unwrap()
}

pub fn part1() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut program = String::new();
    io::stdin().read_line(&mut program).unwrap();
    let list = [&[&0, &1, &2, &3, &4][..]];
    let mut permutator = Permutator::new(&list[..]);
    let mut max = 0;

    while let Some(sequence) = permutator.next() {
        //println!("trying {:?}", sequence);
        if sequence.contains(&&0)
            && sequence.contains(&&1)
            && sequence.contains(&&2)
            && sequence.contains(&&3)
            && sequence.contains(&&4)
        {
            println!("valid {:?}", sequence);
            max = max.max(
                sequence
                    .iter()
                    .fold(0, |acc, x| execute(program.clone(), vec![**x, acc])),
            );
        }
    }

    // let output = execute(program.clone(), input);
    println!("AMP 4: {:?}", max);
}

pub fn part2() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut program = String::new();
    io::stdin().read_line(&mut program).unwrap();
    // let list = [&[&5, &6, &7, &8, &9][..]];
    // let mut permutator = Permutator::new(&list[..]);
    // let mut max = 0;

    // while let Some(sequence) = permutator.next() {
    //     if sequence.contains(&&5)
    //         && sequence.contains(&&6)
    //         && sequence.contains(&&7)
    //         && sequence.contains(&&8)
    //         && sequence.contains(&&9)
    //     {
    //         println!("valid {:?}", sequence);
    //         max = max.max(
    //             sequence
    //                 .iter()
    //                 .fold(0, |acc, x| execute(program.clone(), vec![**x, acc])),
    //         );
    //     }
    // }

    // let sequence = [9, 8, 7, 6, 5];
    // let max = sequence
    //     .iter()
    //     .fold(0, |acc, x| execute(program.clone(), vec![*x, acc]));

    let max = execute(program.clone(), vec![8, 0]);

    // let output = execute(program.clone(), input);
    println!("AMP 4: {:?}", max);
}
