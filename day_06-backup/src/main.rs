use im::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::result::Result as StdResult;
use tracing::Level;
use utils::geom::{Direction, Grid};
use utils::{geom, prelude::*};

#[macro_use]
extern crate tramp;
use tramp::{tramp, BorrowRec, Rec};

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;
    let location: Point = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, c)| {
                if c == '^' {
                    Some(Point::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| Error::SolutionNotFound("starting location not found".to_owned()))?;

    let starting_point = (Direction::Up, location);
    let map: Map = input.parse()?;

    print_part_1(part_one(&map, starting_point.clone()));
    print_part_2(part_two(&map, starting_point));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(map, starting_point))]
fn part_one(map: &Map, starting_point: (Direction, Point)) -> Result<usize> {
    fn take_step(
        map: &Map,
        position: Point,
        direction: Direction,
        visited: HashSet<Point>,
    ) -> BorrowRec<usize> {
        let next_position = position.step(&direction);

        match map.get(&next_position) {
            Some(Location::Obstacle) => {
                tracing::debug!("obstacle at {:?}, turning right", next_position);
                let next_direction = direction.turn_right();
                rec_call!(take_step(map, position, next_direction, visited))
            }
            Some(Location::Empty) => {
                let visited = visited.update(next_position.clone());
                tracing::debug!("moving to {:?}, now visited {:?}", next_position, visited);
                rec_call!(take_step(map, next_position, direction, visited))
            }
            None => {
                tracing::debug!("reached end of map, visited {:?}", visited);
                rec_ret!(visited.len())
            }
        }
    }

    let (direction, position) = starting_point;
    let visited = HashSet::unit(position.clone());
    Ok(tramp(take_step(map, position, direction, visited)))
}

#[tracing::instrument(level=Level::DEBUG,skip(map))]
fn part_two(map: &Map, starting_point: (Direction, Point)) -> Result<usize> {
    fn take_step(
        map: Map,
        addition: Option<Point>,
        position: Point,
        direction: Direction,
        visited: HashMap<Point, HashSet<Direction>>,
        modifications: HashSet<Point>,
    ) -> Rec<Option<usize>> {
        let next_position = position.step(&direction);

        match map.get(&next_position).map(|&n| {
            if Some(&next_position) == addition.as_ref() {
                Location::Obstacle
            } else {
                n
            }
        }) {
            Some(Location::Obstacle) => {
                tracing::debug!("obstacle at {:?}, turning right", next_position);
                let next_direction = direction.turn_right();
                let visited = visited.update_with(
                    position.clone(),
                    HashSet::unit(next_direction),
                    |set, new| set.union(new),
                );

                rec_call!(take_step(
                    map,
                    addition,
                    position,
                    direction,
                    visited,
                    modifications
                ))
            }
            Some(Location::Empty)
                if visited
                    .get(&next_position)
                    .map_or(false, |set| set.contains(&direction)) =>
            {
                tracing::debug!(
                    "already visited {:?} in {:?} direction",
                    next_position,
                    direction
                );
                rec_ret!(None)
            }
            Some(Location::Empty) => {
                let modifications = if addition.is_none()
                    && !modifications.contains(&next_position)
                    && !visited.iter().any(|(v, _)| v == &next_position)
                {
                    let creates_loop = tramp(take_step(
                        map.clone(),
                        Some(next_position.clone()),
                        position.clone(),
                        direction.turn_right(),
                        visited.clone(),
                        modifications.clone(),
                    ))
                    .is_none();

                    if creates_loop {
                        tracing::info!("inserting 'O' at {:?}", next_position);
                        modifications.update(next_position.clone())
                    } else {
                        modifications
                    }
                } else {
                    modifications
                };

                let visited =
                    visited.update_with(position, HashSet::unit(direction), |set, new| {
                        set.union(new)
                    });

                tracing::debug!("moving to {:?}, now visited {:?}", next_position, visited);
                rec_call!(take_step(
                    map,
                    addition,
                    next_position,
                    direction,
                    visited,
                    modifications
                ))
            }
            None => {
                tracing::debug!("reached end of map, visited {:?}", visited);
                rec_ret!(Some(modifications.len()))
            }
        }
    }

    let (direction, position) = starting_point;
    let visited = HashMap::unit(position.clone(), HashSet::unit(direction));
    tramp(take_step(
        map.clone(),
        None,
        position,
        direction,
        visited,
        HashSet::new(),
    ))
    .ok_or(Error::SolutionNotFound("no solution found".to_owned()))
}

type Map = Grid<i32, Location>;

type Point = geom::Point<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    Empty,
    Obstacle,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Empty => write!(f, "."),
            Location::Obstacle => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Location {
    type Error = Error;

    fn try_from(c: char) -> StdResult<Self, Self::Error> {
        match c {
            '.' => Ok(Location::Empty),
            '#' => Ok(Location::Obstacle),
            '^' => Ok(Location::Empty),
            _ => Err(parse_error(
                "could not parse location",
                &format!("invalid character: {}", c),
            )),
        }
    }
}
