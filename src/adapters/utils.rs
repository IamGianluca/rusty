use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn rebuild_database() {
    use std::process::Command;
    Command::new("diesel")
        .arg("migration")
        .arg("redo")
        .arg("--all")
        .output()
        .expect("Something is wrong");
}

fn get_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set.")
}

pub fn get_test_database_connection() -> diesel::PgConnection {
    let database_url = get_database_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_new_test_database_connection() -> diesel::PgConnection {
    rebuild_database();
    let database_url = get_database_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_database_connection() -> diesel::PgConnection {
    let database_url = get_database_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
