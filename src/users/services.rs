use futures::stream::StreamExt;
use mongodb::Database;
use async_graphql::{Error, ErrorExtensions};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::util::{
    constant::{CFG, GqlResult},
    common::{Claims, token_data},
};
use crate::users::models::{User, UserNew, SignInfo};

// get user info by email
pub async fn get_user_by_email(db: Database, email: &str) -> GqlResult<User> {
    let coll = db.collection("users");

    let exist_document = coll.find_one(bson::doc! {"email": email}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
            Ok(user)
        } else {
            Err(Error::new("2-email").extend_with(|_, e| e.set("details", "Email not found")))
        }
    } else {
        Err(Error::new("1-email").extend_with(|_, e| e.set("details", "Error searching mongodb")))
    }
}

// get user info by username
pub async fn get_user_by_username(db: Database, username: &str) -> GqlResult<User> {
    let coll = db.collection("users");

    let exist_document = coll.find_one(bson::doc! {"username": username}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
            Ok(user)
        } else {
            Err(Error::new("4-username").extend_with(|_, e| e.set("details", "Username not found")))
        }
    } else {
        Err(Error::new("3-username")
            .extend_with(|_, e| e.set("details", "Error searching mongodb")))
    }
}

pub async fn user_register(db: Database, mut user_new: UserNew) -> GqlResult<User> {
    let coll = db.collection("users");

    user_new.email = user_new.email.to_lowercase();
    user_new.username = user_new.username.to_lowercase();

    if self::get_user_by_email(db.clone(), &user_new.email).await.is_ok() {
        Err(Error::new("email exists").extend_with(|_, e| e.set("details", "1_EMAIL_EXIStS")))
    } else if self::get_user_by_username(db.clone(), &user_new.username).await.is_ok() {
        Err(Error::new("username exists").extend_with(|_, e| e.set("details", "2_USERNAME_EXISTS")))
    } else {
        user_new.cred = super::cred::cred_encode(&user_new.username, &user_new.cred).await;
        let user_new_bson = bson::to_bson(&user_new).unwrap();

        if let bson::Bson::Document(document) = user_new_bson {
            // Insert into a MongoDB collection
            coll.insert_one(document, None)
                .await
                .expect("Failed to insert into a MongoDB collection!");

            self::get_user_by_email(db.clone(), &user_new.email).await
        } else {
            Err(Error::new("5-register").extend_with(|_, e| {
                e.set("details", "Error converting the BSON object into a MongoDB document")
            }))
        }
    }
}

pub async fn user_sign_in(db: Database, mut unknown_user: UserNew) -> GqlResult<SignInfo> {
    unknown_user.email = unknown_user.email.to_lowercase();
    unknown_user.username = unknown_user.username.to_lowercase();

    let user_res;
    match regex::Regex::new(r"(@)").unwrap().is_match(&unknown_user.email) {
        true => {
            user_res = self::get_user_by_email(db.clone(), &unknown_user.email).await;
        }
        false => {
            user_res = self::get_user_by_username(db.clone(), &unknown_user.username).await;
        }
    }

    if let Ok(user) = user_res {
        if super::cred::cred_verify(&user.username, &unknown_user.cred, &user.cred).await {
            let mut header = Header::default();
            // header.kid = Some("signing_key".to_owned());
            header.alg = Algorithm::HS512;

            let site_key = CFG.get("SITE_KEY").unwrap().as_bytes();
            let claim_exp = CFG.get("CLAIM_EXP").unwrap().parse::<usize>().unwrap();
            let claims = Claims {
                email: user.email.to_owned(),
                username: user.username.to_owned(),
                exp: claim_exp,
            };

            let token = match encode(&header, &claims, &EncodingKey::from_secret(site_key)) {
                Ok(t) => t,
                Err(error) => Err(Error::new("7-user-sign-in").extend_with(|_, e| {
                    e.set("details", format!("Error to encode token: {}", error))
                }))
                .unwrap(),
            };

            let sign_info = SignInfo { email: user.email, username: user.username, token: token };
            Ok(sign_info)
        } else {
            Err(Error::new("user_sign_in")
                .extend_with(|_, e| e.set("details", "Invalid credential")))
        }
    } else {
        Err(Error::new("user_sign_in").extend_with(|_, e| e.set("details", "User not exist")))
    }
}

pub async fn users_list(db: Database, token: &str) -> GqlResult<Vec<User>> {
    let token_data = token_data(token).await;
    if token_data.is_ok() {
        let coll = db.collection("users");

        let mut users: Vec<User> = vec![];

        // Query all documents in the collection.
        let mut cursor = coll.find(None, None).await.unwrap();

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let user = bson::from_bson(bson::Bson::Document(document)).unwrap();
                    users.push(user);
                }
                Err(error) => Err(Error::new("6-all-users")
                    .extend_with(|_, e| e.set("details", format!("Error to find doc: {}", error))))
                .unwrap(),
            }
        }

        if users.len() > 0 {
            Ok(users)
        } else {
            Err(Error::new("6-all-users").extend_with(|_, e| e.set("details", "No records")))
        }
    } else {
        Err(Error::new("6-all-users")
            .extend_with(|_, e| e.set("details", format!("{}", token_data.err().unwrap()))))
    }
}

// Change user password
pub async fn user_change_password(
    db: Database,
    pwd_cur: &str,
    pwd_new: &str,
    token: &str,
) -> GqlResult<User> {
    let token_data = token_data(token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = self::get_user_by_email(db.clone(), &email).await;
        if let Ok(mut user) = user_res {
            if super::cred::cred_verify(&user.username, pwd_cur, &user.cred).await {
                user.cred = super::cred::cred_encode(&user.username, pwd_new).await;

                let coll = db.collection("users");
                coll.update_one(
                    bson::doc! {"_id": &user._id},
                    bson::doc! {"$set": {"cred": &user.cred}},
                    None,
                )
                .await
                .expect("Failed to update a MongoDB collection!");

                Ok(user)
            } else {
                Err(Error::new("user_change_password")
                    .extend_with(|_, e| e.set("details", "Error verifying current passwordd")))
            }
        } else {
            Err(Error::new("user_change_password")
                .extend_with(|_, e| e.set("details", "User not exist")))
        }
    } else {
        Err(Error::new("user_change_password")
            .extend_with(|_, e| e.set("details", format!("{}", token_data.err().unwrap()))))
    }
}

// update user profile
pub async fn user_update_profile(db: Database, user_new: UserNew, token: &str) -> GqlResult<User> {
    let token_data = token_data(token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = self::get_user_by_email(db.clone(), &email).await;
        if let Ok(mut user) = user_res {
            let coll = db.collection("users");

            user.email = user_new.email.to_lowercase();
            user.username = user_new.username.to_lowercase();

            let user_bson = bson::to_bson(&user).unwrap();
            let user_doc = user_bson.as_document().unwrap().to_owned();

            coll.find_one_and_replace(bson::doc! {"_id": &user._id}, user_doc, None)
                .await
                .expect("Failed to replace a MongoDB collection!");

            Ok(user)
        } else {
            Err(Error::new("user_update_profile")
                .extend_with(|_, e| e.set("details", "User not exist")))
        }
    } else {
        Err(Error::new("user_update_profile")
            .extend_with(|_, e| e.set("details", format!("{}", token_data.err().unwrap()))))
    }
}
