use itertools::Itertools;
use std::fmt::Display;
use std::iter;
use tracing::Level;
use tramp::{rec_call, rec_ret, tramp, BorrowRec, Rec};
use utils::prelude::*;

fn main() -> Result<()> {
    init_tracing();

    let blocks: Vec<Block> = read_input()?
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| parse_error("could not parse digit", &format!("{c}")))
        })
        .scan((true, 0), |acc, c| {
            let res = match c {
                Ok(c) if acc.0 => Ok(Block::File(acc.1, c as usize)),
                Ok(c) => Ok(Block::Empty(c as usize)),
                Err(e) => Err(e),
            };

            *acc = (!acc.0, acc.1 + if !acc.0 { 1 } else { 0 });
            Some(res)
        })
        .try_collect()?;

    print_part_1(part_one(&blocks));
    print_part_2(part_two(&blocks));

    Ok(())
}

#[tracing::instrument(level=Level::DEBUG,skip(blocks))]
fn part_one(blocks: &[Block]) -> Result<u64> {
    let result = tramp(repartition(
        blocks,
        0,
        None,
        blocks.len() - 1,
        None,
        Vec::new(),
    ));
    Ok(calculate_checksum(&result))
}

fn calculate_checksum(blocks: &[Block]) -> u64 {
    blocks
        .iter()
        .flat_map(|b| b.iter())
        .enumerate()
        .fold(0, |acc, (i, c)| acc + (i as u64) * (c as u64))
}

fn repartition(
    source: &[Block],
    start: usize,
    blank_size: Option<usize>,
    end: usize,
    file_size: Option<usize>,
    mut destination: Vec<Block>,
) -> BorrowRec<Vec<Block>> {
    if start >= end {
        if let Some(left_over) = file_size {
            match source.get(end) {
                Some(Block::File(c, _)) => destination.push(Block::File(*c, left_over)),
                _ => unreachable!(),
            };
        }
        rec_ret!(destination)
    } else if let Some(blank_size) = blank_size {
        if blank_size == 0 {
            rec_call!(repartition(
                source,
                start + 1,
                None,
                end,
                file_size,
                destination
            ))
        } else {
            match source.get(end) {
                Some(Block::Empty(_)) => {
                    rec_call!(repartition(
                        source,
                        start,
                        Some(blank_size),
                        end - 1,
                        None,
                        destination
                    ))
                }
                Some(Block::File(c, size)) => {
                    let file_size = file_size.unwrap_or(*size);
                    if file_size == 0 {
                        rec_call!(repartition(
                            source,
                            start,
                            Some(blank_size),
                            end - 1,
                            None,
                            destination
                        ))
                    } else {
                        let move_size = file_size.min(blank_size);
                        destination.push(Block::File(*c, move_size));
                        rec_call!(repartition(
                            source,
                            start,
                            Some(blank_size - move_size),
                            end,
                            Some(file_size - move_size),
                            destination,
                        ))
                    }
                }
                None => unreachable!(),
            }
        }
    } else {
        tracing::debug!("no blank_size, checking source at {}", start);
        match &source.get(start) {
            Some(f @ Block::File(_, _)) => {
                destination.push(**f);
                rec_call!(repartition(
                    source,
                    start + 1,
                    None,
                    end,
                    file_size,
                    destination
                ))
            }
            Some(Block::Empty(size)) => {
                let size = *size;
                rec_call!(repartition(
                    source,
                    start,
                    Some(size),
                    end,
                    file_size,
                    destination
                ))
            }
            None => {
                unreachable!();
            }
        }
    }
}

#[tracing::instrument(level=Level::DEBUG,skip(blocks))]
fn part_two(blocks: &[Block]) -> Result<u64> {
    let result = tramp(reorder(
        blocks.len() - 1,
        blocks.to_vec(),
    ));
    Ok(calculate_checksum(&result))
}

fn reorder(i: usize, mut blocks: Vec<Block>) -> Rec<Vec<Block>> {
    match blocks.get(i) {
        Some(Block::Empty(_)) => {
            rec_call!(reorder(i - 1, blocks))
        }
        Some(&f @ Block::File(_, size)) => {
            let j = blocks
                .iter()
                .take(i)
                .enumerate()
                .find_map(|(j, b)| match b {
                    Block::Empty(empty_space) if empty_space >= &size => Some(j),
                    _ => None,
                });

            if let Some(j) = j {
                blocks.insert(j, f);
                blocks.remove(i + 1);

                let mut next = i;

                match blocks.get_mut(i) {
                    Some(Block::Empty(empty_space)) => {
                        *empty_space += size;
                        next -= 1
                    }
                    Some(_) => {
                        blocks.insert(i + 1, Block::Empty(size));
                    }
                    None => unreachable!(),
                }

                let empty = blocks.remove(j + 1);
                if empty.len() > size {
                    blocks.insert(j + 1, Block::Empty(empty.len() - size));
                } else {
                    next -= 1;
                }

                rec_call!(reorder(next, blocks))
            } else if i > 0 {
                rec_call!(reorder(i - 1, blocks))
            } else {
                rec_ret!(blocks)
            }
        }
        None => rec_ret!(blocks),
    }
}

#[derive(Clone, Copy, Debug)]
enum Block {
    Empty(usize),
    File(usize, usize),
}

impl Block {
    fn char(&self) -> char {
        match self {
            Self::Empty(_) => '.',
            Self::File(c, _) => std::char::from_digit(*c as u32, 10).unwrap(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Empty(size) => *size,
            Self::File(_, size) => *size,
        }
    }

    fn chars(&self) -> impl Iterator<Item = char> {
        iter::repeat(self.char()).take(self.len())
    }

    fn iter(&self) -> impl Iterator<Item = usize> {
        match self {
            Self::Empty(size) => iter::repeat(0).take(*size),
            Self::File(c, size) => iter::repeat(*c).take(*size),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chars().join(""))
    }
}
