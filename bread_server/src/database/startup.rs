use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
/// database::startup - this module handles all of the database startup tasks that have to be run at program start
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::sync::Arc;
/// MIGRATIONS is a compile-time pack-in of all pending migrations for this application, this will allow you to
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// initialize_db - creates the db file if it doesn't already exist and run migrations if we need to!
/// Arguments
///     db_path : AsRef<Path> -> a path-like object which points
///            to either an existing sqlite database OR a valid place we can create one
///
pub fn initialize_db<P: AsRef<Path>>(db_path: P) -> Result<Arc<SqliteConnection>, Error> {
    // if a file already exists, then we're all ok!
    let p = db_path.as_ref();

    // try to fish out a parent directory for the target db file
    let _ = p
        .parent()
        .ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("cannot find a containing directory for the db : {:?}", p),
            )
        })
        .map(fs::create_dir_all)?; // Try to create the containing directory for our database, return an error if we fail

    // we have to convert the Path to a string to use it with diesel to create a connection
    let mut conn = Arc::new(
        p.to_str()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "cannot create a string version of the path to the db : {:?}",
                        p
                    ),
                )
            })
            .and_then(|path| {
                SqliteConnection::establish(path) // We try to create the connection here and fail over into an io:Error if we can't
                    .map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidInput,
                            format!("cannot create or connect to db : {:?}", p),
                        )
                    })
            })?,
    );
    let mut bare_conn = &mut *Arc::get_mut(&mut conn).unwrap();

    bare_conn.run_pending_migrations(MIGRATIONS).map_err(|_| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("migrations failed on db! : {:?}", p),
        )
    })?;

    Ok(conn)
}
