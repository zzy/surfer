use serde::Serialize;
use tide::{
    Response, StatusCode, Body,
    {http::mime::HTML},
};

use crate::util::constant::CFG;

pub async fn gql_uri() -> String {
    let gql_prot = CFG.get("GQL_PROT").unwrap();
    let gql_addr = CFG.get("GQL_ADDR").unwrap();
    let gql_port = CFG.get("GQL_PORT").unwrap();
    let gql_uri = CFG.get("GQL_URI").unwrap();
    let gql_path = CFG.get("GQL_VER").unwrap();

    format!("{}://{}:{}/{}/{}", gql_prot, gql_addr, gql_port, gql_uri, gql_path)
}

pub async fn tpl_dir() -> String {
    format!("./{}/", "templates")
}

pub async fn rhai_dir() -> String {
    format!("./{}/", "rhai")
}

pub struct Tpl<'tpl> {
    pub name: String,
    pub reg: handlebars::Handlebars<'tpl>,
}

impl<'tpl> Tpl<'tpl> {
    pub async fn new(rel_path: &str) -> Tpl<'tpl> {
        let tpl_name = &rel_path.replace("/", "_");
        let abs_path = format!("{}{}.html", tpl_dir().await, rel_path);

        // create the handlebars registry
        let mut hbs_reg = handlebars::Handlebars::new();
        // register template from a file and assign a name to it
        hbs_reg.register_template_file(tpl_name, abs_path).unwrap();

        Tpl { name: tpl_name.to_string(), reg: hbs_reg }
    }

    pub async fn render<T>(&self, data: &T) -> tide::Result
    where
        T: Serialize,
    {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_content_type(HTML);
        resp.set_body(Body::from_string(
            self.reg.render(&self.name, data).unwrap(),
        ));

        Ok(resp.into())
    }
}
