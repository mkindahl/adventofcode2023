use std::cmp::min;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn str2u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

fn part1(cards: &Vec<(HashSet<u32>, HashSet<u32>)>) {
    let result: u32 = cards
        .iter()
        .map(|(ws, ns)| ws & ns)
        .map(|s| (1 << s.len()) >> 1)
        .sum();
    println!("Part #1: {:?}", result);
}

fn part2(cards: &Vec<(HashSet<u32>, HashSet<u32>)>) {
    let wins = cards.iter().map(|(ws, ns)| ws & ns);
    let mut cnt = vec![1; cards.len() as usize];
    for (n, win) in wins.enumerate() {
        let end = min(n + win.len(), cnt.len());
        let add = cnt[n];
        for cell in &mut cnt[n + 1..=end] {
            *cell = *cell + add;
        }
    }
    println!("Part #2: {}", cnt.iter().sum::<u32>());
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc4.dat");
    let input = read_to_string(&path)?;
    let cards: Vec<_> = input
        .lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(_, p)| p.split_once("|").unwrap())
        .map(|(ws, ns)| {
            (
                ws.split_ascii_whitespace()
                    .map(|n| str2u32(n))
                    .collect::<HashSet<_>>(),
                ns.split_ascii_whitespace()
                    .map(|n| str2u32(n))
                    .collect::<HashSet<_>>(),
            )
        })
        .collect();
    part1(&cards);
    part2(&cards);
    Ok(())
}
