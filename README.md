# rust-microservices
## Microservices with Rust

## Run postgres container
`docker run --rm --detach --name postgres --env POSTGRES_USER=username --env POSTGRES_PASSWORD=password --publish 127.0.0.1:5432:5432 postgres`

## Setup diesel and migrartions

`diesel setup`

`diesel migration generate create_posts`

`diesel migration run`

## Run and test services
`cargo run`

`curl -d '{"title": "Rust microservices", "body": "Using Diesel to implement persistence"}' -H "Content-Type: application/json" -X POST http://localhost:8000/posts`

`curl -d '{"title": "Rust microservices", "body": "Using Rocket to build a API"}' -H "Content-Type: application/json" -X POST http://localhost:8000/posts`

`curl http://localhost:8000/posts/1`

`curl -d '{"id":1, "title": "Rust microservices", "body": "Using Diesel and Rocket to build microservices", "published": true}' -H "Content-Type: application/json" -X PUT http://localhost:8000/posts/1`

`curl -X DELETE http://localhost:8000/posts/2`

`curl http://localhost:8000/posts`

## docker-compose deployment and test

`docker-compose up -d`

`diesel migration run`

`curl -d '{"title": "Rust microservices", "body": "Using Diesel to implement persistence"}' -H "Content-Type: application/json" -X POST http://localhost:8000/posts`

`curl -d '{"title": "Rust microservices", "body": "Using Rocket to build a API"}' -H "Content-Type: application/json" -X POST http://localhost:8000/posts`

`curl http://localhost:8000/posts/1`

`curl -d '{"id":1, "title": "Rust microservices", "body": "Using Diesel and Rocket to build microservices", "published": true}' -H "Content-Type: application/json" -X PUT http://localhost:8000/posts/1`

`curl -X DELETE http://localhost:8000/posts/2`

`curl http://localhost:8000/posts`

`docker-compose down`
