use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;
use crate::sample::Post;
use crate::sample::repository::NewPost;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Post>>, Status> {
    sample::repository::show_posts(&connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_post>")]
pub fn post(new_post: Json<NewPost>, connection: DbConn) ->  Result<status::Created<Json<Post>>, Status> {
    println!("here 0 {}",&new_post.title);
    sample::repository::create_post(new_post.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))

} 

fn post_created(post: Post) -> status::Created<Json<Post>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/post/{id}", host = host(), port = port(), id = post.id).to_string(),
        Some(Json(post)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}