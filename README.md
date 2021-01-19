# rust-microservices
Microservices with Rust

docker run --rm --detach --name postgres --env POSTGRES_USER=username --env POSTGRES_PASSWORD=password --publish 127.0.0.1:5432:5432 postgres

[Once]
diesel setup
diesel migration generate create_posts
[Once]?
diesel migration run

cargo run

[Create]
curl -d '{"title": "Rust microservices", "body": "Using Diesel to implement persistence"}' -H "Content-Type: application/json" -X POST http://localhost:8000/posts

curl -d '{"title": "Rust microservices", "body": "Using Rocket to build a API"}' -H "Content-Type: application/json" -X POST http://localhost:8000/posts

[Get]
curl http://localhost:8000/posts/1

[Update]
curl -d '{"id":1, "title": "Rust microservices", "body": "Using Diesel and Rocket to build microservices", "published": true}' -H "Content-Type: application/json" -X PUT http://localhost:8000/posts/1

[Delete]
curl -X DELETE http://localhost:8000/posts/2

[Get all]
curl http://localhost:8000/posts
