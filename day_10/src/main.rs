use im::HashMap;
use itertools::Itertools;
use tracing::Level;
use utils::{geom, prelude::*};

type Map = geom::Grid<i16, u8>;

type Point = geom::Point<i16>;

fn main() -> Result<()> {
    init_tracing();

    let map: geom::Grid<i16, u8> = read_input()?
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| -> Result<(geom::Point<i16>, u8)> {
                    let point: Point = Point::new(x as i16, y as i16);
                    let height = c
                        .to_digit(10)
                        .ok_or_else(|| parse_error("could not parse digit", &format!("{c}")))?;
                    Ok((point, height as u8))
                })
        })
        .try_collect()?;

    print_part_1(part_one(&map));
    print_part_2(part_two(&map));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(map))]
fn part_one(map: &Map) -> Result<usize> {
    let trailheads = map
        .iter()
        .filter(|(_, &height)| height == 0)
        .map(|(point, _)| point)
        .collect_vec();

    Ok(trailheads
        .into_iter()
        .map(|start| {
            (1..=9)
                .fold(im::HashSet::unit(*start), |trails, height| {
                    trails
                        .into_iter()
                        .flat_map(|t| {
                            map.neighbors(&t)
                                .filter_map(|(p, &h)| if h == height { Some(p) } else { None })
                                .collect_vec()
                        })
                        .collect()
                })
                .len()
        })
        .sum())
}

#[tracing::instrument(level=Level::DEBUG,skip(map))]
fn part_two(map: &Map) -> Result<u64> {
    let trailheads = map
        .iter()
        .filter_map(|(p, &h)| if h == 0 { Some(p) } else { None })
        .collect_vec();
    let mut cache = HashMap::new();
    let r: usize = trailheads
        .into_iter()
        .map(|t| rating(t, map, &mut cache))
        .sum();
    Ok(r as u64)
}

fn rating(location: &Point, map: &Map, cache: &mut HashMap<Point, usize>) -> usize {
    match cache.get(location) {
        Some(&score) => score,
        None => {
            let score = match map.get(location) {
                Some(&9) => 1,
                Some(&h) => map
                    .neighbors(location)
                    .filter_map(|(np, &nh)| {
                        if h + 1 == nh {
                            Some(rating(&np, map, cache))
                        } else {
                            None
                        }
                    })
                    .sum(),
                None => 0,
            };
            cache.insert(*location, score);
            score
        }
    }
}
