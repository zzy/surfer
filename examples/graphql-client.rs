use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_articles.graphql",
    response_derives = "Debug"
)]
pub struct AllArticles;
type ObjectId = String;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let uri = "http://127.0.0.1:8080/v1";

    let build_query = AllArticles::build_query(all_articles::Variables {});
    let query = serde_json::json!(build_query);
    println!("1-{:?}\n", &query);

    let resp_body: Response<all_articles::ResponseData> =
        surf::post(uri).body(query.to_owned()).recv_json().await.unwrap();
    println!("2-{:?}\n", &resp_body);

    let resp_body2: Response<serde_json::Value> =
        surf::post(uri).body(query.to_owned()).recv_json().await.unwrap();
    println!("2.1-{:?}\n", &resp_body2);

    let resp_data: all_articles::ResponseData = resp_body.data.expect("missing response data");
    println!("3-{:?}\n", resp_data);

    let resp_data2: serde_json::Value = resp_body2.data.expect("missing response data");
    println!("3-1-{:?}\n", resp_data2);

    println!("{}", &resp_data2.is_object());
    println!("{}\n", &resp_data2.is_string());

    let articles_data = resp_data.all_articles;
    println!("4-{:?}\n", &articles_data);

    let articles_data2 = &resp_data2["allArticles"];
    println!("4-1-{:?}\n", &articles_data2);

    println!("{}", &articles_data2.is_object());
    println!("{}", &articles_data2.is_string());

    Ok(())
}
