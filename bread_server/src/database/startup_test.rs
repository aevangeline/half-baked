/// startup_test.rs -- makes sure that our startup functions actually work as expected!
#[cfg(test)]
mod startup_test {
    use std::fs;
    use crate::database::startup::initialize_db;

    

    /// create_db_at_top_level - tries to create a functioning database right in our current direction
    #[test]
    fn create_db_at_top_level() {
        assert!(initialize_db("test.db").is_ok(), "couldn't create a test.db and/or run migrations on it");
        assert!(fs::remove_file("test.db").is_ok(), "couldn't clean up test.db!");
    }
    
    /// create_db_in_directory - tries to create a functioning database in a non-extant sub directorhy
    #[test]
    fn create_db_in_directory(){
        assert!(initialize_db("./new_test_dir/test.db").is_ok(), "couldn't create ./new_test_dir/test.db and/or run migrations on it");
        assert!(fs::remove_file("./new_test_dir/test.db").is_ok(), "couldn't clean up ./new_test_dir/test.db");
        assert!(fs::remove_dir("./new_test_dir").is_ok(), "couldn't clean up ./new_test_dir");
    }


}

