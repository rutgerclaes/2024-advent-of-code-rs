use itertools::Itertools;
use std::{collections::HashMap, hash::Hash, iter, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>  {

    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn step(&self, direction: &Direction) -> Self
    where
        T: Copy + num::traits::Zero + num::traits::One + num::traits::Signed + std::ops::Add<Output = T>,
    {
        let (dx, dy) = direction.d();
        Point::new(self.x.add(dx), self.y.add(dy))
    }

    pub fn move_by(&self, dx: T, dy: T) -> Self
    where
        T: Copy + std::ops::Add<Output = T>,
    {
        Point::new(self.x.add(dx), self.y.add(dy))
    }

    pub fn neighbours(&self) -> impl Iterator<Item = Self> + '_
    where
        T: Copy + num::traits::Zero + num::traits::One + num::traits::Signed,
    {
        Direction::iter().map( |direction| self.step(&direction))
    }

    pub fn all_neighbours(&self) -> impl Iterator<Item = Self> + '_
    where
        T: Copy + num::traits::Zero + num::traits::One + num::traits::Signed,
    {
            iter::once( self.move_by( T::zero(), -T::one() ))
            .chain( iter::once( self.move_by( T::one(), -T::one() ) ) )
            .chain( iter::once( self.move_by( T::one(), T::zero() ) ))
            .chain( iter::once( self.move_by( T::one(), T::one() )))
            .chain( iter::once( self.move_by( T::zero(), T::one() )))
            .chain( iter::once( self.move_by( -T::one(), T::one() )))
            .chain( iter::once( self.move_by( -T::one(), T::zero() )))
            .chain( iter::once( self.move_by( -T::one(), -T::one() )))
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Point { x, y }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(point: Point<T>) -> (T, T) {
        (point.x, point.y)
    }
}

impl<T> Default for Point<T>
where
    T: Default,
{
    fn default() -> Self {
        Point {
            x: T::default(),
            y: T::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn orientation(&self) -> Orientation {
        match self {
            Direction::Up | Direction::Down => Orientation::Vertical,
            Direction::Left | Direction::Right => Orientation::Horizontal,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        iter::once(Direction::Up)
            .chain(iter::once(Direction::Down))
            .chain(iter::once(Direction::Left))
            .chain(iter::once(Direction::Right))
    }
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn d<T>(&self) -> (T, T)
    where
        T: num::traits::Zero + num::traits::One + num::traits::Signed,
    {
        match self {
            Direction::Up => (T::zero(), -T::one()),
            Direction::Down => (T::zero(), T::one()),
            Direction::Left => (-T::one(), T::zero()),
            Direction::Right => (T::one(), T::zero()),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BBox<T> {
    pub min_x: T,
    pub max_x: T,
    pub min_y: T,
    pub max_y: T,
}

impl<T> BBox<T> {
    pub fn new(min_x: T, max_x: T, min_y: T, max_y: T) -> Self {
        BBox {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

impl<T> BBox<T>
where
    T: std::cmp::PartialOrd + Copy,
{
    pub fn from_point(point: &Point<T>) -> Self {
        BBox {
            min_x: point.x,
            max_x: point.x,
            min_y: point.y,
            max_y: point.y,
        }
    }

    pub fn from_points<I>(points: I) -> Option<Self>
    where
        I: IntoIterator<Item = Point<T>>,
    {
        let mut points = points.into_iter();

        points.next().map(|head| {
            points.fold(BBox::from_point(&head), |mut bbox, point| {
                bbox.extend(&point);
                bbox
            })
        })
    }

    #[inline]
    pub fn contains(&self, point: &Point<T>) -> bool {
        point.x >= self.min_x
            && point.x <= self.max_x
            && point.y >= self.min_y
            && point.y <= self.max_y
    }

    #[inline]
    pub fn filter(&self, point: Point<T>) -> Option<Point<T>> {
        if self.contains(&point) {
            Some(point)
        } else {
            None
        }
    }

    #[inline]
    pub fn extend(&mut self, point: &Point<T>) {
        if point.x < self.min_x {
            self.min_x = point.x;
        }
        if point.x > self.max_x {
            self.max_x = point.x;
        }
        if point.y < self.min_y {
            self.min_y = point.y;
        }
        if point.y > self.max_y {
            self.max_y = point.y;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T, E> {
    locations: HashMap<Point<T>, E>,
    pub max_x: T,
    pub max_y: T,
}

impl<T, E> Default for Grid<T, E>
where
    T: num::traits::Zero,
{
    fn default() -> Self {
        Grid {
            locations: HashMap::new(),
            max_x: T::zero(),
            max_y: T::zero(),
        }
    }
}

impl<T, E> Grid<T, E>
where
    T: Hash + Eq + Copy,
    E: Hash,
{
    pub fn emtpy() -> Self
    where
        T: num::traits::Zero,
    {
        Grid {
            locations: HashMap::new(),
            max_x: T::zero(),
            max_y: T::zero(),
        }
    }

    pub fn contains(&self, point: &Point<T>) -> bool
    where
        T: std::cmp::PartialOrd + num::traits::Zero,
    {
        point.x >= T::zero()
            && point.x <= self.max_x
            && point.y >= T::zero()
            && point.y <= self.max_y
    }

    pub fn filter(&self, point: Point<T>) -> Option<Point<T>>
    where
        T: std::cmp::PartialOrd + num::traits::Zero,
    {
        if self.contains(&point) {
            Some(point)
        } else {
            None
        }
    }

    pub fn get(&self, point: &Point<T>) -> Option<&E> {
        self.locations.get(point)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point<T>, &E)> {
        self.locations.iter()
    }

    pub fn neighbors<'a>(&'a self, point: &'a Point<T>) -> impl Iterator<Item = (Point<T>,&'a E)> + '_ where T: num::traits::Zero + num::traits::One + num::traits::Signed + std::cmp::PartialOrd {
        Direction::iter()
            .map(|direction| point.step(&direction))
            .filter_map(move |neighbor| self.filter( neighbor ).and_then( |p| self.get(&p).map(|e| (p, e)) ))
    }
}

impl<T, E> FromIterator<(Point<T>, E)> for Grid<T, E>
where
    T: num::traits::Zero + std::cmp::PartialOrd + Hash + Eq + Copy,
    E: Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Point<T>, E)>,
    {
        iter.into_iter()
            .fold(Grid::emtpy(), |mut grid, (point, elem)| {
                if grid.max_x < point.x {
                    grid.max_x = point.x;
                };
                if grid.max_y < point.y {
                    grid.max_y = point.y;
                };
                grid.locations.insert(point, elem);
                grid
            })
    }
}

impl<I, T> FromStr for Grid<I, T>
where
    T: Hash + TryFrom<char>,
    I: From<u16> + num::traits::Zero + Hash + Eq + Copy + PartialOrd,
{
    type Err = <T as TryFrom<char>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let point: Point<I> = Point::new((x as u16).into(), (y as u16).into());
                    let elem = T::try_from(c)?;
                    Ok((point, elem))
                })
            })
            .try_collect()
    }
}
