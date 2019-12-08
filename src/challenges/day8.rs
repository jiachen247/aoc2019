use std::i32;
use std::io;
use std::str;

const DAY: i32 = 8;
const TITLE: &str = "Space Image Format";

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

const BLACK: char = '0';
const WHITE: char = '1';
const TRANSPARENT: char = '2';

pub fn part1() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut program = String::new();
    io::stdin().read_line(&mut program).unwrap();

    let mut min_zeros = 9999999999999999usize;
    let mut min_layer = &"";

    let layers = program
        .as_bytes()
        .chunks((HEIGHT * WIDTH) as usize)
        .map(|x| str::from_utf8(x).unwrap())
        .collect::<Vec<&str>>();

    let mut layers_iter = layers.iter();

    while let Some(layer) = layers_iter.next() {
        let zeros = layer
            .chars()
            .filter(|x| *x == '0')
            .collect::<Vec<char>>()
            .len();

        if zeros < min_zeros {
            min_zeros = zeros;
            min_layer = &layer;
        }
    }

    let ones = min_layer
        .chars()
        .filter(|x| *x == '2')
        .collect::<Vec<char>>()
        .len();

    let twos = min_layer
        .chars()
        .filter(|x| *x == '1')
        .collect::<Vec<char>>()
        .len();

    let result = ones * twos;

    // 1072
    println!("Answer is: {}", result);
}

pub fn part2() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut program = String::new();
    io::stdin().read_line(&mut program).unwrap();

    let mut iter = program.chars();

    let mut canvas: Vec<Vec<char>> = vec![vec![TRANSPARENT; WIDTH]; HEIGHT];

    'outer: loop {
        let mut height_counter: usize = HEIGHT as usize;
        let mut width_counter: usize = WIDTH as usize;

        while height_counter > 0 {
            width_counter = WIDTH as usize;
            while width_counter > 0 {
                if let Some(c) = iter.next() {
                    match canvas[HEIGHT - height_counter][WIDTH - width_counter] {
                        TRANSPARENT | '3' => {
                            canvas[HEIGHT - height_counter][WIDTH - width_counter] = c;
                        }
                        _ => {}
                    }
                    width_counter -= 1;
                } else {
                    break 'outer;
                }
            }
            height_counter -= 1;
        }
    }

    // draws YLFPJ
    for line in canvas {
        let s: String = line.iter().collect();
        println!("{}", s);
    }
}
