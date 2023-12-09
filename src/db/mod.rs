use core::panic;

use diesel::{SqliteConnection, Connection};

pub fn connection() -> SqliteConnection {
    SqliteConnection::establish("db.sql").unwrap_or_else(|_| panic!("Error connecting to db"))
}