use serde::{ Serialize };
use crate::schema::customers;

#[derive(Queryable, Serialize)]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name="customers"]
pub struct NewCustomer<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str
}
