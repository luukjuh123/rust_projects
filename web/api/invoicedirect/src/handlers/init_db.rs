use crate::db::conn::DB_CONN;

#[tokio::main]
pub async fn init_db() -> Result<(), duckdb::Error> {
    let lock = DB_CONN.lock().map_err(|_| Error("Mutex is poisoned"))?;
    let conn = &*lock;
    conn.execute_batch(r"CREATE TABLE IF NOT EXISTS customer (id INTEGER PRIMARY_KEY, name TEXT, email TEXT)")
        .map_err(|e| e);

    Ok(())
}