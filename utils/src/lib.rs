pub mod error;

pub mod io {
    pub mod input;
    pub mod output;
}

pub mod prelude {
    pub use crate::error::{parse_error, Error, Result};
    pub use crate::io::input::read_input;
    pub use crate::io::output::{init_tracing, print_part_1, print_part_2};
}
