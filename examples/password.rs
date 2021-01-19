use std::num::NonZeroU32;
use ring::{digest, pbkdf2};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

#[async_std::main]
async fn main() {
    let username = "芽之家";
    let password = "budshome.com";

    let cred_en = cred_encode(username, password).await;
    let cred_de = base64::decode(&cred_en.as_bytes()).unwrap();

    println!("{}, {}", &cred_en, &cred_en.len());
    println!("{:?}", &cred_de);

    let test33 = cred_verify(username, "%!@#$7655", &cred_en).await;
    let test22 = cred_verify(username, "%!@#$7654", &cred_en).await;

    println!("{}", &test33);
    println!("{}", &test22);
}

// The salt should have a user-specific component so that an attacker
// cannot crack one password for multiple users.
async fn salt(username: &str) -> Vec<u8> {
    let salt_component: [u8; 16] = [
        // This value was generated from a secure PRNG.
        0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52, 0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01,
        0x8a,
    ];

    let mut salt = Vec::with_capacity(salt_component.len() + username.as_bytes().len());

    salt.extend(salt_component.as_ref());
    salt.extend(username.as_bytes());

    salt
}

async fn cred_encode(username: &str, password: &str) -> String {
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

    println!("{:?}", cred);

    base64::encode(&cred)
}

async fn cred_verify(username: &str, attempted_password: &str, actual_cred: &str) -> bool {
    println!("{}", username);
    println!("{}", attempted_password);
    println!("{}", actual_cred);

    let salt = salt(username).await;
    let actual_cred_decode = base64::decode(actual_cred.as_bytes()).unwrap();
    println!("{:?}", actual_cred_decode);

    pbkdf2::verify(
        PBKDF2_ALG,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        attempted_password.as_bytes(),
        &actual_cred_decode,
    )
    .is_ok()
}
