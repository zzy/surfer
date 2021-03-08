use std::collections::BTreeMap;
use tide::{
    Request, Response, Redirect,
    http::{Method, Cookie},
};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::json;
use chrono::Local;

use crate::State;
use crate::util::common::{gql_uri, Tpl};
use crate::models::users::{SignInInfo, RegisterInfo};

type DateTime = chrono::DateTime<Local>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/index.graphql"
)]
struct IndexData;

pub async fn index(req: Request<State>) -> tide::Result {
    let mut username = String::new();
    if let Some(cookie) = req.cookie("username") {
        username.push_str(cookie.value());
    } else {
        username.push_str("-");
    }

    let mut sign_in = false;
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        sign_in = true;
    }

    let build_query = IndexData::build_query(index_data::Variables {
        sign_in: sign_in,
        username: username,
    });
    let query = json!(build_query);

    let resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await?;
    let resp_data = resp_body.data.expect("missing response data");

    let mut index: Tpl = Tpl::new("index").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    if sign_in {
        let user = resp_data["userByUsername"].clone();
        data.insert("user", user);
    }

    let categories = resp_data["categories"].clone();
    data.insert("categories", categories);

    let top_articles = resp_data["topArticles"].clone();
    data.insert("top_articles", top_articles);

    let recommended_articles = resp_data["recommendedArticles"].clone();
    data.insert("recommended_articles", recommended_articles);

    let wish = resp_data["randomWish"].clone();
    data.insert("wish", wish);

    let articles = resp_data["articles"].clone();
    data.insert("articles", articles);

    let topics = resp_data["topics"].clone();
    data.insert("topics", topics);

    index.reg_head(&mut data).await;
    index.reg_header(&mut data).await;
    index.reg_nav(&mut data).await;
    index.reg_introduction(&mut data).await;
    index.reg_topic(&mut data).await;
    index.reg_elsewhere(&mut data).await;
    index.reg_pagination(&mut data).await;
    index.reg_footer(&mut data).await;

    index.reg_script_value_check().await;
    index.reg_script_website_svg().await;
    index.reg_script_sci_format().await;
    index.reg_script_str_trc().await;

    index.render(&data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/register.graphql"
)]
struct RegisterData;

pub async fn register(mut req: Request<State>) -> tide::Result {
    let register: Tpl = Tpl::new("register").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    if req.method().eq(&Method::Post) {
        let register_info: RegisterInfo = req.body_form().await?;

        let now = Local::now();

        let build_query = RegisterData::build_query(register_data::Variables {
            email: register_info.email,
            username: register_info.username,
            nickname: register_info.nickname,
            picture: "/static/favicon.png".to_string(),
            cred: register_info.password,
            blog_name: register_info.blog_name,
            website: register_info.website,
            introduction: register_info.introduction,
            created_at: now,
            updated_at: now,
        });
        let query = json!(build_query);

        let resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await).body(query).recv_json().await?;
        let resp_data = resp_body.data;

        if let Some(register_result) = resp_data {
            data.insert(
                "register_result",
                register_result["userRegister"].to_owned(),
            );

            register.render(&data).await
        } else {
            data.insert(
                "register_failed",
                json!("Username or email already exists"),
            );

            register.render(&data).await
        }
    } else {
        register.render(&data).await
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/sign_in.graphql"
)]
struct SignInData;

pub async fn sign_in(mut req: Request<State>) -> tide::Result {
    let sign_in: Tpl = Tpl::new("sign-in").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    if req.method().eq(&Method::Post) {
        let sign_in_info: SignInInfo = req.body_form().await?;

        let build_query = SignInData::build_query(sign_in_data::Variables {
            signature: sign_in_info.signature,
            password: sign_in_info.password,
        });
        let query = json!(build_query);

        let resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await).body(query).recv_json().await?;
        let resp_data = resp_body.data;

        if let Some(sign_in_info) = resp_data {
            let mut resp: Response = Redirect::new("/").into();

            let sign_in_data = sign_in_info["userSignIn"].clone();
            resp.insert_cookie(Cookie::new(
                "email",
                sign_in_data["email"].as_str().unwrap().to_owned(),
            ));
            resp.insert_cookie(Cookie::new(
                "username",
                sign_in_data["username"].as_str().unwrap().to_owned(),
            ));
            resp.insert_cookie(Cookie::new(
                "token",
                sign_in_data["token"].as_str().unwrap().to_owned(),
            ));

            Ok(resp.into())
        } else {
            data.insert(
                "sign_in_failed",
                json!("Invalid username, email, or password"),
            );

            sign_in.render(&data).await
        }
    } else {
        sign_in.render(&data).await
    }
}

