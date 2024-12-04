use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::result::Result as StdResult;
use std::usize;
use tracing::Level;
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let input = read_input()?;
    let grid = LetterGrid::new( &input );
    print_part_1(part_one(&grid));
    print_part_2(part_two(&input));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(grid))]
fn part_one(grid: &LetterGrid) -> StdResult<usize, Infallible> {
    let word:[char;4]    = ['X','M','A','S'];
    Ok( grid.count_word( &word ) )
}

#[tracing::instrument(level=Level::DEBUG,skip(input))]
fn part_two(input: &str) -> StdResult<usize, Infallible> {
    let grid = LetterGrid::new( input );
    Ok( grid.count_crosses())
}

#[allow(unused)]
fn print_xmas( grid: &LetterGrid, word: &[char;4] ) {
    let positions = grid.find_word::<4,Vec<_>>( &word );

    let check: HashSet<_> = positions.iter().flatten().collect();
    let xmas_chars:HashMap<_,_> = grid.chars.iter().filter( |((x,y),_)| check.contains(&(*x,*y)) ).collect();

    for y in 0..=grid.max_y {
        for x in 0..=grid.max_x {
            print!( "{}", xmas_chars.get( &(x,y) ).unwrap_or(&&'.') );
        }
        println!();
    }
}

struct LetterGrid { chars: HashMap<(usize,usize),char>, max_x: usize, max_y: usize }

impl LetterGrid {
    
    fn new( input: &str ) -> Self {
        let chars: HashMap<(usize,usize),char> = input.lines().enumerate().flat_map(|(y,line)| {
            line.chars().enumerate().map(move |(x,c)| {
                ((x,y), c)
            })
        } ).collect();

        let max_x = *chars.keys().map(|(x,_)| x).max().unwrap();
        let max_y = *chars.keys().map(|(_,y)| y).max().unwrap();

        Self { chars, max_x, max_y }
    }

    fn get( &self, x: usize, y: usize ) -> Option<&char> {
        self.chars.get(&(x,y))
    }

    fn check_word<const N: usize>( &self, positions: &[(usize,usize);N], filter: &[char;N] ) -> bool {
        positions.iter().zip(filter).all(|((x,y),c)| self.get(*x,*y) == Some(c))
    }

    fn check_mas_cross( &self, position: &(usize,usize) ) -> bool {
        let (x,y) = *position;
         
        if x > 0 && y > 0 && x < self.max_x && y < self.max_y && self.get(x,y).unwrap() == &'A' {
            let up_left = *self.get(x-1,y-1).unwrap();
            let up_right = *self.get(x+1,y-1).unwrap();
            let down_left = *self.get(x-1,y+1).unwrap();
            let down_right = *self.get(x+1,y+1).unwrap();

            let a = ( up_left == 'M' && down_right == 'S' ) || ( up_left == 'S' && down_right == 'M' );
            let b = ( up_right == 'M' && down_left == 'S' ) || ( up_right == 'S' && down_left == 'M' );

            a && b
        } else { false }
    }

    fn count_crosses( &self ) -> usize {
        self.chars.keys().filter( |pos| self.check_mas_cross( pos ) ).count()
    }

    fn directions<F,const N: usize>( &self, position: &(usize,usize) ) -> F where F: FromIterator<[(usize,usize);N]> {
        let (x,y) = *position;
        
        let up: Option<[(usize,usize);N]> = if y >= N - 1 {
            let mut a = [(x,y);N];
            (0..N).for_each( |i| a[i] = (a[i].0,a[i].1 - i) );
            Some( a )
        } else { None };

        let down: Option<[(usize,usize);N]> = if y + N <= self.max_y + 1 {
            let mut a = [(x,y);N];
            (0..N).for_each( |i| a[i] = (a[i].0,a[i].1 + i) );
            Some( a )
        } else { None };
 
        let left: Option<[(usize,usize);N]> = if x >= N - 1 {
            let mut a = [(x,y);N];
            (0..N).for_each( |i| a[i] = (a[i].0-i,a[i].1) );
            Some( a )
        } else { None };

        let right: Option<[(usize,usize);N]> = if x + N <= self.max_x + 1{
            let mut a = [(x,y);N];
            (0..N).for_each( |i| a[i] = (a[i].0 + i,a[i].1) );
            Some( a )
        } else { None };

        let up_left: Option<[(usize,usize);N]> = if x >= N - 1 && y >= N - 1 {
            let mut a = [(x,y);N];
            (0..N).for_each( |i| a[i] = (a[i].0-i,a[i].1-i) );
            Some( a )
        } else { None };

        let up_right: Option<[(usize,usize);N]> = if x + N <= self.max_x + 1 && y >= N-1{
            let mut a = [(x,y);N];
            (0..N).for_each( |i| a[i] = (a[i].0+i,a[i].1-i) );
            Some( a )
        } else { None };

        let down_left: Option<[(usize,usize);N]> = if x >= N - 1 && y + N <= self.max_y + 1{
            let mut a = [(x,y);N];
            (0..N).for_each( |i| a[i] = (a[i].0-i,a[i].1+i) );
            Some( a )
        } else { None };

        let down_right: Option<[(usize,usize);N]> = if x + N <= self.max_x + 1 && y + N <= self.max_y + 1{
            let mut a = [(x,y);N];
            (0..N).for_each( |i| a[i] = (a[i].0+i,a[i].1+i) );
            Some( a )
        } else { None };

        vec![ up, down, left, right, up_left, up_right, down_left, down_right ].into_iter().filter_map( |x| x ).collect()
    }

    #[allow(unused)]
    fn find_word<const N:usize,C>( &self, word: &[char;N] ) -> C where C: FromIterator<[(usize,usize);N]> {
        self.chars.keys().flat_map( |pos| {
            let dirs:Vec<_> = self.directions::<_,N>( pos );
            dirs.into_iter().filter( |dir| self.check_word( dir, word ) )
        } ).collect()
    }

    fn count_word<const N:usize>( &self, word: &[char;N] ) -> usize {
        self.chars.keys().map( |p| {
            let d: Vec<_> = self.directions(p);
            d.iter().filter( |dir| self.check_word( dir, word ) ).count()
        } ).sum()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const XMAS: [char;4] = ['X','M','A','S'];

    #[test]
    fn test_part_one() {
        let grid = LetterGrid::new("XMAS\nM..A\nA..M\nSAMX");
        assert_eq!( grid.count_word( &XMAS ), 4);
    }

    #[test]
    fn test_directions() {
        let grid = LetterGrid::new("XMAS\nM..A\nA..M\nSAMX");
        
        let top_left: Vec<_> = grid.directions::<_,4>( &(0,0) );
        assert_eq!( top_left, vec![ [(0,0),(0,1),(0,2),(0,3)], [(0,0),(1,0),(2,0),(3,0)],[(0,0),(1,1),(2,2),(3,3)]] );
    }

    #[test]
    fn test_check_cross() {
        let grid = LetterGrid::new("M.M\n.A.\nS.S");
        assert_eq!( grid.check_mas_cross( &(1,1) ), true );
        assert_eq!( grid.check_mas_cross( &(1,2) ), false );
    }
}