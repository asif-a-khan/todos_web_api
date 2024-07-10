### I forgot how to write markdown so this might be awkward.

## I'm trying to learn how to web dev with rust. The current atate of backend framework documentation is.. limited........

I figured if I setup a todos app that incorporated:
  -  A dedicated database
  -  database schemas
  -  migrations & rollbacks
  -  controllers for the API routes
  -  auth (Ideally OAuth too)
  -  and middleware.

Then I would gain more confidence to build more projects as this will serve as the foundation for everything else. 

If I can do this well, perhaps other aspiring rust devs could use this as a template to learn how to build API's using Rust.

#Stack & external crates used tbh... you can just check this stuff out in the cargo.toml file but I'm bored at work.
- Rust (Tokio async runtime, Tls is Rustls).
  - tokio 1.38.0 
  - serde 1.0.204
  - dotenv 0.15.0
  - axum framework 0.7.5
  - sqlx (MySQL implementation). 0.7.4
 
  - I wanted to use ormx but it seems if you follow their mysql example there are some version compatibility issues with sqlx and/or tokio at the moment. I'll just use this as an oppurtunity to hone my SQL skills.

# Godspeed

