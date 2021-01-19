#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::model::Post;
use crate::sample::model::NewPost;

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

pub fn get_post(post_id: i32, connection: &PgConnection) -> QueryResult<Post> {
    posts::table.find(post_id).get_result::<Post>(connection)
}

pub fn update_post(post_id: i32, post: Post, connection: &PgConnection) -> QueryResult<Post> {
    diesel::update(posts::table.find(post_id))
        .set(&post)
        .get_result(connection)
}

pub fn delete_post(post_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(posts::table.find(post_id))
        .execute(connection)
}
