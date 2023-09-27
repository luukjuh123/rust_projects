#[warn(unused_imports)]
use crate::db::conn::DB_CONN;
use crate::models::customer::{Customer, NewCustomer};
use duckdb::params;
use duckdb::Result;

pub fn create_customer(new_customer: NewCustomer) -> Result<()> {
    let conn = (*DB_CONN).lock().unwrap();
    conn.execute(
        "INSERT INTO customer (name, email) VALUES (?, ?)",
        params![new_customer.name, new_customer.email],
    )?;
    Ok(())
}

// pub fn get_customers() -> Result<Vec<Customer>> {
// Similar to the above, implement logic for retrieving customers.
// }
