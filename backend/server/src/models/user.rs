use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use mongodb::{bson::doc, Collection};
use salvo::prelude::StatusError;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: Option<ObjectId>,
    username: String,
    email: String,
    password: String,
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

    #[inline]
    fn foo(data: Register<'_>) -> User {
        User::from(data)
    }

    #[inline]
    async fn bar(&self, user: User) {
        self.collection.insert_one(user, None).await.expect("todo");
    }

    pub async fn create(&self, data: Register<'_>) -> salvo::Result<()> {
        let user = Self::foo(data);
        // TODO fix repetitiveness
        if self.find_by_email(&user.email).await?.is_some() {
            Err(StatusError::conflict()
                .with_summary("Email already in use")
                .with_detail(format!("The email `{}` is taken", user.email)))?;
        }

        if self.find_by_username(&user.username).await?.is_some() {
            Err(StatusError::conflict()
                .with_summary("Username taken")
                .with_detail(format!("The username `{}` is taken", user.username)))?;
        }

        // self.collection.insert_one(user, None).await.expect("todo");
        self.bar(user).await;
        Ok(())
    }
}

#[autometrics::autometrics]
impl From<Register<'_>> for User {
    fn from(value: Register) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2
            .hash_password(value.password.as_ref().as_ref(), &salt)
            .expect("TODO")
            .to_string();

        Self {
            id: None,
            username: value.username.to_string(),
            email: value.email.to_string(),
            password,
        }
    }
}
