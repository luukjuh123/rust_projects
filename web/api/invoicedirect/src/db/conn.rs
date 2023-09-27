use duckdb::Connection;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref DB_CONN: Mutex<Connection> = {
        let conn = Connection::open_in_memory().expect("Failed to open a DB connection");
        Mutex::new(conn)
    };
}
