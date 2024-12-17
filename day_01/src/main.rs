use std::convert::Infallible;

use itertools::Itertools;
use std::result::Result as StdResult;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let pairs: Vec<(i32, i32)> = parse_input(&read_input()?)?;
    print_part_1(&part_one(&pairs));
    print_part_2(&part_two(&pairs));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(pairs))]
fn part_one(pairs: &[(i32, i32)]) -> StdResult<i32, Infallible> {
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = pairs.iter().copied().unzip();
    a.sort();
    b.sort();
    Ok(a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum())
}

#[tracing::instrument(level=Level::DEBUG,skip(pairs))]
fn part_two(pairs: &[(i32, i32)]) -> StdResult<i32, Infallible> {
    let (a, b): (Vec<i32>, Vec<i32>) = pairs.iter().copied().unzip();
    let counts = b.iter().counts();
    Ok(a.iter()
        .map(|a| a * *counts.get(a).unwrap_or(&0) as i32)
        .sum())
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
