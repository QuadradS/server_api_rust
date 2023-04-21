use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::schema::rustoceans;

#[derive(Serialize, Queryable)]
pub struct Rustocean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String
}


#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustoceans)]
pub struct CreateRustocean {
    pub name: String,
    pub email: String,
}