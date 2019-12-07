use std::collections::HashMap;
use std::io::{self, BufRead};

// couldnt help but to adapt
// https://github.com/Morganamilo/adventofcode2019/blob/master/src/6-1.rs

pub fn part1() {
    let mut count = 0;
    let orbits = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| (l[4..7].to_string(), l[0..3].to_string()))
        .collect::<HashMap<_, _>>();

    // maybe dp to optimize?
    for mut body in orbits.keys() {
        while let Some(parent) = orbits.get(body) {
            count += 1;
            body = parent;
        }
    }
    // Answer is 144909
    println!("{}", count);
}

pub fn part2() {
    let orbits = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| (l[4..=6].to_string(), l[0..=2].to_string()))
        .collect::<HashMap<_, _>>();

    let you = parents(&orbits, "YOU");
    let san = parents(&orbits, "SAN");
    let common = you.iter().position(|b| san.contains(b)).unwrap();
    let count = common + san.iter().position(|&b| b == you[common]).unwrap() + 2;

    println!("{}", count);
}

fn parents<'a>(orbits: &'a HashMap<String, String>, body: &str) -> Vec<&'a str> {
    let mut body = orbits.get(body).unwrap();
    let mut chain = Vec::new();

    while let Some(parent) = orbits.get(body) {
        chain.push(parent.as_str());
        body = parent;
    }

    chain
}
