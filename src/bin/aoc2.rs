use std::cmp::max;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn split_set(set: &str) -> [u32; 3] {
    let mut result = [0; 3];
    for c in set.split(",") {
        let v = c.split(" ").collect::<Vec<_>>();
        match v[2] {
            "red" => result[0] += v[1].parse::<u32>().unwrap(),
            "green" => result[1] += v[1].parse::<u32>().unwrap(),
            "blue" => result[2] += v[1].parse::<u32>().unwrap(),
            _ => panic!("{} not recognized", v[2]),
        }
    }
    result
}

fn pless(a: [u32; 3], b: [u32; 3]) -> bool {
    a[0] <= b[0] && a[1] <= b[1] && a[2] <= b[2]
}

fn pmax(a: [u32; 3], b: [u32; 3]) -> [u32; 3] {
    [max(a[0], b[0]), max(a[1], b[1]), max(a[2], b[2])]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc2.dat");
    let w = [12, 13, 14];
    let input = read_to_string(&path)?;
    let games = input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(k, v)| {
            (
                k.split_once(" ").unwrap().1.parse::<u32>().unwrap(),
                v.split(";").map(|s| split_set(s)),
            )
        });

    let part1: u32 = games
        .clone()
        .map(|(k, mut v)| (k, v.all(|s| pless(s, w))))
        .filter_map(|(k, c)| if c { Some(k) } else { None })
        .sum();

    let part2: u32 = games
        .map(|(k, v)| (k, v.into_iter().reduce(pmax).unwrap()))
        .map(|(_, v)| v.into_iter().product::<u32>())
        .sum();

    println!("Part #1: {:?}", part1);
    println!("Part #2: {:?}", part2);
    Ok(())
}
