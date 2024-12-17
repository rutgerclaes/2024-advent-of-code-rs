use std::collections::HashMap;

use im::{vector, HashSet};
use tracing::Level;

use utils::{geom::Direction, prelude::*};

type Point = utils::geom::Point<i32>;
type Maze = utils::geom::Grid<i32, Element>;

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;

    let start = find_position(&input, 'S')
        .ok_or_else(|| parse_error("could not find starting position", ""))?;
    let destination = find_position(&input, 'E')
        .ok_or_else(|| parse_error("could not find starting position", ""))?;

    let maze: Maze = input.parse()?;
    let solution = part_one(&maze, &start, &destination);
    print_part_1(&solution);
    print_part_2(&part_two(&maze, &start, &destination, Some(solution?)));

    Ok(())
}

fn find_position(input: &str, needle: char) -> Option<Point> {
    input.lines().enumerate().find_map(|(y, l)| {
        l.chars()
            .position(|c| c == needle)
            .map(|x| Point::new(x as i32, y as i32))
    })
}

#[tracing::instrument(level=Level::INFO,skip(maze,start,destination))]
fn part_one(maze: &Maze, start: &Point, destination: &Point) -> Result<u64> {
    let (_, solution) = search(
        maze,
        destination,
        0,
        (start, &Direction::Right),
        HashMap::new(),
        None,
    );
    solution.ok_or_else(|| Error::SolutionNotFound("no solution found".to_owned()))
}

#[tracing::instrument(level=Level::INFO,skip(maze,start,destination))]
fn part_two(
    maze: &Maze,
    start: &Point,
    destination: &Point,
    solution: Option<u64>,
) -> Result<usize> {
    let (_, _, points) = search_and_collect(
        maze,
        destination,
        0,
        vector![(*start, Direction::Right)],
        HashMap::new(),
        solution,
        HashSet::new(),
    );

    Ok(points.len())
}

fn min_cost(state: (Point, Direction), destination: &Point) -> u64 {
    let dx = (state.0.x - destination.x).unsigned_abs() as u64;
    let dy = (state.0.y - destination.y).unsigned_abs() as u64;

    if dx == 0 || dy == 0 {
        dx + dy
    } else {
        dx + dy + 1000
    }
}

fn search(
    maze: &Maze,
    destination: &Point,
    cost: u64,
    state: (&Point, &Direction),
    mut history: HashMap<(Point, Direction), u64>,
    best_solution: Option<u64>,
) -> (HashMap<(Point, Direction), u64>, Option<u64>) {
    let (position, orientation) = state;

    if position == destination {
        match best_solution {
            None => (history, Some(cost)),
            Some(old) if cost < old => (history, Some(cost)),
            _ => (history, best_solution),
        }
    } else {
        history.insert((*position, *orientation), cost);

        let options = generate_options(
            maze,
            position,
            orientation,
            &history,
            cost,
            best_solution,
            destination,
        );

        options.into_iter().fold(
            (history, best_solution),
            |(history, best_solution), ((point, direction), c)| {
                search(
                    maze,
                    destination,
                    cost + c,
                    (&point, &direction),
                    history,
                    best_solution,
                )
            },
        )
    }
}

#[allow(clippy::type_complexity)]
fn search_and_collect(
    maze: &Maze,
    destination: &Point,
    cost: u64,
    path: im::Vector<(Point, Direction)>,
    mut history: HashMap<(Point, Direction), u64>,
    bound: Option<u64>,
    points: HashSet<Point>,
) -> (
    HashMap<(Point, Direction), u64>,
    Option<u64>,
    HashSet<Point>,
) {
    let (position, orientation) = path.last().unwrap();

    if position == destination {
        match bound {
            None => {
                let points: HashSet<_> = path.into_iter().map(|(p, _)| p).collect();
                (history, Some(cost), points)
            }
            Some(old) if old > cost => {
                let points: HashSet<_> = path.into_iter().map(|(p, _)| p).collect();
                (history, Some(cost), points)
            }
            Some(old) if old == cost => {
                let points = points.union(path.into_iter().map(|(p, _)| p).collect());
                (history, Some(old), points)
            }
            Some(old) => (history, Some(old), points),
        }
    } else {
        history.insert((*position, *orientation), cost);

        let options = generate_options(
            maze,
            position,
            orientation,
            &history,
            cost,
            bound,
            destination,
        );

        options.into_iter().fold(
            (history, bound, points),
            |(history, bound, points), (next, c)| {
                let mut extended_path = path.clone();
                extended_path.push_back(next);

                search_and_collect(
                    maze,
                    destination,
                    cost + c,
                    extended_path,
                    history,
                    bound,
                    points,
                )
            },
        )
    }
}

fn generate_options(
    maze: &Maze,
    position: &Point,
    orientation: &Direction,
    history: &HashMap<(Point, Direction), u64>,
    cost: u64,
    bound: Option<u64>,
    destination: &Point,
) -> Vec<((Point, Direction), u64)> {
    [
        if matches!(maze.get(&position.step(orientation)), Some(Element::Empty)) {
            Some(((position.step(orientation), *orientation), 1))
        } else {
            None
        },
        if matches!(
            maze.get(&position.step(&orientation.rotate_right())),
            Some(Element::Empty)
        ) {
            Some(((*position, orientation.rotate_right()), 1000))
        } else {
            None
        },
        if matches!(
            maze.get(&position.step(&orientation.rotate_left())),
            Some(Element::Empty)
        ) {
            Some(((*position, orientation.rotate_left()), 1000))
        } else {
            None
        },
    ]
    .into_iter()
    .filter_map(|o| match o {
        Some((o, additional_cost))
            if history.get(&o).map_or(true, |&previous_cost| {
                cost + additional_cost <= previous_cost
            }) && bound.map_or(true, |b| {
                cost + additional_cost + min_cost(o, destination) <= b
            }) =>
        {
            Some((o, additional_cost))
        }
        _ => None,
    })
    .collect()
}

enum Element {
    Wall,
    Empty,
}

impl TryFrom<char> for Element {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '#' => Ok(Element::Wall),
            '.' | 'E' | 'S' => Ok(Element::Empty),
            _ => Err(parse_error(
                "could not parse element",
                &format!("{:?}", value),
            )),
        }
    }
}
