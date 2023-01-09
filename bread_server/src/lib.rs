/**
 * lib.rs - this contains the top-level logic of half-baked's server, and handles setup, shutdown, and all the other admin tasks
 */
pub mod database;
use directories::BaseDirs;
use std::io;

// connects to the database
pub fn connect_to_db() -> Result<(), io::Error> {
    if let Some(base_dirs) = BaseDirs::new() {
        let db_path = base_dirs.data_dir().join("half-baked/development.db");
        database::startup::initialize_db(db_path);
    }
    Ok(())
}
