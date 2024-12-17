use std::{convert::Infallible, fmt::Display, str::FromStr};

use itertools::{FoldWhile, Itertools};
use std::result::Result as StdResult;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let pairs: Vec<Report> = parse_lines()?;
    print_part_1(&part_one(&pairs));
    print_part_2(&part_two(&pairs));

    Ok(())
}

#[derive(Debug, Clone)]
struct Report(Vec<i32>);

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().join(" "))
    }
}

impl Report {
    fn check<'a, I>(i: I) -> bool
    where
        I: Iterator<Item = &'a i32>,
    {
        let result = i
            .tuple_windows()
            .fold_while((None, false), |(sign, _), (a, b)| {
                let diff = b - a;
                let abs_diff = diff.abs();
                let diff_sign = diff.signum();

                tracing::trace!(
                    distance = abs_diff,
                    sign = diff_sign,
                    "checking safety between {} and {}",
                    a,
                    b
                );

                if !(1..=3).contains(&abs_diff) {
                    tracing::trace!(
                        distance = abs_diff,
                        "unsafe: distance between {} and {} is too large",
                        a,
                        b
                    );
                    FoldWhile::Done((None, false))
                } else {
                    match sign {
                        None => FoldWhile::Continue((Some(diff_sign), true)),
                        Some(prev_sign) if prev_sign == diff_sign => {
                            FoldWhile::Continue((Some(diff_sign), true))
                        }
                        _ => {
                            tracing::trace!(
                                sign = diff_sign,
                                "unsafe: sign changed between {} and {}",
                                a,
                                b
                            );
                            FoldWhile::Done((None, false))
                        }
                    }
                }
            });

        matches!(result, FoldWhile::Continue((_, true)))
    }

    #[tracing::instrument(level=Level::TRACE)]
    fn is_safe(&self) -> bool {
        Self::check(self.0.iter())
    }

    #[tracing::instrument(level=Level::TRACE)]
    fn is_safe_with_dampner(&self) -> bool {
        self.is_safe()
            || self.0.iter().enumerate().any(|(i, _)| {
                tracing::trace!("checking when removing level at index {}", i);
                Self::check(
                    self.0
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| i != *j)
                        .map(|(_, v)| v),
                )
            })
    }
}

impl FromStr for Report {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let levels = s.split(' ').map(|s| s.parse()).try_collect()?;
        Ok(Report(levels))
    }
}

#[tracing::instrument(level=Level::DEBUG,skip(reports))]
fn part_one(reports: &[Report]) -> StdResult<usize, Infallible> {
    Ok(reports
        .iter()
        .map(|r| (r, r.is_safe()))
        .inspect(|(r, s)| tracing::debug!("{}: {}", r, s))
        .filter(|&(_, safe)| safe)
        .count())
}

#[tracing::instrument(level=Level::DEBUG,skip(reports))]
fn part_two(reports: &[Report]) -> StdResult<usize, Infallible> {
    Ok(reports
        .iter()
        .map(|r| (r, r.is_safe_with_dampner()))
        .inspect(|(r, s)| tracing::debug!("{}: {}", r, s))
        .filter(|&(_, safe)| safe)
        .count())
}

#[cfg(test)]
mod test {
    use crate::Report;

    #[test]
    fn test_dampner_on_examples() {
        let r: Report = "7 6 4 2 1".parse().expect("Parsing report should work");
        assert_eq!(r.is_safe_with_dampner(), true);

        let r: Report = "1 2 7 8 9".parse().expect("Parsing report should work");
        assert_eq!(r.is_safe_with_dampner(), false);

        let r: Report = "9 7 6 2 1".parse().expect("Parsing report should work");
        assert_eq!(r.is_safe_with_dampner(), false);

        let r: Report = "1 3 2 4 5".parse().expect("Parsing report should work");
        assert_eq!(r.is_safe_with_dampner(), true);

        let r: Report = "8 6 4 4 1".parse().expect("Parsing report should work");
        assert_eq!(r.is_safe_with_dampner(), true);

        let r: Report = "1 3 6 7 9".parse().expect("Parsing report should work");
        assert_eq!(r.is_safe_with_dampner(), true);
    }

    #[test]
    fn test_dampner_on_first_invalid() {
        let r: Report = "1 5 6".parse().expect("Parsing report should work");
        assert_eq!(r.is_safe_with_dampner(), true);
    }

    #[test]
    fn test_dampner_on_last_invalid() {
        let r: Report = "1 2 6".parse().expect("Parsing report should work");
        assert_eq!(r.is_safe_with_dampner(), true);
    }
}
