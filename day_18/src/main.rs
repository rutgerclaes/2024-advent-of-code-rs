use std::{collections::HashSet, iter};

use itertools::{FoldWhile, Itertools};
use tracing::Level;
use utils::prelude::*;

type Point = utils::geom::Point<i32>;

struct Options {
    limit: usize,
    dimension: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            limit: 1024,
            dimension: 70,
        }
    }
}

fn main() -> Result<()> {
    init_tracing();

    let points: Vec<Point> = read_input()?
        .lines()
        .map(|s| {
            s.split_once(",")
                .ok_or_else(|| parse_error("could not parse point", s))
                .and_then(|(x, y)| Ok(Point::new(x.parse()?, y.parse()?)))
        })
        .try_collect()?;

    let options = Options::example(|| Options {
        limit: 12,
        dimension: 6,
    });
    let bounds = (options.dimension as i32, options.dimension as i32);

    let solution_one = part_one(
        &points.iter().take(options.limit).copied().collect_vec(),
        bounds,
    );
    print_part_1(&solution_one);
    print_part_2(&part_two(&points, options.limit, bounds).map(|p| format!("{},{}", p.x, p.y)));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(obstacles, bounds))]
fn part_one(obstacles: &[Point], bounds: (i32, i32)) -> Result<usize> {
    search(
        0,
        &obstacles.iter().copied().collect(),
        iter::once(Point::new(0, 0)).collect(),
        &mut HashSet::new(),
        bounds,
    )
    .ok_or_else(|| Error::SolutionNotFound("no solution found".to_owned()))
}

#[tracing::instrument(level=Level::DEBUG,skip(obstacles,bounds))]
fn part_two(obstacles: &[Point], limit: usize, bounds: (i32, i32)) -> Result<&Point> {
    fn find(l: usize, u: usize, obstacles: &[Point], bounds: (i32, i32)) -> Result<&Point> {
        if l == u {
            return obstacles
                .get(l - 1)
                .ok_or_else(|| Error::SolutionNotFound("no solution found".to_owned()));
        }
        let i = (l + u) / 2;
        match search(
            0,
            &obstacles.iter().take(i).copied().collect(),
            iter::once(Point::new(0, 0)).collect(),
            &mut HashSet::new(),
            bounds,
        ) {
            Some(_) => find(i + 1, u, obstacles, bounds),
            None => find(l, i, obstacles, bounds),
        }
    }

    find(limit, obstacles.len(), obstacles, bounds)
}

fn search(
    time: usize,
    obstacles: &HashSet<Point>,
    front: HashSet<Point>,
    visited: &mut HashSet<Point>,
    bounds: (i32, i32),
) -> Option<usize> {
    let new_front = front.iter().fold_while(
        (false, HashSet::new(), visited),
        |(_, new_front, visited), point| {
            if point.x == bounds.0 && point.y == bounds.1 {
                FoldWhile::Done((true, new_front, visited))
            } else {
                visited.insert(*point);
                let new_front = point
                    .neighbours()
                    .filter(|n| {
                        n.x >= 0
                            && n.y >= 0
                            && n.x <= bounds.0
                            && n.y <= bounds.1
                            && !visited.contains(n)
                            && !obstacles.contains(n)
                    })
                    .fold(new_front, |mut f, p| {
                        f.insert(p);
                        f
                    });
                FoldWhile::Continue((false, new_front, visited))
            }
        },
    );

    match new_front {
        FoldWhile::Done((true, _, _)) => Some(time),
        FoldWhile::Done(_) => unreachable!(),
        FoldWhile::Continue((false, new_front, _)) if new_front.is_empty() => None,
        FoldWhile::Continue((false, new_front, visited)) => {
            search(time + 1, obstacles, new_front, visited, bounds)
        }
        FoldWhile::Continue((true, _, _)) => unreachable!(),
    }
}

#[allow(dead_code)]
fn debug_ascii(
    time: usize,
    obstacles: &HashSet<utils::geom::Point<i32>>,
    front: &HashSet<utils::geom::Point<i32>>,
    visited: &mut HashSet<utils::geom::Point<i32>>,
    bounds: (i32, i32),
) {
    println!("time: {}", time);
    for y in 0..=bounds.1 {
        for x in 0..bounds.0 {
            let p = Point::new(x, y);
            if obstacles.contains(&p) {
                print!("#");
            } else if visited.contains(&p) {
                print!("O")
            } else if front.contains(&p) {
                print!("+")
            } else {
                print!(".")
            }
        }
        println!()
    }
}
