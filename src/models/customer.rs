use super::super::schema::*;
// use crate::schema::*;

#[derive(Queryable)]
pub struct Customer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String
}

#[derive(Insertable)]
#[table_name="customers"]
pub struct NewCustomer<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str
}
