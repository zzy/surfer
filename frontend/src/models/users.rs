use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInInfo {
    pub signature: String,
    pub password: String,
}
