# Graphql Services Server - tide + async-graphql

## MongoDB data

MongoDB data(include structure & documents) file is `/data/surfer-dev.sql`.

If you need mongodb cloud count, please send email to me.

## Build & run

``` Bash
git clone https://github.com/zzy/surfer.git
cd surfer
cargo build

cd backend
```

Rename file `.env.example` to `.env`, or put the environment variables into a `.env` file:

```
ADDR=127.0.0.1
PORT=8000

SITE_KEY=NALnvA++OlmRAiO2h..... # Replace with your SITE_KEY
CLAIM_EXP=10000000000

GQL_URI=gql
GQL_VER=v1
GIQL_VER=v1i

MONGODB_URI=mongodb://mongo:mongo@127.0.0.1:27017
MONGODB_NAME=blog
```

Then, build & run:

``` Bash
cargo run
```

GraphiQL: connect to http://127.0.0.1:8000/gql/v1i with browser.

![Graphql Image](../data/graphiql.jpg)

## Queries

- userById(id: ObjectId!): User!
- userByEmail(email: String!): User!
- userByUsername(username: String!): User!
- userSignIn(signature: String!, password: String!): - SignInfo!
- users(token: String!): [User!]!
- articles(published: Int!): [Article!]!
- articlesInPosition(
  username: String!
  position: String!
  limit: Int!
): [Article!]!
- articlesByUserId(userId: ObjectId!, published: Int!): [Article!]!
- articlesByUsername(username: String!, published: Int!): [Article!]!
- articlesByCategoryId(categoryId: ObjectId!, published: Int!): [Article!]!
- articleBySlug(username: String!, slug: String!): Article!
- categories: [Category!]!
- categoriesByUserId(userId: ObjectId!): [Category!]!
- categoriesByUsername(username: String!): [Category!]!
- categoryById(id: ObjectId!): Category!
- categoryBySlug(slug: String!): Category!
- topics: [Topic!]!
- topicsByArticleId(articleId: ObjectId!): [Topic!]!
- wishes(published: Int!): [Wish!]!
- randomWish(username: String!): Wish!

## MUTATIONS

- userRegister(userNew: UserNew!): User!
- userChangePassword(pwdCur: String!, pwdNew: String!, token: String!): User!
- userUpdateProfile(userNew: UserNew!, token: String!): User!
- articleNew(articleNew: ArticleNew!): Article!
- categoryNew(categoryNew: CategoryNew!): Category!
- categoryUserNew(categoryUserNew: CategoryUserNew!): CategoryUser!
- topicNew(topicNew: TopicNew!): Topic!
- topicArticleNew(topicArticleNew: TopicArticleNew!): TopicArticle!
- wishNew(wishNew: WishNew!): Wish!

## Contributing

You are welcome in contributing to this project.
