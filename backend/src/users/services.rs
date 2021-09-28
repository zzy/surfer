use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{
        oid::ObjectId, DateTime, Document, doc, to_document, from_document,
    },
};
use async_graphql::{Error, ErrorExtensions};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use regex::Regex;

use crate::util::{
    constant::{CFG, GqlResult},
    cred::{cred_encode, cred_verify, Claims, token_data},
};

use super::models::{User, UserNew, SignInfo, Wish, WishNew};

// get user info by id
pub async fn user_by_id(db: Database, id: ObjectId) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let user_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let user: User = from_document(user_document)?;
    Ok(user)
}

// get user info by email
pub async fn user_by_email(db: Database, email: &str) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let exist_document = coll.find_one(doc! {"email": email}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = from_document(user_document)?;
            Ok(user)
        } else {
            Err(Error::new("Email not found").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("Error searching mongodb")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// get user info by username
pub async fn user_by_username(db: Database, username: &str) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let exist_document = coll.find_one(doc! {"username": username}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = from_document(user_document)?;
            Ok(user)
        } else {
            Err(Error::new("Username not found").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("Error searching mongodb")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

pub async fn user_register(
    db: Database,
    mut user_new: UserNew,
) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    user_new.email.make_ascii_lowercase();
    user_new.username.make_ascii_lowercase();

    if self::user_by_email(db.clone(), &user_new.email).await.is_ok() {
        Err(Error::new("email exists")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    } else if self::user_by_username(db.clone(), &user_new.username)
        .await
        .is_ok()
    {
        Err(Error::new("username exists")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    } else {
        user_new.cred = cred_encode(&user_new.username, &user_new.cred).await;
        user_new.banned = false;

        let mut user_new_document = to_document(&user_new)?;
        let now = DateTime::now();
        user_new_document.insert("created_at", now);
        user_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.insert_one(user_new_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");

        self::user_by_email(db.clone(), &user_new.email).await
    }
}

pub async fn user_sign_in(
    db: Database,
    signature: &str,
    password: &str,
) -> GqlResult<SignInfo> {
    let signature = &signature.to_lowercase();

    let user_res;
    let is_email = Regex::new(r"(@)")?.is_match(signature);
    if is_email {
        user_res = self::user_by_email(db.clone(), signature).await;
    } else {
        user_res = self::user_by_username(db.clone(), signature).await;
    }

    if let Ok(user) = user_res {
        let is_verified =
            cred_verify(&user.username, password, &user.cred).await;
        if is_verified {
            let mut header = Header::default();
            // header.kid = Some("signing_key".to_owned());
            header.alg = Algorithm::HS512;

            let site_key = CFG.get("SITE_KEY").unwrap().as_bytes();
            let claim_exp =
                CFG.get("CLAIM_EXP").unwrap().parse::<usize>().unwrap();
            let claims = Claims {
                email: user.email.to_owned(),
                username: user.username.to_owned(),
                exp: claim_exp,
            };

            let token = match encode(
                &header,
                &claims,
                &EncodingKey::from_secret(site_key),
            ) {
                Ok(t) => t,
                Err(error) => Err(Error::new("Error to encode token")
                    .extend_with(|_, e| {
                        e.set("details", format!("{}", error))
                    }))?,
            };

            let sign_info = SignInfo {
                email: user.email,
                username: user.username,
                token: token,
            };
            Ok(sign_info)
        } else {
            Err(Error::new("Invalid credential").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("User not exist")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

pub async fn users(db: Database, token: &str) -> GqlResult<Vec<User>> {
    let token_data = token_data(token).await;
    if token_data.is_ok() {
        let coll = db.collection::<Document>("users");

        let mut users: Vec<User> = vec![];

        // Query all documents in the collection.
        let mut cursor = coll.find(None, None).await.unwrap();
        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let user = from_document(document)?;
                    users.push(user);
                }
                Err(error) => Err(Error::new("Error to find doc")
                    .extend_with(|_, e| {
                        e.set(
                            "details",
                            format!("Error to find doc: {}", error),
                        )
                    }))?,
            }
        }

        Ok(users)
    } else {
        Err(Error::new("No token")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
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
        let user_res = self::user_by_email(db.clone(), &email).await;
        if let Ok(mut user) = user_res {
            if cred_verify(&user.username, pwd_cur, &user.cred).await {
                user.cred = cred_encode(&user.username, pwd_new).await;

                let coll = db.collection::<Document>("users");
                coll.update_one(
                    doc! {"_id": &user._id},
                    doc! {"$set": {"cred": &user.cred}},
                    None,
                )
                .await
                .expect("Failed to update a MongoDB collection!");

                Ok(user)
            } else {
                Err(Error::new("user_change_password").extend_with(|_, e| {
                    e.set("details", "Error verifying current passwordd")
                }))
            }
        } else {
            Err(Error::new("User not exist").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("No token")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// update user profile
pub async fn user_update_profile(
    db: Database,
    user_new: UserNew,
    token: &str,
) -> GqlResult<User> {
    let token_data = token_data(token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = self::user_by_email(db.clone(), &email).await;
        if let Ok(mut user) = user_res {
            let coll = db.collection::<Document>("users");

            user.email = user_new.email.to_lowercase();
            user.username = user_new.username.to_lowercase();

            let user_document = to_document(&user)?;
            coll.find_one_and_replace(
                doc! {"_id": &user._id},
                user_document,
                None,
            )
            .await
            .expect("Failed to replace a MongoDB collection!");

            Ok(user)
        } else {
            Err(Error::new("User not exist").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("No token")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// Create new wish
pub async fn wish_new(db: Database, wish_new: WishNew) -> GqlResult<Wish> {
    let coll = db.collection::<Document>("wishes");

    let exist_document = coll
        .find_one(
            doc! {"user_id": &wish_new.user_id, "aphorism": &wish_new.aphorism},
            None,
        )
        .await?;
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let mut wish_new_document = to_document(&wish_new)?;
        let now = DateTime::now();
        wish_new_document.insert("created_at", now);
        wish_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.insert_one(wish_new_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let wish_document = coll
        .find_one(
            doc! {"user_id": &wish_new.user_id, "aphorism": &wish_new.aphorism},
            None,
        )
        .await
        .expect("Document not found")
        .unwrap();

    let wish: Wish = from_document(wish_document)?;
    Ok(wish)
}

// get all wishes
pub async fn wishes(db: Database, published: i32) -> GqlResult<Vec<Wish>> {
    let mut find_doc = doc! {};
    if published > 0 {
        find_doc.insert("published", true);
    } else if published < 0 {
        find_doc.insert("published", false);
    }
    let coll = db.collection::<Document>("wishes");
    let mut cursor = coll.find(find_doc, None).await?;

    let mut wishes: Vec<Wish> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let wish = from_document(document)?;
                wishes.push(wish);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(wishes)
}

// get random wish
pub async fn random_wish(db: Database, username: &str) -> GqlResult<Wish> {
    let mut find_doc = doc! {"published": true};
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        let user = self::user_by_username(db.clone(), username).await?;
        find_doc.insert("user_id", &user._id);
    }
    let match_doc = doc! {"$match": find_doc};

    let one_wish = self::one_wish(db.clone(), match_doc).await;
    if one_wish.is_ok() {
        one_wish
    } else {
        self::one_wish(db, doc! {"$match": {"published": true}}).await
    }
}

async fn one_wish(db: Database, match_doc: Document) -> GqlResult<Wish> {
    let coll = db.collection::<Document>("wishes");
    let mut cursor = coll
        .aggregate(vec![doc! {"$sample": {"size": 1}}, match_doc], None)
        .await?;

    if let Some(result) = cursor.next().await {
        let wish = from_document(result?)?;
        Ok(wish)
    } else {
        Err(Error::new("No records")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}