pub async fn sign_out(_req: Request<State>) -> tide::Result {
    let mut resp: Response = Redirect::new("/").into();

    resp.remove_cookie(Cookie::named("email"));
    resp.remove_cookie(Cookie::named("username"));
    resp.remove_cookie(Cookie::named("token"));

    Ok(resp.into())
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user_index.graphql"
)]
struct UserIndexData;

pub async fn user_index(req: Request<State>) -> tide::Result {
    let username = req.param("username").unwrap();

    // make data and render it
    let build_query = UserIndexData::build_query(user_index_data::Variables {
        username: username.to_string(),
    });
    let query = json!(build_query);

    let resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();
    let resp_data = resp_body.data.expect("missing response data");

    let mut user_index_tpl: Tpl = Tpl::new("users/index").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    let user = resp_data["userByUsername"].clone();
    data.insert("user", user);

    let categories = resp_data["categoriesByUsername"].clone();
    data.insert("categories", categories);

    let top_articles = resp_data["topArticles"].clone();
    data.insert("top_articles", top_articles);

    let recommended_articles = resp_data["recommendedArticles"].clone();
    data.insert("recommended_articles", recommended_articles);

    let wish = resp_data["randomWish"].clone();
    data.insert("wish", wish);

    let articles = resp_data["articlesByUsername"].clone();
    data.insert("articles", articles);

    let topics = resp_data["topicsByUsername"].clone();
    data.insert("topics", topics);

    user_index_tpl.reg_head(&mut data).await;
    user_index_tpl.reg_header(&mut data).await;
    user_index_tpl.reg_nav(&mut data).await;
    user_index_tpl.reg_introduction(&mut data).await;
    user_index_tpl.reg_topic(&mut data).await;
    user_index_tpl.reg_elsewhere(&mut data).await;
    user_index_tpl.reg_pagination(&mut data).await;
    user_index_tpl.reg_footer(&mut data).await;

    user_index_tpl.reg_script_value_check().await;
    user_index_tpl.reg_script_website_svg().await;
    user_index_tpl.reg_script_sci_format().await;
    user_index_tpl.reg_script_str_trc().await;

    user_index_tpl.render(&data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article_index.graphql"
)]
struct ArticleIndexData;

pub async fn article_index(req: Request<State>) -> tide::Result {
    let username = req.param("username").unwrap();
    let slug = req.param("slug").unwrap();

    // make data and render it
    let build_query =
        ArticleIndexData::build_query(article_index_data::Variables {
            username: username.to_string(),
            slug: slug.to_string(),
        });
    let query = json!(build_query);

    let resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();
    let resp_data = resp_body.data.expect("missing response data");

    let mut article_index_tpl: Tpl = Tpl::new("articles/index").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    let categories = resp_data["categoriesByUsername"].clone();
    data.insert("categories", categories);

    let user = resp_data["articleBySlug"]["user"].clone();
    data.insert("user", user);

    let article = resp_data["articleBySlug"].clone();
    data.insert("article", article);

    let wish = resp_data["randomWish"].clone();
    data.insert("wish", wish);

    let topics = resp_data["topicsByUsername"].clone();
    data.insert("topics", topics);

    let articles = resp_data["articlesByUsername"].clone();
    data.insert("articles", articles);

    article_index_tpl.reg_head(&mut data).await;
    article_index_tpl.reg_header(&mut data).await;
    article_index_tpl.reg_nav(&mut data).await;
    article_index_tpl.reg_topic(&mut data).await;
    article_index_tpl.reg_elsewhere(&mut data).await;
    article_index_tpl.reg_footer(&mut data).await;

    article_index_tpl.reg_script_value_check().await;
    article_index_tpl.reg_script_website_svg().await;
    article_index_tpl.reg_script_sci_format().await;

    article_index_tpl.render(&data).await
}
