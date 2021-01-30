use std::collections::BTreeMap;
use tide::{
    Response, StatusCode, Body,
    {http::mime::HTML},
};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

use crate::util::constant::CFG;

pub async fn gql_uri() -> String {
    let gql_prot = CFG.get("GQL_PROT").unwrap();
    let gql_addr = CFG.get("GQL_ADDR").unwrap();
    let gql_port = CFG.get("GQL_PORT").unwrap();
    let gql_uri = CFG.get("GQL_URI").unwrap();
    let gql_path = CFG.get("GQL_VER").unwrap();

    format!("{}://{}:{}/{}/{}", gql_prot, gql_addr, gql_port, gql_uri, gql_path)
}

pub async fn rhai_dir() -> String {
    format!("./{}/", "./rhai")
}

pub async fn tpl_dir() -> String {
    format!("./{}/", "./templates")
}

pub struct Tpl<'tpl> {
    pub name: String,
    pub reg: Handlebars<'tpl>,
}

impl<'tpl> Tpl<'tpl> {
    pub async fn new(rel_path: &str) -> Tpl<'tpl> {
        let tpl_name = &rel_path.replace("/", "_");
        let abs_path = format!("{}{}.html", tpl_dir().await, rel_path);

        // create the handlebars registry
        let mut hbs_reg = Handlebars::new();
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

    pub async fn reg_head(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "head",
                format!("{}{}", tpl_dir().await, "common/head.html"),
            )
            .unwrap();

        data.insert("head", json!("head"));
    }

    pub async fn reg_header(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "header",
                format!("{}{}", tpl_dir().await, "common/header.html"),
            )
            .unwrap();

        data.insert("header", json!("header"));
    }

    pub async fn reg_nav(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "nav",
                format!("{}{}", tpl_dir().await, "common/nav.html"),
            )
            .unwrap();

        data.insert("nav", json!("nav"));
    }

    pub async fn reg_footer(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "footer",
                format!("{}{}", tpl_dir().await, "common/footer.html"),
            )
            .unwrap();

        data.insert("footer", json!("footer"));
    }

    pub async fn reg_script_website_svg(&mut self) {
        self.reg
            .register_script_helper_file(
                "website-svg",
                format!("{}{}", rhai_dir().await, "website-svg.rhai"),
            )
            .unwrap();
    }

    pub async fn reg_script_blog_name(&mut self) {
        self.reg
            .register_script_helper_file(
                "blog-name",
                format!("{}{}", rhai_dir().await, "blog-name.rhai"),
            )
            .unwrap();
    }
}
