use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;
use num::Signed;
use tracing::Level;
use utils::prelude::*;

type Point = utils::geom::Point<i16>;
type Maze = utils::geom::Grid<i16, Element>;

fn main() -> Result<()> {
    init_tracing();

    let maze: Maze = read_input()?.parse()?;

    print_part_1(&part_one(&maze));
    print_part_2(&part_two(&maze));

    Ok(())
}

enum Element {
    Wall,
    Empty,
    Start,
    Destination,
}

impl TryFrom<char> for Element {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '#' => Ok(Element::Wall),
            '.' => Ok(Element::Empty),
            'S' => Ok(Element::Start),
            'E' => Ok(Element::Destination),
            _ => Err(parse_error("invalid element", &value.to_string())),
        }
    }
}

fn fill(start: &Point, maze: &Maze) -> HashMap<Point, usize> {
    fn inner(
        maze: &Maze,
        time: usize,
        front: HashSet<Point>,
        mut from_start: HashMap<Point, usize>,
    ) -> HashMap<Point, usize> {
        front.iter().for_each(|p| {
            from_start.insert(*p, time);
        });

        let new_front = front
            .iter()
            .flat_map(|p| p.neighbours())
            .filter(|p| !matches!(maze.get(p), Some(Element::Wall)) && !from_start.contains_key(p))
            .collect::<HashSet<_>>();

        if new_front.is_empty() {
            from_start
        } else {
            inner(maze, time + 1, new_front, from_start)
        }
    }

    inner(maze, 0, iter::once(*start).collect(), HashMap::new())
}

#[tracing::instrument(level=Level::DEBUG,skip(maze))]
fn part_one(maze: &Maze) -> Result<usize> {
    let shortcuts = |p: &Point| -> HashSet<Point> {
        let candidates = [
            (p.x, p.y + 2),
            (p.x + 1, p.y + 1),
            (p.x + 2, p.y),
            (p.x + 1, p.y - 1),
            (p.x, p.y - 2),
            (p.x - 1, p.y + 1),
            (p.x - 2, p.y),
            (p.x - 1, p.y - 1),
        ];
        candidates.iter().map(|(x, y)| Point::new(*x, *y)).collect()
    };

    solve(maze, shortcuts, 100)
}

fn solve<F>(maze: &Maze, shortcut: F, threshold: usize) -> Result<usize>
where
    F: Fn(&Point) -> HashSet<Point>,
{
    let start = maze
        .iter()
        .find_map(|(p, e)| {
            if matches!(e, Element::Start) {
                Some(p)
            } else {
                None
            }
        })
        .ok_or_else(|| Error::SolutionNotFound("no start found".to_owned()))?;

    let end = maze
        .iter()
        .find_map(|(p, e)| {
            if matches!(e, Element::Destination) {
                Some(p)
            } else {
                None
            }
        })
        .ok_or_else(|| Error::SolutionNotFound("no destination found".to_owned()))?;

    let from_start = fill(start, maze);
    let best_time = from_start.get(end).unwrap();
    let to_destination = fill(end, maze);

    let shortcuts: HashMap<usize, usize> = from_start
        .iter()
        .flat_map(|(from, from_start)| {
            shortcut(from)
                .iter()
                .filter_map(|to| {
                    let distance = (from.x - to.x).abs() + (from.y - to.y).abs();
                    to_destination
                        .get(to)
                        .map(|from_end| from_start + from_end + distance as usize)
                        .filter(|&time| time < *best_time)
                        .map(|t| (best_time - t, 1))
                })
                .collect_vec()
        })
        .into_grouping_map()
        .sum();

    if tracing::enabled!(Level::DEBUG) {
        shortcuts.iter().for_each(|(savings, count)| {
            tracing::debug!(
                "- There are {} cheats that save {} picoseconds",
                count,
                savings
            );
        });
    }

    Ok(shortcuts.iter().fold(0, |sum, (&savings, count)| {
        if savings >= threshold {
            sum + count
        } else {
            sum
        }
    }))
}

#[tracing::instrument(level=Level::DEBUG,skip(maze))]
fn part_two(maze: &Maze) -> Result<usize> {
    let delta = (-20..=20)
        .flat_map(|dx| (-20..=20).map(move |dy| (dx, dy)))
        .filter(|(dx, dy)| dx.abs() + dy.abs() <= 20)
        .collect_vec();

    let shortcut = |point: &Point| -> HashSet<Point> {
        delta
            .iter()
            .map(|(dx, dy)| Point::new(point.x + dx, point.y + dy))
            .collect()
    };

    solve(maze, shortcut, 100)
}
