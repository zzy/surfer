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

pub async fn scripts_dir() -> String {
    format!("./{}/", "./scripts")
}

pub async fn tpls_dir() -> String {
    format!("./{}/", "./templates")
}

pub struct Tpl<'tpl> {
    pub name: String,
    pub reg: Handlebars<'tpl>,
}

impl<'tpl> Tpl<'tpl> {
    pub async fn new(rel_path: &str) -> Tpl<'tpl> {
        let tpl_name = &rel_path.replace("/", "_");
        let abs_path = format!("{}{}.html", tpls_dir().await, rel_path);

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
                format!("{}{}", tpls_dir().await, "common/head.html"),
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
                format!("{}{}", tpls_dir().await, "common/header.html"),
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
                format!("{}{}", tpls_dir().await, "common/nav.html"),
            )
            .unwrap();

        data.insert("nav", json!("nav"));
    }

    pub async fn reg_sidebar(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "sidebar",
                format!("{}{}", tpls_dir().await, "common/sidebar.html"),
            )
            .unwrap();

        data.insert("sidebar", json!("sidebar"));
    }

    pub async fn reg_introduction(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "introduction",
                format!("{}{}", tpls_dir().await, "common/introduction.html"),
            )
            .unwrap();

        data.insert("introduction", json!("introduction"));
    }

    pub async fn reg_topic(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "topic",
                format!("{}{}", tpls_dir().await, "common/topic.html"),
            )
            .unwrap();

        data.insert("topic", json!("topic"));
    }

    pub async fn reg_elsewhere(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "elsewhere",
                format!("{}{}", tpls_dir().await, "common/elsewhere.html"),
            )
            .unwrap();

        data.insert("elsewhere", json!("elsewhere"));
    }

    pub async fn reg_pagination(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "pagination",
                format!("{}{}", tpls_dir().await, "common/pagination.html"),
            )
            .unwrap();

        data.insert("pagination", json!("pagination"));
    }

    pub async fn reg_footer(
        &mut self,
        data: &mut BTreeMap<&str, serde_json::Value>,
    ) {
        self.reg
            .register_template_file(
                "footer",
                format!("{}{}", tpls_dir().await, "common/footer.html"),
            )
            .unwrap();

        data.insert("footer", json!("footer"));
    }

    pub async fn reg_script_value_check(&mut self) {
        self.reg
            .register_script_helper_file(
                "value-check",
                format!("{}{}", scripts_dir().await, "value-check.rhai"),
            )
            .unwrap();
    }

    pub async fn reg_script_website_svg(&mut self) {
        self.reg
            .register_script_helper_file(
                "website-svg",
                format!("{}{}", scripts_dir().await, "website-svg.rhai"),
            )
            .unwrap();
    }

    pub async fn reg_script_sci_format(&mut self) {
        self.reg
            .register_script_helper_file(
                "sci-format",
                format!("{}{}", scripts_dir().await, "sci-format.rhai"),
            )
            .unwrap();
    }

    pub async fn reg_script_str_trc(&mut self) {
        self.reg
            .register_script_helper_file(
                "str-trc",
                format!("{}{}", scripts_dir().await, "str-trc.rhai"),
            )
            .unwrap();
    }
}

pub fn get_username_from_cookies(
    req: tide::Request<crate::State>,
) -> Option<String> {
    let mut username: Option<String> = None;
    if let Some(cookie) = req.cookie("username") {
        let target = cookie.value().trim();
        if "".ne(target) && "-".ne(target) {
            username = Some(String::from(target));
        }
    }
    username
}
