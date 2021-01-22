# blog-rs

Clean boilerplate for graphql services using tide, async-graphql, surf, graphql-client, handlebars-rust, jsonwebtoken, and mongodb.

Demo site: [https://blog.budshome.com](https://blog.budshome.com)

## Features

- [x] User management, and salt and hash a password with PBKDF2 - 使用 PBKDF2 对密码进行加密（salt）和散列（hash）运算
- [ ] ...

## Stacks

- [Rust](https://www.rust-lang.org) - [中文资料集萃](https://budshome.com)
- [Tide](https://crates.io/crates/tide) - [中文文档](https://tide.budshome.com)
- [async-graphql](https://crates.io/crates/async-graphql) - [中文文档](https://async-graphql.budshome.com)
- [mongodb & mongo-rust-driver](https://crates.io/crates/mongodb)
- [Surf](https://crates.io/crates/surf)
- [graphql_client](https://crates.io/crates/graphql_client)
- [handlebars-rust](https://crates.io/crates/handlebars)
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
- [cookie-rs](https://crates.io/crates/cookie)

### Why not use React / Vue / Angular / ...?

In brief, for **SEO**.

## How to Build & Run?

``` Bash
git clone https://github.com/zzy/blog-rs.git
cd blog-rs
```

Put the environment variables into a `.env` file:

```
ADDRESS=127.0.0.1
PORT=8080

GRAPHQL_VER=graphql
GRAPHIQL_VER=graphiql

MONGODB_URI=mongodb://mongo:mongo@127.0.0.1:27017
MONGODB_BLOG=blog

SITE_KEY=0F4EHz+1/hqVvZjuB8EcooQs1K6QKBvLUxqTHt4tpxE=
CLAIM_EXP=10000000000
```

### Build & Run:

``` Bash
cargo build
cargo run
```

Then connect to http://127.0.0.1:8080 with browser.

GraphiQL: connect to http://127.0.0.1:8080/graphiql with browser.

## Contributing

You are welcome in contributing to the blog-rs project.
