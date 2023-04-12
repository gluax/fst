use mongodb::{bson::oid::ObjectId, Client, Database};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// use crate::{errors::Result, utils::ErrorIf};

mod auth;
pub use auth::*;
mod user;
pub use user::*;

pub static MONGODB: OnceCell<Database> = OnceCell::new();

const DATABASE_NAME: &str = "db";

#[inline]
fn get_mongodb() -> &'static Database {
    unsafe { MONGODB.get_unchecked() }
}

pub async fn init_db() {
    // TODO config file
    // TODO move to models? utils?
    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".into());
    // TODO handle this expect call
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("failed to connect");
    let db = client.database(DATABASE_NAME);

    MONGODB.set(db).expect("TODO");
}
