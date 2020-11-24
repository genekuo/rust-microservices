use std::fmt;
use std::sync::{Arc, Mutex};
use slab::Slab;
use lazy_static::lazy_static;
use futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;
use regex::Regex;

type UserId = u64;

struct UserData;

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{}")
    }
}

type UserDb = Arc<Mutex<Slab<UserData>>>;

lazy_static! {
    static ref USER_PATH: Regex = Regex::new("^/user/((?P<user_id>\\d+?)/?)?$").unwrap();
    static ref USERS_PATH: Regex = Regex::new("^/users/?$").unwrap();
}

fn user_handler(req: Request<Body>, user_db: &UserDb)
    -> impl Future<Item=Response<Body>, Error=Error> {

        let response = {
            let method = req.method();
            let path = req.uri().path();
            let mut users = user_db.lock().unwrap();
            if USERS_PATH.is_match(path) {
                if method == &Method::GET {
                    let list = users.iter()
                        .map(|(id, _)| id.to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    Response::new(list.into())
                } else {
                    response_status(StatusCode::METHOD_NOT_ALLOWED)
                }
            } else if let Some(cap) = USER_PATH.captures(path) {
                let user_id = cap.name("user_id").and_then(|m| {
                    m.as_str()
                     .parse::<UserId>()
                     .ok()
                     .map(|x| x as usize)
                });
                match (method, user_id) {
                    (&Method::GET, Some(id)) => {
                        if let Some(data) = users.get(id) {
                            Response::new(data.to_string().into())
                        } else {
                            response_status(StatusCode::NOT_FOUND)
                        }
                    },
                    (&Method::POST, None) => {
                        let id = users.insert(UserData);
                        Response::new(id.to_string().into())
                    },
                    (&Method::POST, Some(_)) => {
                        response_status(StatusCode::BAD_REQUEST)
                    },
                    (&Method::PUT, Some(id)) => {
                        if let Some(user) = users.get_mut(id) {
                            *user = UserData;
                            response_status(StatusCode::OK)
                        } else {
                            response_status(StatusCode::NOT_FOUND)
                        }
                    },
                    (&Method::DELETE, Some(id)) => {
                        if users.contains(id) {
                            users.remove(id);
                            response_status(StatusCode::OK)
                        } else {
                            response_status(StatusCode::NOT_FOUND)
                        }
                    },
                    _ => {
                        response_status(StatusCode::METHOD_NOT_ALLOWED)
                    },
                }
            } else {
                response_status(StatusCode::NOT_FOUND)
            }
        };
        future::ok(response)
    }

fn response_status(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}

fn main() {
    let addr = ([0, 0, 0, 0], 8080).into();
    let builder = Server::bind(&addr);
    let user_db: UserDb = Arc::new(Mutex::new(Slab::new()));
    let server = builder.serve(move || {
        let user_db = user_db.clone();
        service_fn(move |req| user_handler(req, &user_db))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}
