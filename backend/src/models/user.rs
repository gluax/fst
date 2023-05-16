use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use mongodb::{bson::doc, Collection};
use salvo::prelude::StatusError;

use crate::{
    config::Config,
    utils::{emails::send_email, verification_codes::generate_verification_code},
};
use maud::{html, DOCTYPE};

use super::*;

// TODO mongo doesn't yet support zero-copy deserialization
// https://jira.mongodb.org/browse/RUST-1175
// #[derive(Debug, Deserialize, Serialize)]
// pub struct User<'a> {
//     #[serde(rename = "_id")]
//     id: Option<ObjectId>,
//     #[serde(borrow)]
//     username: &'a str,
//     #[serde(borrow)]
//     email: &'a str,
//     #[serde(borrow)]
//     password: &'a str,
// }

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: ObjectId,
    username: String,
    email: String,
    password: String,
    joined: DateTime<Utc>,
    two_fa_enabled: bool,
    email_verified: bool,
}

#[autometrics::autometrics]
impl User {
    pub fn is_verified(&self) -> bool {
        self.email_verified
    }

    // TODO @app owner add your own fancy html email
    fn verify_html_template(code: &str) -> String {
        html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="UTF-8";
                    meta http-equiv="Content-Type" content="text/html charset=UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    title { "AppName: Email Verification Required!" }
                }
                body {
                    table style="display: flex; flex-direction: column; align-items: center;" {
                        h2 style="font-family: Arial, Helvetica, sans-serif;" { "Verification Code" }
                        h4 style="font-family: Arial, Helvetica, sans-serif;" { "Do not share this code with others" }
                        p style="font-family: Arial, Helvetica, sans-serif;" { "Confirmation code: " (code) }
                    }
                }
            }
        }.into_string()
    }

    pub async fn send_verification_email(&self, config: &Config) -> salvo::Result<()> {
        let verification_code = generate_verification_code(8);
        send_email(
            &config.email_app_credentials,
            &config.verification_email_from,
            format!("{} <{}>", self.username, self.email),
            Self::verify_html_template(&verification_code),
        )
        .await
        .expect("TODO");

        Ok(())
    }
}

#[derive(Debug)]
pub struct UserCollection {
    collection: Collection<User>,
}

#[autometrics::autometrics]
impl UserCollection {
    pub fn new() -> Self {
        Self {
            collection: get_mongodb().collection("users"),
        }
    }

    pub async fn find_by_username(&self, username: &str) -> salvo::Result<Option<User>> {
        let filter = doc! { "username": username };
        let user = self.collection.find_one(filter, None).await.expect("todo");
        Ok(user)
    }

    // TODO fix repetitiveness
    pub async fn find_by_email(&self, email: &str) -> salvo::Result<Option<User>> {
        let filter = doc! { "email": email };
        let user = self.collection.find_one(filter, None).await.expect("todo");
        Ok(user)
    }

    pub async fn create(&self, data: Register<'_>) -> salvo::Result<User> {
        let user = User::try_from(data)?;

        // TODO fix repetitiveness
        if self.find_by_email(&user.email).await?.is_some() {
            Err(StatusError::conflict()
                .summary("Email already in use")
                .detail(format!("The email `{}` is taken", user.email)))?;
        }

        if self.find_by_username(&user.username).await?.is_some() {
            Err(StatusError::conflict()
                .summary("Username taken")
                .detail(format!("The username `{}` is taken", user.username)))?;
        }

        self.collection.insert_one(&user, None).await.expect("todo");
        Ok(user)
    }
}

#[autometrics::autometrics]
impl TryFrom<Register<'_>> for User {
    type Error = salvo::Error;
    fn try_from(value: Register) -> salvo::Result<Self> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2
            .hash_password(value.password.as_ref().as_ref(), &salt)
            .expect("TODO")
            .to_string();

        Ok(Self {
            id: ObjectId::new(),
            username: value.username.to_string(),
            email: value.email.to_string(),
            password,
            joined: Utc::now(),
            ..Default::default()
        })
    }
}
