use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    decode, TokenData, Algorithm, DecodingKey, Validation, errors::Error,
};

use crate::util::constant::CFG;

// pub async fn web_base_uri() -> String {
//     // let web_prot = CFG.get("WEB_PROT").unwrap();
//     let web_addr = CFG.get("WEB_ADDR").unwrap();
//     let web_port = CFG.get("WEB_PORT").unwrap();

//     // format!("{}://{}:{}", web_prot, web_addr, web_port)
//     format!("//{}:{}", web_addr, web_port)
// }

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
