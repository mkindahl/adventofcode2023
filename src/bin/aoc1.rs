#![feature(iter_array_chunks)]

use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn part1_digits(line: &str) -> Vec<char> {
    let mut v = vec![];
    for i in 0..line.len() {
        let ch = line.chars().nth(i).unwrap();
        if ch.is_ascii_digit() {
            v.push(ch);
        }
    }
    v
}

fn part2_digits(line: &str) -> Vec<char> {
    let mut v = vec![];
    for i in 0..line.len() {
        let ch = line.chars().nth(i).unwrap();
        if ch.is_ascii_digit() {
            v.push(ch);
        } else {
            let suffix = line.get(i..).unwrap();
            if suffix.starts_with("one") {
                v.push('1');
            }
            if suffix.starts_with("one") {
                v.push('1');
            }
            if suffix.starts_with("two") {
                v.push('2');
            }
            if suffix.starts_with("three") {
                v.push('3');
            }
            if suffix.starts_with("four") {
                v.push('4');
            }
            if suffix.starts_with("five") {
                v.push('5');
            }
            if suffix.starts_with("six") {
                v.push('6');
            }
            if suffix.starts_with("seven") {
                v.push('7');
            }
            if suffix.starts_with("eight") {
                v.push('8');
            }
            if suffix.starts_with("nine") {
                v.push('9');
            }
        }
    }
    v
}

fn process(input: String, checker: fn(&str) -> Vec<char>) -> u32 {
    input
        .lines()
        .map(|l| {
            let v = checker(&l);
            match (v.first(), v.last()) {
                (Some(a), Some(b)) => vec![*a, *b]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
                _ => 0,
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc1.dat");
    let input = read_to_string(&path)?;

    println!("Part #1: {:?}", process(input.clone(), part1_digits));
    println!("Part #2: {:?}", process(input.clone(), part2_digits));
    Ok(())
}
