use itertools::Itertools;
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    iter,
    ops::{Add, Sub},
    str::FromStr,
};

use crate::error::Error;

pub fn find_position<T>(input: &str, needle: char) -> Option<Point<T>>
where
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    input.lines().enumerate().find_map(|(y, l)| {
        l.chars()
            .position(|c| c == needle)
            .map(|x| Point::new(x.try_into().unwrap(), y.try_into().unwrap()))
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    #[inline]
    pub fn step(&self, direction: &Direction) -> Self
    where
        T: Copy
            + num::traits::Zero
            + num::traits::One
            + num::traits::Signed
            + std::ops::Add<Output = T>,
    {
        self.move_by(direction.d().into())
    }

    #[inline]
    pub fn move_by(&self, vector: Vector<T>) -> Self
    where
        T: Copy + Add<Output = T>,
    {
        self.add(&vector)
    }

    #[inline]
    pub fn add(&self, vector: &Vector<T>) -> Self
    where
        T: Copy + Add<Output = T>,
    {
        Point::new(self.x.add(vector.dx), self.y.add(vector.dy))
    }

    #[inline]
    pub fn sub(&self, vector: &Vector<T>) -> Self
    where
        T: Copy + Sub<Output = T>,
    {
        Point::new(self.x.sub(vector.dx), self.y.sub(vector.dy))
    }

    pub fn neighbours(&self) -> impl Iterator<Item = Self> + '_
    where
        T: Copy + num::traits::Zero + num::traits::One + num::traits::Signed,
    {
        Direction::iter().map(|direction| self.step(&direction))
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
        }
    }
}

impl<T> Sub for &Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
        }
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

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

impl<T> Add for &Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl<T> Vector<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Vector { dx, dy }
    }

    pub fn scale(&self, scalar: T) -> Vector<T>
    where
        T: Copy + std::ops::Mul<Output = T>,
    {
        Vector {
            dx: self.dx * scalar,
            dy: self.dy * scalar,
        }
    }
}

impl<T> From<(T, T)> for Vector<T> {
    fn from((dx, dy): (T, T)) -> Self {
        Vector { dx, dy }
    }
}

impl<T> From<Vector<T>> for (T, T) {
    fn from(vector: Vector<T>) -> (T, T) {
        (vector.dx, vector.dy)
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

    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl<T> From<Direction> for Vector<T>
where
    T: num::traits::Zero + num::traits::One + num::traits::Signed,
{
    fn from(value: Direction) -> Self {
        let d: (T, T) = value.d();
        Vector { dx: d.0, dy: d.1 }
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

    pub fn y_iter(&self) -> impl Iterator<Item = T> + '_
    where
        T: num::traits::Zero
            + num::traits::One
            + Add<Output = T>
            + std::cmp::PartialOrd
            + Copy
            + 'static,
    {
        let max = self.max_y;
        iter::successors(Some(T::zero()), |i| Some(i.add(T::one()))).take_while(move |&y| y <= max)
    }

    pub fn x_iter(&self) -> impl Iterator<Item = T> + '_
    where
        T: num::traits::Zero
            + num::traits::One
            + Add<Output = T>
            + std::cmp::PartialOrd
            + Copy
            + 'static,
    {
        let max = self.max_x;
        iter::successors(Some(T::zero()), |i| Some(i.add(T::one()))).take_while(move |&x| x <= max)
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = Point<T>> + '_>
    where
        T: num::traits::Zero
            + num::traits::One
            + Add<Output = T>
            + std::cmp::PartialOrd
            + Copy
            + 'static,
    {
        self.y_iter()
            .map(move |y| self.x_iter().map(move |x| Point::new(x, y)))
    }

    pub fn render<F>(&self, f: F) -> String
    where
        F: Fn(&Point<T>) -> char,
        T: num::traits::Zero
            + num::traits::One
            + Add<Output = T>
            + std::cmp::PartialOrd
            + Copy
            + 'static,
    {
        self.y_iter()
            .map(|y| {
                self.x_iter()
                    .map(|x| f(&Point::new(x, y)))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
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

    pub fn neighbors<'a>(
        &'a self,
        point: &'a Point<T>,
    ) -> impl Iterator<Item = (Point<T>, &'a E)> + 'a
    where
        T: num::traits::Zero + num::traits::One + num::traits::Signed + std::cmp::PartialOrd,
    {
        Direction::iter()
            .map(|direction| point.step(&direction))
            .filter_map(move |neighbor| {
                self.filter(neighbor)
                    .and_then(|p| self.get(&p).map(|e| (p, e)))
            })
    }

    pub fn bbox(&self) -> BBox<T>
    where
        T: std::cmp::PartialOrd + Copy + num::traits::Zero,
    {
        BBox::new(T::zero(), self.max_x, T::zero(), self.max_y)
    }
}

impl<T, E> FromIterator<(Point<T>, E)> for Grid<T, E>
where
    T: num::traits::Zero + std::cmp::PartialOrd + Hash + Eq + Copy,
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

impl<T, E> FromStr for Grid<T, E>
where
    E: TryFrom<char>,
    T: TryFrom<usize> + num::traits::Zero + Hash + Eq + Copy + PartialOrd,
    Error: From<<E as TryFrom<char>>::Error> + From<<T as TryFrom<usize>>::Error>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let point: Point<T> = Point::new(x.try_into()?, y.try_into()?);
                    let elem = E::try_from(c)?;
                    Ok((point, elem))
                })
            })
            .try_collect()
    }
}
