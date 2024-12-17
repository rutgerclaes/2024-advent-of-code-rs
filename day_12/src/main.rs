use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;
use tracing::Level;
use utils::{
    geom::{self, Direction, Vector},
    prelude::*,
};

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .collect_vec();
    let garden = construct_garden(input)?;

    tracing::debug!("constructed garden with {} regions", garden.iter().count());

    print_part_1(&part_one(&garden));
    print_part_2(&part_two(&garden));

    Ok(())
}

type Point = geom::Point<i32>;

struct Garden(Vec<(char, HashSet<Point>)>);

impl Garden {
    fn iter(&self) -> impl Iterator<Item = &(char, HashSet<Point>)> {
        self.0.iter()
    }
}

fn construct_garden<I>(tiles: I) -> Result<Garden>
where
    I: IntoIterator<Item = ((usize, usize), char)>,
{
    let assignable: HashMap<_, _> = tiles
        .into_iter()
        .map(|((x, y), char)| (Point::new(x as i32, y as i32), char))
        .collect();

    fn grow(
        plant: char,
        mut points: HashSet<Point>,
        boundary: HashSet<Point>,
        mut pool: HashMap<Point, char>,
    ) -> HashSet<Point> {
        let next_boundary: HashSet<_> = boundary
            .iter()
            .flat_map(|p| p.neighbours().filter(|p| pool.get(p) == Some(&plant)))
            .collect();
        points.extend(boundary);
        pool.retain(|p, _| !next_boundary.contains(p));

        if next_boundary.is_empty() {
            points
        } else {
            grow(plant, points, next_boundary, pool)
        }
    }

    fn assign(
        mut assignments: Vec<(char, HashSet<Point>)>,
        mut assignable: HashMap<Point, char>,
    ) -> Vec<(char, HashSet<Point>)> {
        match assignable.iter().next() {
            None => assignments,
            Some((point, &plant)) => {
                let pool = assignable
                    .iter()
                    .filter(|(_, &c)| c == plant)
                    .map(|(&p, _)| (p, plant))
                    .collect();
                let boundary = iter::once(*point).collect();
                let points = grow(plant, HashSet::new(), boundary, pool);
                assignable.retain(|p, _| !points.contains(p));
                assignments.push((plant, points));
                assign(assignments, assignable)
            }
        }
    }

    let garden = Garden(assign(Vec::new(), assignable));
    Ok(garden)
}

#[tracing::instrument(level=Level::DEBUG,skip(garden))]
fn part_one(garden: &Garden) -> Result<usize> {
    let price = garden
        .iter()
        .map(|(c, points)| {
            let area = points.len();
            let perimeter = points
                .iter()
                .flat_map(|p| p.neighbours().filter(|p| !points.contains(p)))
                .count();

            let price = area * perimeter;
            tracing::debug!(
                "region {} with area {} and perimeter {}: {}",
                c,
                area,
                perimeter,
                price
            );
            price
        })
        .sum();

    Ok(price)
}

#[tracing::instrument(level=Level::DEBUG,skip(garden))]
fn part_two(garden: &Garden) -> Result<u64> {
    let price = garden
        .iter()
        .map(|(c, points)| {
            let area = points.len();
            let edges = edges(points);

            let price = area as u64 * edges as u64;

            tracing::debug!(
                "region {} with area {} and edges {}: {}",
                c,
                area,
                edges,
                price
            );
            price
        })
        .sum();

    Ok(price)
}

fn edges(points: &HashSet<Point>) -> usize {
    let counts: HashMap<_, _> = points.iter().map(|p| (p, corners(p, points))).collect();
    counts.values().sum()
}

fn corners(point: &Point, others: &HashSet<Point>) -> usize {
    let outer = Direction::iter()
        .filter(|d| {
            !others.contains(&point.step(d)) && !others.contains(&point.step(&d.rotate_left()))
        })
        .count();

    let inner = Direction::iter()
        .filter(|&d| {
            let right = d.rotate_right();
            let diag: Vector<i32> = Vector::from(d) + Vector::from(right);
            others.contains(&point.step(&d))
                && others.contains(&point.step(&right))
                && !others.contains(&point.move_by(diag))
        })
        .count();

    inner + outer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corners() {
        assert_eq!(
            corners(&Point::new(0, 0), &[Point::new(0, 0)].into_iter().collect()),
            4
        );

        assert_eq!(
            corners(
                &Point::new(0, 0),
                &[Point::new(0, 0), Point::new(1, 0)].into_iter().collect()
            ),
            2
        );
        assert_eq!(
            corners(
                &Point::new(0, 0),
                &[Point::new(0, 0), Point::new(0, 1)].into_iter().collect()
            ),
            2
        );

        assert_eq!(
            corners(
                &Point::new(0, 0),
                &[Point::new(1, 0), Point::new(0, 0), Point::new(0, 1)]
                    .into_iter()
                    .collect()
            ),
            2
        );

        assert_eq!(
            corners(
                &Point::new(0, 0),
                &[Point::new(0, 0), Point::new(0, 1), Point::new(0, -1)]
                    .into_iter()
                    .collect()
            ),
            0
        );
    }

    #[test]
    fn test_edges() {
        assert_eq!(edges(&[Point::new(0, 0)].into_iter().collect()), 4);
        assert_eq!(
            edges(&[Point::new(0, 0), Point::new(1, 0)].into_iter().collect()),
            4
        );
        assert_eq!(
            edges(
                &[
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(1, 1),
                    Point::new(0, 1)
                ]
                .into_iter()
                .collect()
            ),
            4
        );

        assert_eq!(
            edges(
                &[Point::new(0, 0), Point::new(1, 0), Point::new(0, 1)]
                    .into_iter()
                    .collect()
            ),
            6
        );

        assert_eq!(
            edges(
                &[
                    Point::new(0, 0),
                    Point::new(0, 1),
                    Point::new(1, 1),
                    Point::new(2, 1),
                    Point::new(2, 0)
                ]
                .into_iter()
                .collect()
            ),
            8
        );
        assert_eq!(
            edges(
                &[
                    Point::new(0, 0),
                    Point::new(0, 1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                    Point::new(2, 2),
                    Point::new(2, 1),
                    Point::new(2, 0)
                ]
                .into_iter()
                .collect()
            ),
            8
        );

        assert_eq!(
            edges(
                &[
                    Point::new(0, 0),
                    Point::new(0, 1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                    Point::new(2, 2),
                    Point::new(2, 1),
                    Point::new(2, 0),
                    Point::new(1, 0)
                ]
                .into_iter()
                .collect()
            ),
            8
        );
        assert_eq!(
            edges(
                &[
                    Point::new(0, 0),
                    Point::new(0, 1),
                    Point::new(0, 2),
                    Point::new(1, 2),
                    Point::new(2, 2),
                    Point::new(2, 1),
                    Point::new(2, 0),
                    Point::new(1, 0),
                    Point::new(1, 1)
                ]
                .into_iter()
                .collect()
            ),
            4
        );
    }
}
