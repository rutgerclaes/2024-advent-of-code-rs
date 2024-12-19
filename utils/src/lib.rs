pub mod error;

pub mod io {
    pub mod input;
    pub mod output;
}

pub mod config;

pub mod geom;

pub mod prelude {
    pub use crate::config::ExampleSettings;
    pub use crate::error::{parse_error, Error, Result};
    pub use crate::io::input::{parse_lines, read_input, read_lines};
    pub use crate::io::output::{init_tracing, print_part_1, print_part_2};
}
