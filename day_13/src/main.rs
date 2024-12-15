use itertools::Itertools;
use regex::Regex;
use tracing::Level;
use utils::prelude::*;

type Button = utils::geom::Vector<i64>;
type Prize = utils::geom::Point<i64>;

fn main() -> Result<()> {
    init_tracing();

    let regex_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_p = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let input = read_input()?;
    let arcades: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .tuples()
        .map(|(line_a, line_b, line_prize)| {
            let a = regex_a
                .captures(line_a)
                .ok_or_else(|| parse_error("could not parse button a", line_a))?;
            let b = regex_b
                .captures(line_b)
                .ok_or_else(|| parse_error("could not parse button b", line_b))?;
            let p = regex_p
                .captures(line_prize)
                .ok_or_else(|| parse_error("could not parse prize", line_prize))?;

            Ok::<_, Error>(Arcade {
                a: Button::new(a[1].parse()?, a[2].parse()?),
                b: Button::new(b[1].parse()?, b[2].parse()?),
                prize: Prize::new(p[1].parse()?, p[2].parse()?),
            })
        })
        .try_collect()?;

    print_part_1(part_one(&arcades));
    print_part_2(part_two(&arcades));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(arcades))]
fn part_one(arcades: &[Arcade]) -> Result<u64> {
    Ok(arcades
        .iter()
        .filter_map(|a| a.solve())
        .map(|(na, nb)| na * 3 + nb)
        .sum())
}

#[tracing::instrument(level=Level::DEBUG,skip(arcades))]
fn part_two(arcades: &[Arcade]) -> Result<u64> {
    Ok(arcades
        .iter()
        .cloned()
        .filter_map(|a| a.supercharge().solve())
        .map(|(na, nb)| na * 3 + nb)
        .sum())
}

#[derive(Debug, Clone)]
struct Arcade {
    a: Button,
    b: Button,
    prize: Prize,
}

impl Arcade {
    fn supercharge(self) -> Self {
        Arcade {
            prize: Prize::new(self.prize.x + 10000000000000, self.prize.y + 10000000000000),
            ..self
        }
    }

    fn solve(&self) -> Option<(u64, u64)> {
        let ax = self.a.dx as f64;
        let ay = self.a.dy as f64;
        let bx = self.b.dx as f64;
        let by = self.b.dy as f64;
        let px = self.prize.x as f64;
        let py = self.prize.y as f64;

        let b = (py / by - (ay * px) / (by * ax)) / (1f64 - (ay * bx) / (by * ax));
        if b < 0f64 {
            return None;
        }

        tracing::debug!("b = {}", b);
        let a = (px - bx * b) / ax;
        tracing::debug!("a = {}", a);

        if a < 0f64 {
            return None;
        }

        let a = a.round() as u64;
        let b = b.round() as u64;

        if a as i64 * self.a.dx + b as i64 * self.b.dx == self.prize.x
            && a as i64 * self.a.dy + b as i64 * self.b.dy == self.prize.y
        {
            Some((a, b))
        } else {
            None
        }
    }
}
