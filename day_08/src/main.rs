use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use tracing::Level;
use utils::geom::BBox;
use utils::geom::Point;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let (max_x, max_y, stations) = read_input()?.lines().enumerate().fold(
        (0, 0, Vec::<(char, Point<i32>)>::new()),
        |(max_x, max_y, stations), (y, line)| {
            let (max_x, stations) = line.chars().enumerate().fold(
                (max_x, stations),
                |(max_x, mut stations), (x, c)| {
                    if c != '.' {
                        stations.push((c, Point::new(x as i32, y as i32)));
                    }
                    (max_x.max(x), stations)
                },
            );

            (max_x, max_y.max(y), stations)
        },
    );

    let stations: HashMap<char, Vec<Point<i32>>> = stations.into_iter().into_group_map();
    let bounds = BBox::new(0, max_x as i32, 0, max_y as i32);

    print_part_1(part_one(&stations, &bounds));
    print_part_2(part_two(&stations, &bounds));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(stations, bbox))]
fn part_one(stations: &HashMap<char, Vec<Point<i32>>>, bbox: &BBox<i32>) -> Result<usize> {
    let points: HashSet<_> = stations
        .values()
        .flat_map(|points| {
            points
                .iter()
                .combinations(2)
                .filter_map(|a| a.into_iter().collect_tuple())
                .flat_map(|(a, b)| [(a, b), (b, a)])
                .filter_map(|(a, b)| {
                    let dx = b.x - a.x;
                    let dy = b.y - a.y;
                    let p = b.move_by(dx, dy);

                    bbox.filter(p)
                })
                .collect::<HashSet<_>>()
        })
        .collect();

    Ok(points.len())
}

#[tracing::instrument(level=Level::DEBUG,skip(stations, bbox))]
fn part_two(stations: &HashMap<char, Vec<Point<i32>>>, bbox: &BBox<i32>) -> Result<usize> {
    let points: HashSet<Point<i32>> = stations
        .values()
        .flat_map(|points| {
            points
                .iter()
                .combinations(2)
                .filter_map(|a| a.into_iter().collect_tuple())
                .flat_map(|(a, b)| [(a, b), (b, a)])
                .flat_map(|(a, &b)| {
                    let dx = b.x - a.x;
                    let dy = b.y - a.y;

                    std::iter::successors(Some(b), move |p| bbox.filter(p.move_by(dx, dy)))
                })
                .collect::<HashSet<_>>()
        })
        .collect();

    Ok(points.len())
}
