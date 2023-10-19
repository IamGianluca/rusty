use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn rebuild_db() {
    use std::process::Command;
    Command::new("diesel")
        .arg("migration")
        .arg("redo")
        .arg("--all")
        .output()
        .expect("Something is wrong");
}

fn get_db_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set.")
}

pub fn init_db() -> diesel::PgConnection {
    rebuild_db();
    let database_url = get_db_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_db_conn() -> diesel::PgConnection {
    let database_url = get_db_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
