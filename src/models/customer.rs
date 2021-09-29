use serde::{ Serialize, Deserialize };
use crate::schema::customers;

#[derive(Queryable, Serialize)]
#[derive(Debug)]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub active: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="customers"]
pub struct NewCustomer {
    pub first_name: String,
    pub last_name: String
}
