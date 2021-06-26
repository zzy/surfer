use crate::util::constant::CFG;

pub async fn gql_uri() -> String {
    let addr = CFG.get("gql.addr").unwrap();
    let port = CFG.get("gql.port").unwrap();
    let path = CFG.get("gql.path").unwrap();

    format!("http://{}:{}/{}", addr, port, path)
}
