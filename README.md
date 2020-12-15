# rust-microservices
Microservices with Rust

docker run --rm --detach --name postgres --env POSTGRES_USER=username --env POSTGRES_PASSWORD=password --publish 127.0.0.1:5432:5432 postgres

[Once]
diesel setup
diesel migration generate create_posts
[Once]
diesel migration run

cargo run

curl -d '{"title": "Rust microservices", "body": "Using Diesel and Rocket to build a microservices"}' -H "Content-Type: application/json" -X POST http://localhost:8000/post

curl -d '{"title": "Spring microservices", "body": "Using Spring Boot and Spring Cloud to build a microservices"}' -H "Content-Type: application/json" -X POST http://localhost:8000/post

curl http://localhost:8000/post