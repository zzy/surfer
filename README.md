# surfer

The **Blog** built on *pure Rust stack*. 

Backend for graphql services using tide, async-graphql, jsonwebtoken, mongodb and so on. 

There are two options for web frontend:
- Frontend-yew for web application using yew, graphql_client, cookie and so on.
- Frontend-handlebars for web application using tide, yew, rhai, surf, graphql_client, handlebars-rust, cookie and so on.

## Features

Demo site:
- [niqin.com - NiQin Books Platform | 泥芹书馆](https://niqin.com)
- [gaidun.com - Project Matchmaking | 项目对接](https://gaidun.com)

See also: https://github.com/zzy/tide-async-graphql-mongodb

## MongoDB data

MongoDB data(include structure & documents) file is `/data/surfer-dev.sql`.

If you need mongodb cloud count, please send email to me.

## Stacks

- [Rust](https://github.com/rust-lang/rust) - [Rust By Example](https://rust-by-example.niqin.com) and [Cargo Book](https://cargo.niqin.com)
- [Tide](https://crates.io/crates/tide) - [Tide Book](https://tide-book.niqin.com)
- [rhai](https://crates.io/crates/rhai) - [Embedded Scripting for Rust](https://rhai-script.niqin.com)
- [async-graphql](https://crates.io/crates/async-graphql) - [async-graphql docs](https://async-graphql.niqin.com)
- [mongodb & mongo-rust-driver](https://crates.io/crates/mongodb)
- [Surf](https://crates.io/crates/surf)
- [graphql_client](https://crates.io/crates/graphql_client)
- [yew](https://yew.niqin.com)
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
