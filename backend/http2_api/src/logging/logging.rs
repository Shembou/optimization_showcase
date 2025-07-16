use std::{io, path::Path};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init_tracing(log_dir: &str, file_prefix: &str) -> io::Result<()> {
    std::fs::create_dir_all(Path::new(log_dir))?;
    let file_appender = rolling::daily(log_dir, format!("{file_prefix}.log"));
    let (file_writer, _guard) = non_blocking(file_appender); // _guard must live to flush on drop

    let timer = fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S".into());

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_timer(timer.clone())
        .with_writer(file_writer);

    let stdout_layer = fmt::layer()
        .with_timer(timer)
        .with_writer(io::stdout)
        .with_filter(EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(file_layer)
        .with(stdout_layer)
        .init();

    Ok(())
}
