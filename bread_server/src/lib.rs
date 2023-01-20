/// lib.rs - this contains the top-level logic of half-baked's server, and handles setup, shutdown, and all the other admin tasks
pub mod database;
use directories::BaseDirs;
use env_logger;
use log;
use log::LevelFilter;
use std::io;
use std::io::{Error, ErrorKind};

// connects to the database
pub fn connect_to_db() -> Result<(), io::Error> {
    if let Some(base_dirs) = BaseDirs::new() {
        let db_path_buf = base_dirs.data_dir().join("half-baked/development.db");
        let db_path = db_path_buf.as_path();
        database::startup::initialize_db(db_path)?;
        log::info!(
            "Connected to DB @ {}",
            db_path.to_str().unwrap_or("No Path Found")
        );
        return Ok(());
    }
    log::error!("No available system-defined data directory, cannot open database");
    Err(Error::from(ErrorKind::NotFound))
}

/// setups up the logger for the package
pub fn setup_logger() {
    let mut builder = env_logger::Builder::new();
    builder.filter_level(LevelFilter::Warn);
    log::info!("Initialized Logging");
    builder.init();
}
