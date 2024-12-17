use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::result::Result as StdResult;
use std::str::FromStr;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;
    let mut lines = input.lines().peekable();
    let rules: RuleSet = lines
        .peeking_take_while(|line| !line.is_empty())
        .map(|l| {
            l.split_once('|')
                .ok_or_else(|| parse_error("incorrect rule format", l))
                .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))
        })
        .try_collect()?;

    let updates: Vec<Update> = lines
        .skip_while(|l| l.is_empty())
        .map(|l| l.parse())
        .try_collect()?;

    print_part_1(&part_one(&rules, &updates));
    print_part_2(&part_two(&rules, &updates));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(rules,updates))]
fn part_one(rules: &RuleSet, updates: &[Update]) -> StdResult<usize, Infallible> {
    Ok(updates
        .iter()
        .filter_map(|u| {
            if u.is_valid(rules) {
                Some(u.get_middle())
            } else {
                None
            }
        })
        .sum())
}

#[tracing::instrument(level=Level::DEBUG,skip(rules,updates))]
fn part_two(rules: &RuleSet, updates: &[Update]) -> Result<usize> {
    updates
        .iter()
        .filter_map(|u| {
            if u.is_valid(rules) {
                None
            } else {
                // Find the element that has an equal number of elements in the before and after rules
                Some(u.find_middle(rules).ok_or_else(|| {
                    Error::SolutionNotFound(format!("did not find middle element in {u:?}"))
                }))
            }
        })
        .try_fold(0, |sum, e| e.map(|e| e + sum))
}

struct RuleSet(HashMap<usize, HashSet<usize>>);

impl FromIterator<(usize, usize)> for RuleSet {
    fn from_iter<T: IntoIterator<Item = (usize, usize)>>(iter: T) -> Self {
        Self(iter.into_iter().into_grouping_map().collect())
    }
}
impl RuleSet {
    fn check_after(&self, first: &usize, second: &usize) -> bool {
        self.0
            .get(first)
            .map(|after_first| after_first.contains(second))
            .unwrap_or(false)
    }
}

#[derive(Debug)]
struct Update(Vec<usize>);

impl FromStr for Update {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let elem = s.split(',').map(|s| s.parse()).try_collect()?;
        Ok(Update(elem))
    }
}

impl Update {
    fn is_valid(&self, rules: &RuleSet) -> bool {
        self.0.iter().combinations(2).all(|pair| {
            let (a, b) = (pair[0], pair[1]);
            rules.check_after(a, b)
        })
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get_middle(&self) -> &usize {
        &self.0[self.len() / 2]
    }

    fn find_middle(&self, rules: &RuleSet) -> Option<&usize> {
        self.0
            .iter()
            .find(|i| self.0.iter().filter(|j| rules.check_after(i, j)).count() == self.len() / 2)
    }
}
