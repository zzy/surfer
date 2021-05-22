# surfer

The **Blog** built on *pure Rust stack*. 

Backend for graphql services using tide, async-graphql, jsonwebtoken, mongodb and so on. 

Frontend for web application using tide, rhai, surf, graphql_client, handlebars-rust, cookie and so on.

## Features

Demo site: [https://blog.budshome.com](https://blog.budshome.com)

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

- [**Frontend: web application server**](./frontend/README.md)
- [**Backend: graphql servies server**](./backend/README.md)

## How to Test & Run `rhai scripts`

You could use `rhai-repl` to test your rhai code, and use `rhai-run` to run it. `rhai-repl.rs` and `rhai-run.rs` are in the folder `frontend/scripts`, please copy them into `frontend/examples` folder, then test or run rhai code with command:

``` bash 
cargo run --example <rhai-repl>/<rhai-run ./scripts/script_to_run.rhai>
``` 

If you would want to install the rhai tool, use the command 

``` bash
cargo install --path . --example <rhai-repl>/<rhai-run>
```

then test rhai code using `rhai-repl`, and run scripts using the `rhai-run`:

``` bash
rhai-run ./scripts/script_to_run.rhai
```

## Contributing

You are welcome in contributing to the surfer project. 
