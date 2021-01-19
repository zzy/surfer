use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, TokenData, Algorithm, DecodingKey, Validation, errors::Error};
use tide::{
    Response, StatusCode, Body,
    {http::mime::HTML},
};

use crate::util::constant::CFG;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub username: String,
    pub exp: usize,
}

pub async fn token_data(token: &str) -> Result<TokenData<Claims>, Error> {
    let site_key = CFG.get("SITE_KEY").unwrap().as_bytes();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(site_key),
        &Validation::new(Algorithm::HS512),
    );

    data
}

pub struct Tpl {
    pub name: String,
    pub reg: handlebars::Handlebars<'static>,
}

impl Tpl {
    pub async fn new(rel_path: &str) -> Tpl {
        let tpl_name = &rel_path.replace("/", "_");
        let abs_path = format!("./static/{}.html", rel_path);

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
        resp.set_body(Body::from_string(self.reg.render(&self.name, data).unwrap()));

        Ok(resp.into())
    }
}
