use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use tracing::Level;
use utils::{geom::Direction, prelude::*};

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;

    let mut lines = input.lines().peekable();

    let first = lines
        .peeking_take_while(|l| !l.trim().is_empty())
        .collect_vec();
    let position = first
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.chars()
                .position(|c| c == '@')
                .map(|x| Point::new(x as i16, y as i16))
        })
        .ok_or_else(|| parse_error("could not find starting position", ""))?;

    let map: Vec<Option<(Point, Element)>> = first
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().map(move |(x, c)| {
                let p = Point::new(x as i16, y as i16);
                let e = match c {
                    '#' => Ok(Some(Element::Wall)),
                    'O' => Ok(Some(Element::Box)),
                    '.' | '@' => Ok(None),
                    c => Err(parse_error("could not parse map", &format!("{:?}", c))),
                }?;

                Ok::<_, Error>(e.map(|e| (p, e)))
            })
        })
        .try_collect()?;

    let map = Map {
        elements: map.into_iter().flatten().collect(),
    };

    let instuctions: Vec<Direction> = lines
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            c => Err(parse_error(
                "could not parse instruction",
                &format!("{:?}", c),
            )),
        })
        .try_collect()?;

    print_part_1(&part_one(position, map.clone(), &instuctions));
    print_part_2(&part_two(position, map, &instuctions));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(position, map,instructions))]
fn part_one(position: Point, map: Map, instructions: &[Direction]) -> Result<u64> {
    let (_, map) = instructions.iter().fold((position, map), |(p, mut m), d| {
        let next = p.step(d);
        let p = m.move_to(next, d).unwrap_or(p);
        (p, m)
    });

    Ok(map.gps_coordinates())
}

#[tracing::instrument(level=Level::DEBUG,skip(position, map,instructions))]
fn part_two(position: Point, map: Map, instructions: &[Direction]) -> Result<u64> {
    let wide = map.scale();
    let position = Point::new(position.x * 2, position.y);

    let (_, map) = instructions.iter().fold((position, wide), |(p, mut m), d| {
        let p = m.w_move_to(p, d).unwrap_or(p);
        (p, m)
    });

    Ok(map.gps_coordinates())
}

type Point = utils::geom::Point<i16>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Wall,
    Box,
}

#[derive(Debug, Clone)]
struct Map {
    elements: HashMap<Point, Element>,
}

enum Lookup {
    Wall,
    Boxes(Vec<Point>),
    Empty,
}

impl Map {
    fn scale(self) -> Map {
        let elements: HashMap<_, _> = self
            .elements
            .into_iter()
            .flat_map(|(p, e)| {
                let s = Point::new(p.x * 2, p.y);
                match e {
                    Element::Wall => vec![
                        (s, Element::Wall),
                        (s.step(&Direction::Right), Element::Wall),
                    ],
                    Element::Box => vec![(s, Element::Box)],
                }
            })
            .collect();

        Map { elements }
    }

    fn lookup(&self, p: &Point) -> Lookup {
        match self.elements.get(p) {
            None => match self.elements.get(&p.step(&Direction::Left)) {
                Some(Element::Box) => Lookup::Boxes(vec![p.step(&Direction::Left), *p]),
                _ => Lookup::Empty,
            },
            Some(Element::Wall) => Lookup::Wall,
            Some(Element::Box) => Lookup::Boxes(vec![*p, p.step(&Direction::Right)]),
        }
    }

    fn calculate(
        &self,
        mut to_move: Vec<Point>,
        direction: &Direction,
        mut moving: HashSet<Point>,
    ) -> Option<HashSet<Point>> {
        match to_move.pop() {
            None => Some(moving),
            Some(p) => {
                let next = p.step(direction);
                match self.lookup(&next) {
                    Lookup::Wall => None,
                    Lookup::Empty => {
                        moving.insert(p);
                        self.calculate(to_move, direction, moving)
                    }
                    Lookup::Boxes(boxes) => {
                        let to_move = boxes.iter().fold(to_move, |mut a, p| {
                            if !moving.contains(p) && !a.contains(p) {
                                a.push(*p);
                                a
                            } else {
                                a
                            }
                        });
                        moving.insert(p);
                        self.calculate(to_move, direction, moving)
                    }
                }
            }
        }
    }

    fn w_move_to(&mut self, p: Point, direction: &Direction) -> Option<Point> {
        if let Some(points) = self.calculate(vec![p], direction, HashSet::new()) {
            let new: Vec<(Point, Element)> = points
                .iter()
                .filter_map(|p| self.elements.remove(p).map(|e| (p.step(direction), e)))
                .collect_vec();
            for (p, e) in new {
                self.elements.insert(p, e);
            }
            Some(p.step(direction))
        } else {
            None
        }
    }

    fn move_to(&mut self, p: Point, d: &Direction) -> Option<Point> {
        match self.elements.get(&p) {
            None => Some(p),
            Some(Element::Wall) => None,
            Some(Element::Box) => {
                let next = p.step(d);
                match self.move_to(next, d) {
                    None => None,
                    Some(next) => {
                        let e = self.elements.remove(&p).unwrap();
                        self.elements.insert(next, e);
                        Some(p)
                    }
                }
            }
        }
    }

    fn gps_coordinates(&self) -> u64 {
        self.elements
            .iter()
            .filter_map(|(p, e)| match e {
                Element::Box => Some(p.x as u64 + p.y as u64 * 100),
                _ => None,
            })
            .sum()
    }
}
