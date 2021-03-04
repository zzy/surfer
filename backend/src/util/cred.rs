use std::num::NonZeroU32;
use ring::{digest, pbkdf2};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    decode, TokenData, Algorithm, DecodingKey, Validation, errors::Error,
};

use crate::util::constant::CFG;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

// The salt should have a user-specific component so that an attacker
// cannot crack one password for multiple users.
async fn salt(username: &str) -> Vec<u8> {
    let salt_component: [u8; 16] = [
        // This value was generated from a secure PRNG.
        0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52, 0x24, 0xf2, 0x27, 0xd1,
        0xfe, 0x39, 0x01, 0x8a,
    ];

    let mut salt =
        Vec::with_capacity(salt_component.len() + username.as_bytes().len());

    salt.extend(salt_component.as_ref());
    salt.extend(username.as_bytes());

    salt
}

pub async fn cred_encode(username: &str, password: &str) -> String {
    const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
    type Credential = [u8; CREDENTIAL_LEN];

    let salt = salt(username).await;

    let mut cred: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut cred,
    );

    base64::encode(&cred)
}

pub async fn cred_verify(
    username: &str,
    pwd_try: &str,
    actual_cred: &str,
) -> bool {
    let salt = salt(username).await;
    let actual_cred_decode = base64::decode(actual_cred.as_bytes()).unwrap();

    pbkdf2::verify(
        PBKDF2_ALG,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        pwd_try.as_bytes(),
        &actual_cred_decode,
    )
    .is_ok()
}

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
