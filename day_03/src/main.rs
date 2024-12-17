use regex::Regex;
use std::convert::Infallible;
use std::result::Result as StdResult;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;
    print_part_1(&part_one(&input));
    print_part_2(&part_two(&input));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(input))]
fn part_one(input: &str) -> StdResult<i64, Infallible> {
    let r = Regex::new(r"mul\((?<a>[0-9]+),(?<b>[0-9]+)\)").unwrap();

    Ok(r.captures_iter(input)
        .map(|cap| {
            let a: i64 = cap
                .name("a")
                .unwrap()
                .as_str()
                .parse()
                .expect("a should be a number");
            let b: i64 = cap
                .name("b")
                .unwrap()
                .as_str()
                .parse()
                .expect("b should be a number");
            a * b
        })
        .sum())
}

#[tracing::instrument(level=Level::DEBUG,skip(input))]
fn part_two(input: &str) -> StdResult<i64, Infallible> {
    let r = Regex::new(r"mul\((?<a>[0-9]+),(?<b>[0-9]+)\)|do\(\)|don't\(\)").unwrap();

    let (_, sum) = r
        .captures_iter(input)
        .fold((true, 0), |(enabled, sum), cap| {
            match cap.get(0).unwrap().as_str() {
                "do()" => (true, sum),
                "don't()" => (false, sum),
                _ if enabled => {
                    let a: i64 = cap
                        .name("a")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("a should be a number");
                    let b: i64 = cap
                        .name("b")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("b should be a number");
                    (enabled, sum + a * b)
                }
                _ => (enabled, sum),
            }
        });

    Ok(sum)
}
