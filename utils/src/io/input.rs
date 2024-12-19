use crate::error::{Error, Result};
use err_into::ErrorInto;
use itertools::Itertools;
use std::result::Result as StdResult;
use std::{
    io::{self, Read},
    str::FromStr,
};

pub fn read_input() -> StdResult<String, io::Error> {
    let input = match std::env::args().nth(1) {
        Some(file) if !file.starts_with('-') => {
            tracing::debug!(file = file, "Reading input from file");
            std::fs::read_to_string(file).err_into()
        }
        _ => {
            tracing::debug!("Reading input from stdin");
            let mut input = String::new();
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut input)?;
            Ok(input)
        }
    };

    if tracing::enabled!(tracing::Level::DEBUG) {
        if let Ok(input) = input.as_ref() {
            tracing::debug!(
                "Read {} bytes in {} lines",
                input.len(),
                input.lines().count()
            );
        }
    }

    input
}

pub fn read_lines<I>() -> StdResult<I, io::Error>
where
    I: FromIterator<String>,
{
    read_input().map(|input| input.lines().map(str::to_string).collect())
}

pub fn parse_lines<T, I>() -> Result<I>
where
    T: FromStr,
    I: FromIterator<T>,
    T::Err: Into<Error>,
{
    read_input()
        .err_into()
        .and_then(|input| input.lines().map(|l| l.parse().err_into()).try_collect())
}
