/// database::startup - this module handles all of the database startup tasks that have to be run at program start
use diesel_migrations::embed_migrations;
use diesel::prelude::SQLiteConnection;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
/// MIGRATIONS is a compile-time pack-in of all pending migrations for this application, this will allow you to 
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// initialize_db - creates the db file if it doesn't already exist and run migrations
/// Arguments 
///     path : AsRef<Path> -> a path-like object which points
///            to either an existing sqlite database OR a valid place we can create one
pub fn initialize_db<P: AsRef<Path>>(db_path: P) -> Result<()> { 
    // if a file already exists, then we're all ok!
    if (db_path.is_file()){
        return Ok(());
    }
    // if something else is there besides a file, we have a problem and have to bail out
    if(db_path.exists()){
        return Err(Error(ErrorKind::InvalidFilename));
    }

    // try to create the path that the db file will live under
    fs::create_dir_all(db_path.parent())?;
    // create a connection the database, and if it doesn't exist then Sqlite will make one
    let conn = SQLiteConnection::establish(db_path)?;
    // run any migrations that need to be processsed on the db
    conn.run_pending_migrations(MIGRATIONS)?;
    return Ok(());
}