use diesel::{Insertable,Queryable};
use serde_derive::{Deserialize,Serialize};
use crate::schema::products

#[derive(Queryable,Serialize,Debug)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub price: f32,
    pub link: String,
}

#[derive(Insertable,Deserialize)]
#[table_name="products"]
pub struct NewProduct{
    pub title: String,
    pub price: f32,
    pub link: String,
}