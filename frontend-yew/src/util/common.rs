use crate::util::constant::CFG;

pub fn gql_uri() -> String {
    let addr = CFG.get("gql.addr").unwrap();
    // for local test
    // let port = CFG.get("gql.port").unwrap();
    let path = CFG.get("gql.path").unwrap();

    format!("https://{}/{}", addr, path)
    // for local test
    // format!("http://{}:{}/{}", addr, port, path)
}
