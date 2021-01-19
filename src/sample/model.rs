#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::posts;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
