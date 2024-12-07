use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::result::Result as StdResult;
use tracing::Level;
use utils::geom::{Direction, Grid};
use utils::{geom, prelude::*};

#[macro_use]
extern crate tramp;
use tramp::{tramp, BorrowRec};

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
        mut visited: HashSet<Point>,
    ) -> BorrowRec<usize> {
        let next_position = position.step(&direction);

        match map.get(&next_position) {
            Some(Location::Obstacle) => {
                tracing::debug!("obstacle at {:?}, turning right", next_position);
                let direction = direction.turn_right();
                rec_call!(take_step(map, position, direction, visited))
            }
            Some(Location::Empty) => {
                visited.insert(next_position.clone());
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
    let mut visited = HashSet::new();
    visited.insert(position.clone());
    Ok(tramp(take_step(map, position, direction, visited)))
}

#[tracing::instrument(level=Level::DEBUG,skip(map,starting_point))]
fn part_two(map: &Map, starting_point: (Direction, Point)) -> Result<usize> {
    
    #[inline]
    fn update_visited( visited: &mut HashMap<Point,HashSet<Direction>>, position: Point, direction: Direction ) {
        if let Some( d ) = visited.get_mut( &position ) {
            d.insert( direction );
        } else {
            visited.insert( position.clone(), [direction].iter().cloned().collect() );
        }
    }

    fn take_step(
        map: &Map,
        addition: Option<Point>,
        position: Point,
        direction: Direction,
        mut visited: HashMap<Point,HashSet<Direction>>,
        mut modifications: HashSet<Point>,
    ) -> BorrowRec<Option<usize>> {
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
                update_visited(&mut visited, position.clone(), direction);

                rec_call!(take_step(
                    map,
                    addition,
                    position,
                    next_direction,
                    visited,
                    modifications
                ))
            }
            Some(Location::Empty) if visited.get( &next_position ).map_or( false, |set| set.contains( &direction ) ) => {
                tracing::debug!(
                    "already visited {:?} in {:?} direction",
                    next_position,
                    direction
                );
                rec_ret!(None)
            }
            Some(Location::Empty) => {
                if addition.is_none() && !modifications.contains(&next_position) && !visited.contains_key( &next_position ) {
                    let loops: Option<usize> = tramp(take_step(
                        map,
                        Some(next_position.clone()),
                        position.clone(),
                        direction.turn_right(),
                        visited.clone(),
                        modifications.clone(),
                    ));
                    if loops.is_none() {
                        tracing::info!("inserting 'O' at {:?}", next_position);
                        modifications.insert(next_position.clone());
                    }
                }

                update_visited(&mut visited, next_position.clone(), direction);

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
    let mut visited = HashMap::new();
    visited.insert(position.clone(), [direction].iter().cloned().collect());
    tramp(take_step(
        map,
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

impl Default for Location {
    fn default() -> Self {
        Location::Empty
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
