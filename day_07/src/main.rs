use itertools::Itertools;
use std::fmt::Display;
use std::result::Result as StdResult;
use std::str::FromStr;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let equations: Vec<Equation> = parse_lines()?;

    print_part_1(part_one(&equations));
    print_part_2(part_two(&equations));

    Ok(())
}

#[derive(Debug, Clone)]
struct Equation(u64, Vec<u64>);

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.1.iter().join(" "))
    }
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let (left, right) = s
            .split_once(": ")
            .ok_or_else(|| parse_error("failed to find ':'", s))?;
        let left: u64 = left.parse()?;
        let right: Vec<u64> = right
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .try_collect()?;
        Ok(Self(left, right))
    }
}

impl Equation {
    fn resolve(&self, signs: &[Operator]) -> Option<Vec<Operator>> {
        fn inner(
            eq: &Equation,
            i: usize,
            acc: u64,
            signs: Vec<Operator>,
            possible_signs: &[Operator],
        ) -> Option<Vec<Operator>> {
            if i == eq.1.len() {
                if acc == eq.0 {
                    Some(signs)
                } else {
                    None
                }
            } else {
                possible_signs.iter().find_map(|&sign| {
                    let next = sign.apply(acc, eq.1[i]);

                    if next > eq.0 {
                        None
                    } else {
                        let mut signs = signs.clone();
                        signs.push(sign);
                        inner(eq, i + 1, next, signs, possible_signs)
                    }
                })
            }
        }

        inner(self, 0, 0, Vec::new(), signs)
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Times,
    Concat,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Times => write!(f, "*"),
            Self::Concat => write!(f, "||"),
        }
    }
}

impl Operator {
    const MATH: [Self; 2] = [Self::Plus, Self::Times];

    const ALL: [Self; 3] = [Self::Plus, Self::Times, Self::Concat];

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Plus => a + b,
            Self::Times => a * b,
            Self::Concat => a * 10u64.pow(b.ilog10() as u32 + 1) + b,
        }
    }
}

#[tracing::instrument(level=Level::DEBUG,skip(equations))]
fn part_one(equations: &[Equation]) -> Result<u64> {
    Ok(equations
        .iter()
        .filter_map(|e| e.resolve(&Operator::MATH).map(|_| e.0))
        .sum())
}

#[tracing::instrument(level=Level::DEBUG,skip(equations))]
fn part_two(equations: &[Equation]) -> Result<u64> {
    Ok(equations
        .iter()
        .filter_map(|e| e.resolve(&Operator::ALL).map(|_| e.0))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operators() {
        assert_eq!(Operator::Plus.apply(1, 2), 3);
        assert_eq!(Operator::Plus.apply(100, 2), 102);

        assert_eq!(Operator::Times.apply(1, 2), 2);
        assert_eq!(Operator::Times.apply(8, 4), 32);

        assert_eq!(Operator::Concat.apply(1, 2), 12);
        assert_eq!(Operator::Concat.apply(12, 34), 1234);
        assert_eq!(Operator::Concat.apply(100, 100), 100100);
        assert_eq!(Operator::Concat.apply(10, 20), 1020);
        assert_eq!(Operator::Concat.apply(1, 10), 110);
    }
}
