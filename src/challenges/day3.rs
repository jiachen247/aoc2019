use std::cmp;
use std::io;

const DAY: i32 = 3;
const TITLE: &str = "Crossed Wires";

const ARR_SIZE: i32 = 25_000;

#[derive(Copy, Clone)]
struct Tile {
    distance: i32,
    taken: bool,
}

struct Point {
    x: i32,
    y: i32,
}

pub struct Move {
    direction: char,
    magnitude: i32,
}

fn manhattan_distance(pt1: &Point, pt2: &Point) -> i32 {
    (pt1.x - pt2.x).abs() + (pt1.y - pt2.y).abs()
}

fn draw_wire(initial: &Point, state: &mut Vec<Vec<bool>>, wire: &Vec<Move>) -> i32 {
    let mut min = std::i32::MAX;
    let mut position: Point = Point {
        x: initial.x,
        y: initial.y,
    };

    for Move {
        direction,
        magnitude,
    } in wire
    {
        let mut count = 1;

        while count <= *magnitude {
            position = match direction {
                'U' => Point {
                    x: position.x,
                    y: position.y + 1,
                },
                'D' => Point {
                    x: position.x,
                    y: position.y - 1,
                },
                'L' => Point {
                    x: position.x - 1,
                    y: position.y,
                },
                'R' => Point {
                    x: position.x + 1,
                    y: position.y,
                },
                _ => Point {
                    x: initial.x,
                    y: initial.y,
                },
            };

            if state[position.x as usize][position.y as usize] {
                min = cmp::min(min, manhattan_distance(&initial, &position));
            } else {
                state[position.x as usize][position.y as usize] = true;
            }
            count += 1;
        }
    }
    min
}

fn draw_wire2(
    initial: &Point,
    state: &mut Vec<Vec<Tile>>,
    wire: &Vec<Move>,
    first_pass: bool,
) -> i32 {
    let mut min = std::i32::MAX;
    let mut index = 1;

    let mut position: Point = Point {
        x: initial.x,
        y: initial.y,
    };

    for Move {
        direction,
        magnitude,
    } in wire
    {
        let mut count = 1;

        while count <= *magnitude {
            position = match direction {
                'U' => Point {
                    x: position.x,
                    y: position.y + 1,
                },
                'D' => Point {
                    x: position.x,
                    y: position.y - 1,
                },
                'L' => Point {
                    x: position.x - 1,
                    y: position.y,
                },
                'R' => Point {
                    x: position.x + 1,
                    y: position.y,
                },
                _ => Point {
                    x: initial.x,
                    y: initial.y,
                },
            };

            if state[position.x as usize][position.y as usize].taken && !first_pass {
                min = cmp::min(
                    min,
                    index + state[position.x as usize][position.y as usize].distance,
                );
            } else if first_pass && !state[position.x as usize][position.y as usize].taken {
                state[position.x as usize][position.y as usize].distance = index;
                state[position.x as usize][position.y as usize].taken = true;
            }

            count += 1;
            index += 1;
        }
    }
    min
}

fn parse_wire(line: &String) -> Vec<Move> {
    line.trim()
        .split(',')
        .into_iter()
        .map(|s| Move {
            direction: s.chars().next().unwrap(),
            magnitude: String::from(s.chars().next().map(|c| &s[c.len_utf8()..]).unwrap())
                .parse::<i32>()
                .unwrap(),
        })
        .collect()
}

pub fn part1() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut line1 = String::new();
    io::stdin().read_line(&mut line1).unwrap();
    let mut line2 = String::new();
    io::stdin().read_line(&mut line2).unwrap();

    let wire1: Vec<Move> = parse_wire(&line1);
    let wire2: Vec<Move> = parse_wire(&line2);

    let mut state = vec![vec![false; ARR_SIZE as usize]; ARR_SIZE as usize];
    let starting: Point = Point {
        x: ARR_SIZE / 2,
        y: ARR_SIZE / 2,
    };
    state[starting.x as usize][starting.y as usize] = true;

    draw_wire(&starting, &mut state, &wire1);
    let min = draw_wire(&starting, &mut state, &wire2);

    // Answer is 248
    println!("print answer!!! {:?}", min);
}

pub fn part2() {
    println!("--- Day {} Part 1: {} ---", DAY, TITLE);

    let mut line1 = String::new();
    io::stdin().read_line(&mut line1).unwrap();
    let mut line2 = String::new();
    io::stdin().read_line(&mut line2).unwrap();

    let wire1: Vec<Move> = parse_wire(&line1);
    let wire2: Vec<Move> = parse_wire(&line2);

    let mut state = vec![
        vec![
            Tile {
                distance: 0,
                taken: false
            };
            ARR_SIZE as usize
        ];
        ARR_SIZE as usize
    ];
    let starting: Point = Point {
        x: ARR_SIZE / 2,
        y: ARR_SIZE / 2,
    };
    state[starting.x as usize][starting.y as usize] = Tile {
        distance: 0,
        taken: true,
    };

    draw_wire2(&starting, &mut state, &wire1, true);
    let min = draw_wire2(&starting, &mut state, &wire2, false);

    // Answer is 28580
    println!("print answer!!! {:?}", min);
}
