// src/models/customer.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
}
