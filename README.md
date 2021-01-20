# tide-handlebars-graphql-mongodb

Clean boilerplate for graphql services using tide, async-graphql, surf, graphql-client, handlebars-rust, jsonwebtoken, and mongodb. 

## Features

- [x] User register
- [x] Salt and hash a password with PBKDF2 - 使用 PBKDF2 对密码进行加密（salt）和散列（hash）运算
- [x] Sign in
- [x] JSON web token authentication - JWT 鉴权整合
- [x] Change password
- [x] Profile Update
- [x] User: query & mutation
- [x] Project: query & mutation
- [x] Client request, bring & parse GraphQL data
- [x] Render data to template engine

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

## How to run?

``` Bash
git clone https://github.com/zzy/tide-handlebars-graphql-mongodb.git
cd tide-handlebars-graphql-mongodb
```

Put the environment variables into a `.env` file:

```
ADDRESS=127.0.0.1
PORT=8080

GRAPHQL_PATH=graphql
GRAPHIQL_PATH=graphiql

MONGODB_URI=mongodb://mongo:mongo@127.0.0.1:27017
MONGODB_BLOG=budshome

SITE_KEY=0F4EHz+1/hqVvZjuB8EcooQs1K6QKBvLUxqTHt4tpxE=
CLAIM_EXP=10000000000
```

Build & Run:

``` Bash
cargo build
cargo run
```

Then connect to http://127.0.0.1:8080 with browser.

GraphiQL: connect to http://127.0.0.1:8080/graphiql with browser.

## Queries

- getUserByEmail(...): User!
- getUserByUsername(...): User!
- userSignIn(...): SignInfo!
- allUsers(...): [User!]!
- allProjects: [Project!]!
- allProjectsByUser(...): [Project!]!

## MUTATIONS

- userRegister(...): User!
- userChangePassword(...): User!
- userUpdateProfile(...): User!
- addProject(...): Project!

## Sample Usage

Sample mutation for user register:
```
mutation {
  userRegister(
    newUser: { 
      email: "example@budshome.com", 
      username: "我是谁", 
      password: "wo#$shi^$shui" 
    }
  ) {
    id
    email
    username
  }
}
```

Sample query for user sign in:
```
{
  userSignIn(
    userAccount: {
      email: "example@budshome.com"
      username: ""
      password: "wo#$shi^$shui"
    }
  ) {
    email
    username
    token
  }
}
```

When submit method `userSignIn`, a token would be generated, use this token for query all users and every user's projects:
```
{
  allUsers(
    token: "fyJ0eXAiOiJKV1Q..."
  ) {
    id
    email
    username

    projects {
      id
      userId
      subject
      website
    }
  }
}
```

Sample query and mutation for projects was similar to users.
