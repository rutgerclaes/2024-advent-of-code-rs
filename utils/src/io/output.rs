use console::style;
use time::macros::format_description;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::LocalTime},
    EnvFilter,
};

pub fn init_tracing() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::ERROR.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(std::io::stderr)
        .compact()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_timer(LocalTime::new(format_description!(
            "[hour]:[minute]:[second]"
        )))
        .with_env_filter(filter)
        .init();
}

pub fn print_solution<D, E>(prefix: &str, solution: Result<D, E>)
where
    D: std::fmt::Display,
    E: std::fmt::Display,
{
    match solution {
        Ok(value) => {
            if tracing::enabled!(tracing::Level::INFO) {
                tracing::info!("Solution to {}: {}", prefix, value);
            } else {
                println!(
                    "Solution to {}: {}",
                    style(prefix).bold().bright(),
                    style(value).bold().green()
                );
            }
        }
        Err(error) => {
            if tracing::enabled!(tracing::Level::ERROR) {
                tracing::error!( error = %error, "Error calculating {}", prefix)
            } else {
                eprintln!(
                    "Error calculating {}: {}",
                    style(prefix).bold(),
                    style(error).bold().red()
                )
            }
        }
    }
}

pub fn print_part_1<D, E>(solution: Result<D, E>)
where
    D: std::fmt::Display,
    E: std::fmt::Display,
{
    print_solution("Part 1", solution);
}

pub fn print_part_2<D, E>(solution: Result<D, E>)
where
    D: std::fmt::Display,
    E: std::fmt::Display,
{
    print_solution("Part 2", solution);
}
