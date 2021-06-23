use crate::util::constant::CFG;

pub async fn gql_uri() -> String {
    let addr = CFG.get("addr").unwrap();
    let port = CFG.get("port").unwrap();
    let path = CFG.get("path").unwrap();

    format!("http://{}:{}/{}", addr, port, path)
}
