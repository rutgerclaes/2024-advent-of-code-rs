use std::io::Read;

use err_into::ErrorInto;
use itertools::Itertools;
use utils::prelude::*;

fn main() -> Result<()> {
    let pairs: Vec<(i32, i32)> = parse_input(&read_input()?)?;
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
    a.sort();
    b.sort();
    let part_1: i32 = a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum();
    println!("Solution to Part 1: {}", part_1);

    Ok(())
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
