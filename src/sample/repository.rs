#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::Post;
use crate::schema::posts;
use crate::schema::posts::dsl::*;

pub fn create_post(new_post: NewPost, conn: &PgConnection) -> QueryResult<Post> {
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}

pub fn show_posts(connection: &PgConnection) -> QueryResult<Vec<Post>>  {
    //posts.filter(published.eq(true))
    posts.limit(5)
        .load::<Post>(&*connection)
}

#[table_name="posts"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}








