use crate::error::Result;
use err_into::ErrorInto;
use std::io::Read;

pub fn read_input() -> Result<String> {
    let input = match std::env::args().nth(1) {
        Some(file) if file != "-" => {
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
