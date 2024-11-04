use chrono::Local;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

fn get_level_color(level: log::Level) -> &'static str {
    match level {
        log::Level::Error => "\x1B[31m", // Red
        log::Level::Warn => "\x1B[33m",  // Yellow
        log::Level::Info => "\x1B[32m",  // Green
        log::Level::Debug => "\x1B[36m", // Cyan
        log::Level::Trace => "\x1B[35m", // Magenta
    }
}

pub fn init_logger() {
    let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
    
    let mut builder = Builder::new();
    builder.target(Target::Stdout);
    
    // Set specific log levels for different modules
    builder
        .filter(None, if env == "production" { LevelFilter::Info } else { LevelFilter::Debug })
        // Suppress html5ever debug logs
        .filter_module("html5ever", LevelFilter::Info)
        .filter_module("selectors", LevelFilter::Info)
        .format(move |buf, record| {
            let level_color = get_level_color(record.level());
            let reset_color = "\x1B[0m";
            
            // Get thread name or id
            let thread_name = std::thread::current()
                .name()
                .map(|s| s.to_owned())
                .unwrap_or_else(|| format!("{:?}", std::thread::current().id()));

            writeln!(
                buf,
                "{timestamp} {colored_level} {thread} {target} → {message}",
                timestamp = Local::now().format("\x1B[34m%Y-%m-%d %H:%M:%S%.3f\x1B[0m"),
                colored_level = format!(
                    "{color}{: <5}{reset}",
                    record.level().to_string(),
                    color = level_color,
                    reset = reset_color,
                ),
                thread = format!("\x1B[35m[{thread_name}]\x1B[0m"),
                target = format!(
                    "\x1B[33m{module}{line}\x1B[0m",
                    module = record.module_path().unwrap_or("unknown"),
                    line = record.line().map_or(String::new(), |l| format!(":{}", l))
                ),
                message = record.args(),
            )
        })
        .init();
} 