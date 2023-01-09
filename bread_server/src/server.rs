/// server.rs - the main file for our server that handles all the logic of half-baked
/// This file is supposed to stay as simple as possible!
use bread_server::connect_to_db;
use std::io;
fn main() -> Result<(), io::Error> {
    connect_to_db()?;
    println!("Hello, World!");
    Ok(())
}
