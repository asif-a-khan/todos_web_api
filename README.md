# Steps to run locally
1. Ensure you have docker cli installed for Linux and/or docker-desktop installed on Windows. If you're on a Mac... kick rocks.. jk I'll try to add the steps for Mac OS later.
2. Clone/download repo to wherever you want.
3. cd into the directory of the repo.
4. Create a .env file in the root of the directory. Edit the variables to what you want.

```
DATABASE_URL="mysql://root:<enter_password>@mysql_db:3306/<enter_db_name>"
MYSQL_ROOT_PASSWORD="<enter_password>"
MYSQL_DATABASE="<enter_db_name>"
```

5. Run docker compose up -d --build.
6. You should have a container with two images running now. One for the Rust web server and the other for MySql database.
7. The web server is exposed on port 8000 so you can interact with it locally from localhost:8000 and the database is exposed on port 3306 so you can interact with it on port 3306.
8. Open up postman or whichever api testing kit you use (I use insomnia).

## Routes 

Base: "localhost"

- GET "/todos" ~ Returns a vector (Array) of all todos.
- GET "/todos/<enter_id>" ~ Finds the todo with the specified ID if it exists. Returns the todo in JSON format.
  
- POST "/todos" ~ Creates a todo item and stores it in the database. Returns the created todo ID. You need to supply a JSON request body as such:
```
{
  "description": "Enter a new todo",
  "done": true
}
```

- PATCH "/todos/<enter_id>" ~ Finds and updates the specified todo via ID if it exists. You need to supply a JSON request body containing the same content as the POST request body. I'll eventually make this more convenient so that you can update as many or little fields you want. For now you have to provide all the fields in a todo.

- Delete "/todos/<enter_id>" ~ Finds and deletes a todo with the specified ID if it exists. Does not return anything as of right now.


  
# Overview

## I'm trying to learn how to web dev with rust. The current state of backend framework documentation is.. limited........

I figured if I setup a todos app that incorporated:
  -  A dedicated database
  -  database schemas
  -  migrations & rollbacks
  -  controllers for the API routes
  -  auth (Ideally OAuth too)
  -  and middleware.

Then I would gain more confidence to build more projects as this will serve as the foundation for everything else. 

If I can do this well, perhaps other aspiring rust devs could use this as a template to learn how to build API's using Rust.

## Stack & crates used 
Tbh... you can just check this stuff out in the cargo.toml file but I'm bored at work.

- Rust (Tokio async runtime, Tls is Rustls).
  - tokio 1.38.0 
  - serde 1.0.204
  - dotenv 0.15.0
  - axum framework 0.7.5
  - sqlx (MySQL implementation). 0.7.4
 
  - I wanted to use ormx but it seems if you follow their mysql example there are some version compatibility issues with sqlx and/or tokio at the moment. I'll just use this as an oppurtunity to hone my SQL skills.

# Godspeed

