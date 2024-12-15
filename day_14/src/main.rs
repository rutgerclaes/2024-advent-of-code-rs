use std::{collections::HashSet, iter, str::FromStr};

use console::{Key, Term};
use regex::Regex;
use tracing::Level;
use utils::{
    geom::{Point, Vector},
    prelude::*,
};

fn main() -> Result<()> {
    init_tracing();

    let robots: Vec<Robot> = parse_lines()?;

    print_part_1(part_one(&robots));
    print_part_2(part_two(&robots));

    Ok(())
}

struct Robot {
    p: Point<i16>,
    v: Vector<i16>,
}

impl Robot {
    fn simulate(&self, t: u32, boundaries: (i16, i16)) -> Point<i16> {
        let x =
            (self.p.x as i64 + self.v.dx as i64 * t as i64).rem_euclid(boundaries.0 as i64) as i16;
        let y =
            (self.p.y as i64 + self.v.dy as i64 * t as i64).rem_euclid(boundaries.1 as i64) as i16;

        Point::new(x, y)
    }
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let r = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let c = r
            .captures(s)
            .ok_or_else(|| parse_error("could not parse robot", s))?;

        let x: i16 = c[1].parse()?;
        let y: i16 = c[2].parse()?;
        let vx: i16 = c[3].parse()?;
        let vy: i16 = c[4].parse()?;

        Ok(Robot {
            p: Point::new(x, y),
            v: Vector::new(vx, vy),
        })
    }
}

#[tracing::instrument(level=Level::DEBUG,skip(robots))]
fn part_one(robots: &[Robot]) -> Result<usize> {
    let boundaries = (101, 103);

    let division = (boundaries.0 / 2, boundaries.1 / 2);

    tracing::info!("Boundaries: {:?}, division: {:?}", boundaries, division);

    let (a, b, c, d) = robots.iter().map(|r| r.simulate(100, boundaries)).fold(
        (0, 0, 0, 0),
        |(a, b, c, d), point| match point {
            utils::geom::Point { x, y } if x < division.0 && y < division.1 => (a + 1, b, c, d),
            utils::geom::Point { x, y } if x > division.0 && y < division.1 => (a, b + 1, c, d),
            utils::geom::Point { x, y } if x < division.0 && y > division.1 => (a, b, c + 1, d),
            utils::geom::Point { x, y } if x > division.0 && y > division.1 => (a, b, c, d + 1),
            _ => (a, b, c, d),
        },
    );

    tracing::info!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

    Ok(a * b * c * d)
}

#[tracing::instrument(level=Level::DEBUG,skip(robots))]
fn part_two(robots: &[Robot]) -> Result<u32> {
    let boundaries = (101, 103);
    let term = Term::stdout();

    iter::successors(Some(7846), |t| Some(t + 1))
        .find(|t| {
            println!("time: {}", t);
            let pos = robots
                .iter()
                .map(|r| r.simulate(*t, boundaries))
                .collect::<HashSet<_>>();

            render(&pos, boundaries);
            println!("\n\n");

            matches!( term.read_key(), Ok(Key::Enter) )
        })
        .ok_or_else(|| Error::SolutionNotFound("Could not find the solution".to_string()))
}

fn render(positions: &HashSet<Point<i16>>, boundaries: (i16, i16)) {
    for y in 0..boundaries.1 {
        for x in 0..boundaries.0 {
            if positions.contains(&utils::geom::Point::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_simulate() {
        let r = super::Robot {
            p: utils::geom::Point::new(2, 4),
            v: utils::geom::Vector::new(2, -3),
        };

        let boundaries = (11, 7);

        assert_eq!((-2_i32).rem_euclid(7), 5);

        let p = r.simulate(0, boundaries);
        assert_eq!(p, utils::geom::Point::new(2, 4));

        let p = r.simulate(1, boundaries);
        assert_eq!(p, utils::geom::Point::new(4, 1));

        let p = r.simulate(2, boundaries);
        assert_eq!(p, utils::geom::Point::new(6, 5));

        let p = r.simulate(3, boundaries);
        assert_eq!(p, utils::geom::Point::new(8, 2));

        let p = r.simulate(4, boundaries);
        assert_eq!(p, utils::geom::Point::new(10, 6));

        let p = r.simulate(5, boundaries);
        assert_eq!(p, utils::geom::Point::new(1, 3));
    }
}
