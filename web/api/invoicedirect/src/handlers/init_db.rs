use crate::db::conn::DB_CONN;

#[tokio::main]
pub async fn init_db() -> Result<()> {
    let conn = DB_CONN.lock()?;
    conn.execute_batch(r"CREATE TABLE IF NOT EXISTS customer (id INTEGER PRIMARY KEY, name TEXT, email TEXT);")
}
