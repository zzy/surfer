# surfer

The **Blog** built on *pure Rust stack*. 

Backend for graphql services using tide, async-graphql, jsonwebtoken, mongodb and so on. 

There are two options for web frontend:
- Frontend-yew for web application using yew, graphql_client, cookie and so on.
- Frontend-handlebars for web application using tide, yew, rhai, surf, graphql_client, handlebars-rust, cookie and so on.

## Features

Demo site:
- [https://blog.rusthub.top](https://blog.rusthub.top) with frontend-yew.
- [https://blog.budshome.com](https://blog.budshome.com) with frontend-handlebars.

## MongoDB data

MongoDB data(include structure & documents) file is `/data/surfer-dev.sql`.

If you need mongodb cloud count, email to me or wechat(微信): yupen-com, please.

## Stacks

- [Rust](https://github.com/rust-lang/rust) - [Rust By Example](https://rust-by-example.budshome.com) and [Cargo Book](https://cargo.budshome.com)
- [Tide](https://crates.io/crates/tide) - [Tide Book](https://tide.budshome.com)
- [rhai](https://crates.io/crates/rhai) - [Embedded Scripting for Rust](https://rhai.budshome.com)
- [async-graphql](https://crates.io/crates/async-graphql) - [async-graphql docs](https://async-graphql.budshome.com)
- [mongodb & mongo-rust-driver](https://crates.io/crates/mongodb)
- [Surf](https://crates.io/crates/surf)
- [graphql_client](https://crates.io/crates/graphql_client)
- [handlebars-rust](https://crates.io/crates/handlebars)
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
- [cookie-rs](https://crates.io/crates/cookie)

## How to Build & Run?

Please read:

- [**Backend: graphql servies server**](./backend/README.md)
- [**Frontend-yew: web application server**](./frontend-yew/README.md)
- [**Frontend-handlebars: web application server**](./frontend-handlebars/README.md)

## Contributing

You are welcome in contributing to the surfer project. 
