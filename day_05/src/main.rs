use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::result::Result as StdResult;
use std::usize;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;
    let mut lines = input.lines().peekable();
    let rules: Vec<(usize, usize)> = lines
        .peeking_take_while(|line| !line.is_empty())
        .map(|l| {
            l.split_once('|')
                .ok_or_else(|| parse_error("incorrect rule format", l))
                .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))
        })
        .try_collect()?;

    let updates: Vec<Vec<usize>> = lines
        .skip_while(|l| l.is_empty())
        .map(|l| l.split(',').map(|s| s.parse()).try_collect())
        .try_collect()?;

    println!("{:?}", rules);
    println!("{:?}", updates);

    print_part_1(part_one(&rules, &updates));
    //print_part_2(part_two(&input));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip())]
fn part_one( rules: &[(usize,usize)], updates: &Vec<Vec<usize>> ) -> StdResult<usize, Infallible> {
    let rules: HashSet<(usize,usize)> = rules.iter().copied().collect();

    Ok( updates.iter().filter( |update| { 
        update.iter().combinations(2).all( |pair| {
            let (a,b) = (*pair[0],*pair[1]);
            rules.contains( &(a,b) )
        })
    } ).map( |update| update[ update.len() / 2]).sum())
}

#[tracing::instrument(level=Level::DEBUG,skip())]
fn part_two(input: &str) -> StdResult<usize, Infallible> {
    todo!()
}
