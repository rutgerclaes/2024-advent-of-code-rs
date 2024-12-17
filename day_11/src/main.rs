use std::collections::HashMap;

use itertools::Itertools;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let stones = read_input()?
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    print_part_1(&part_one(&stones));
    print_part_2(&part_two(&stones));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(stones))]
fn part_one(stones: &[u64]) -> Result<usize> {
    Ok(simulate(stones, 25))
}

#[tracing::instrument(level=Level::DEBUG,skip(stones))]
fn part_two(stones: &[u64]) -> Result<usize> {
    Ok(simulate(stones, 75))
}

fn simulate(stones: &[u64], iterations: usize) -> usize {
    let stones = stones.iter().map(|&s| (s, 1)).into_grouping_map().sum();

    (0..iterations)
        .fold(stones, |stones, _| {
            let new_stones = HashMap::new();
            stones
                .into_iter()
                .fold(new_stones, |mut new_stones, (stone, count)| {
                    if stone == 0 {
                        insert(&mut new_stones, 1, count);
                        new_stones
                    } else if nb_of_digits(&stone) % 2 == 0 {
                        let (first, second) = split(&stone);
                        insert(&mut new_stones, first, count);
                        insert(&mut new_stones, second, count);
                        new_stones
                    } else {
                        insert(&mut new_stones, stone * 2024, count);
                        new_stones
                    }
                })
        })
        .values()
        .sum()
}

fn insert(stones: &mut HashMap<u64, usize>, stone: u64, count: usize) {
    let existing = stones.entry(stone).or_insert(0);
    *existing += count;
}

fn nb_of_digits(n: &u64) -> u32 {
    if n < &10 {
        1
    } else {
        n.ilog10() + 1
    }
}

fn split(n: &u64) -> (u64, u64) {
    let half = nb_of_digits(n) / 2;
    let first = n / 10u64.pow(half);
    let second = n % 10u64.pow(half);
    (first, second)
}
