use crate::error::Result;

pub trait ExampleSettings {
    fn example<F>(f: F) -> Self
    where
        F: FnOnce() -> Self;

    fn try_example<F>(f: F) -> Result<Self>
    where
        Self: Sized,
        F: FnOnce() -> Result<Self>;
}

impl<D> ExampleSettings for D
where
    D: Default,
{
    fn example<F>(f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        let mut pargs = pico_args::Arguments::from_env();
        if pargs.contains("--example") {
            f()
        } else {
            Default::default()
        }
    }

    fn try_example<F>(f: F) -> Result<Self>
    where
        Self: Sized,
        F: FnOnce() -> Result<Self>,
    {
        let mut pargs = pico_args::Arguments::from_env();
        if pargs.contains("--example") {
            f()
        } else {
            Ok(Default::default())
        }
    }
}
