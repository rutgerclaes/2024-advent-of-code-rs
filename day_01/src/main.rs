use std::io::Read;

use err_into::ErrorInto;
use itertools::Itertools;
use utils::prelude::*;

fn main() -> Result<()> {
    let pairs: Vec<(i32, i32)> = parse_input(&read_input()?)?;
    println!("Solution to Part 1: {}", part_one(&pairs));
    println!("Solution to Part 2: {}", part_two(&pairs));

    Ok(())
}

fn part_one(pairs: &[(i32, i32)]) -> i32 {
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = pairs.iter().copied().unzip();
    a.sort();
    b.sort();
    a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum()
}

fn part_two(pairs: &[(i32, i32)]) -> i32 {
    let (a, b): (Vec<i32>, Vec<i32>) = pairs.iter().copied().unzip();
    let counts = b.iter().counts();
    a.iter()
        .map(|a| a * *counts.get(a).unwrap_or(&0) as i32)
        .sum()
}

fn read_input() -> Result<String> {
    match std::env::args().nth(1) {
        Some(file) if file != "-" => std::fs::read_to_string(file).err_into(),
        _ => {
            let mut input = String::new();
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut input)?;
            Ok(input)
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once("   ")
                .ok_or_else(|| parse_error("wrong delimiter", line))?;
            Ok((a.parse()?, b.parse()?))
        })
        .try_collect()
}
