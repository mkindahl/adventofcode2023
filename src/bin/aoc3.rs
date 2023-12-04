use std::cmp::min;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn taint_neigbourhood(t: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    let w = t[0].len();
    let h = t.len();
    for x in i.saturating_sub(1)..min(i + 2, h) {
        for y in j.saturating_sub(1)..min(j + 2, w) {
            t[x][y] = true;
        }
    }
}

fn taint_matrix(m: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let w = m[0].len();
    let h = m.len();
    let t: Vec<Vec<bool>> = m
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| !(c.is_ascii_digit() || *c == '.'))
                .collect()
        })
        .collect();
    let mut c = t.clone();
    for i in 0..h {
        for j in 0..w {
            if t[i][j] {
                taint_neigbourhood(&mut c, i, j);
            }
        }
    }
    c
}

fn part1(grid: &Vec<Vec<char>>) {
    let taint = taint_matrix(&grid);

    let mut numbers = vec![];
    for (i, r) in grid.iter().enumerate() {
        let mut number = 0;
        let mut tainted = false;
        for (j, c) in r.iter().enumerate() {
            if c.is_ascii_digit() {
                number = 10 * number + c.to_digit(10).unwrap();
                if taint[i][j] {
                    tainted = true;
                }
            } else {
                if tainted {
                    numbers.push(number);
                }
                number = 0;
                tainted = false;
            }
        }
        if tainted {
            numbers.push(number);
        }
    }

    println!("Part #1: {:?}", numbers.into_iter().sum::<u32>());
}

type Coord = [usize; 2]; // Coordinate
type BBox = [Coord; 2]; // Bounding box

fn collect_numbers(grid: &Vec<Vec<char>>) -> Vec<(u32, BBox)> {
    let mut numbers: Vec<(u32, [[usize; 2]; 2])> = vec![];
    for (i, r) in grid.iter().enumerate() {
        let mut number: Option<(u32, [usize; 2])> = None;
        for (j, c) in r.iter().enumerate() {
            if c.is_ascii_digit() {
                let dig = c.to_digit(10).unwrap();
                number = match number.take() {
                    Some((num, bbox)) => Some((10 * num + dig, bbox)),
                    None => Some((dig, [i.saturating_sub(1), j.saturating_sub(1)])),
                };
            } else {
                match number.take() {
                    Some((num, bb)) => numbers.push((num, [bb, [i + 1, j]])),
                    None => {}
                }
            }
        }
        if let Some((num, bb)) = number.take() {
            numbers.push((num, [bb, [i + 1, r.len()]]));
        }
    }
    numbers
}

fn parts(grid: &Vec<Vec<char>>) -> Vec<(char, [usize; 2])> {
    let mut parts: Vec<(char, [usize; 2])> = vec![];
    for (i, r) in grid.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if !c.is_ascii_digit() && *c != '.' {
                parts.push((*c, [i, j]));
            }
        }
    }
    parts
}

fn in_bb(c: &Coord, bb: &BBox) -> bool {
    bb[0][0] <= c[0] && bb[0][1] <= c[1] && c[0] <= bb[1][0] && c[1] <= bb[1][1]
}

fn part2(grid: &Vec<Vec<char>>) {
    let numbers = collect_numbers(grid);
    let parts = parts(grid);
    let gearboxes = parts
        .iter()
        .map(|(p, c)| {
            (
                p,
                numbers
                    .iter()
                    .filter_map(|(n, bb)| if in_bb(c, bb) { Some(n) } else { None }),
            )
        })
        .filter_map(|(&p, ns)| {
            if p == '*' && ns.clone().count() == 2 {
                Some(ns.product::<u32>())
            } else {
                None
            }
        });
    println!("Part #2: {:?}", gearboxes.sum::<u32>());
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc3.dat");
    let input = read_to_string(&path)?;

    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    part1(&grid);
    part2(&grid);
    Ok(())
}
