/// database - This module exposes a clean CRUD interface that lets the rest of the half-baked todo timer app
/// For the purposes of this application, SQLite was chosen as the data storage engine due to the low overhead and reliability
pub mod models;
pub mod startup;

// here's all our testing modules!
mod startup_test;
