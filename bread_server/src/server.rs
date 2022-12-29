/// server.rs - the main file for our server that handles all the logic of half-baked
/// This file is supposed to stay as simple as possible!
mod database;

use database::startup::initialize_db;
use directories::BaseDirs;
use dotenvy::dotenv;
use std::env;
use std::path::Path;

fn main() {
    dotenv().ok();
    let base_dir = env::var("DATABSE_URL").unwrap_or(
        BaseDirs::new()
            .map(|base_dirs| base_dirs.data_dir().display().to_string())
            .unwrap(),
    );
    let database_url = Path::new(&base_dir).join("development.db");
    match initialize_db(&database_url) {
        Ok(_) => println!("Successfully created database!"),
        Err(e) => println!("Failed with error : {}", e),
    }
}
