use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;
    let mut lines = input.lines();

    let lookup: Lookup = lines
        .next()
        .ok_or_else(|| parse_error("empty input", ""))?
        .split(", ")
        .map(|s| s.chars().collect_vec())
        .collect();

    let designs = lines
        .skip_while(|line| line.is_empty())
        .map(|line| Design(line.chars().collect()))
        .collect_vec();

    print_part_1(&part_one(&lookup, &designs));
    print_part_2(&part_two(&lookup, &designs));

    Ok(())
}

struct Lookup {
    patterns: HashMap<usize, HashSet<Vec<char>>>,
    lens: Vec<usize>,
}

impl FromIterator<Vec<char>> for Lookup {
    fn from_iter<T: IntoIterator<Item = Vec<char>>>(iter: T) -> Self {
        let table: HashMap<usize, HashSet<Vec<char>>> = iter
            .into_iter()
            .map(|p| (p.len(), p))
            .into_grouping_map()
            .collect();
        let max = table.keys().copied().max().unwrap_or(0);
        let lens = table.keys().copied().sorted_by_key(|i| max - i).collect();

        Self {
            patterns: table,
            lens,
        }
    }
}

impl Lookup {
    fn next(&self, design: &[char], offset: usize) -> Vec<Vec<char>> {
        self.lens
            .iter()
            .filter_map(|l| self.patterns.get(l).map(|s| (l, s)))
            .filter_map(|(len, set)| {
                let end = offset + len;
                if end <= design.len() {
                    let slice = &design[offset..end];
                    if set.contains(slice) {
                        Some(slice.to_vec())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Design(Vec<char>);

impl Design {
    #[tracing::instrument(level=Level::DEBUG,skip(self,patterns))]
    fn check(&self, patterns: &Lookup) -> bool {
        fn inner(
            design: &Vec<char>,
            index: usize,
            patterns: &Lookup,
            deadends: &mut HashSet<usize>,
        ) -> bool {
            if design.len() == index {
                true
            } else {
                let next = patterns
                    .next(design, index)
                    .into_iter()
                    .filter(|p| !deadends.contains(&(index + p.len())))
                    .collect_vec();
                if next.is_empty() {
                    deadends.insert(index);
                    false
                } else {
                    next.iter()
                        .any(|p| inner(design, index + p.len(), patterns, deadends))
                }
            }
        }

        inner(&self.0, 0, patterns, &mut HashSet::new())
    }

    fn count(&self, patterns: &Lookup) -> usize {
        fn inner(
            design: &Vec<char>,
            index: usize,
            patterns: &Lookup,
            partials: &mut HashMap<usize, usize>,
        ) -> usize {
            tracing::debug!(
                resolved = design.iter().take(index).join(""),
                "index: {}/{}",
                index,
                design.len()
            );
            if design.len() == index {
                1
            } else if let Some(&cached) = partials.get(&index) {
                cached
            } else {
                let next = patterns.next(design, index);
                let count = next
                    .iter()
                    .map(|p| inner(design, index + p.len(), patterns, partials))
                    .sum();
                partials.insert(index, count);
                count
            }
        }

        inner(&self.0, 0, patterns, &mut HashMap::new())
    }
}

#[tracing::instrument(level=Level::DEBUG,skip(patterns,designs))]
fn part_one(patterns: &Lookup, designs: &[Design]) -> Result<usize> {
    Ok(designs.iter().filter(|d| d.check(patterns)).count())
}

#[tracing::instrument(level=Level::DEBUG,skip(patterns,designs))]
fn part_two(patterns: &Lookup, designs: &[Design]) -> Result<usize> {
    Ok(designs.iter().map(|d| d.count(patterns)).sum())
}
