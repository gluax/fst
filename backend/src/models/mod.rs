use crate::utils::Validate;

use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection, Database,
};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// use crate::{errors::Result, utils::ErrorIf};

mod auth;
pub use auth::*;

mod user;
pub use user::*;

pub static MONGODB: OnceCell<Database> = OnceCell::new();

// TODO: @gluax should be based on env var as well
const DATABASE_NAME: &str = "db";

#[inline]
fn get_mongodb() -> &'static Database {
    unsafe { MONGODB.get_unchecked() }
}

pub async fn init_db(mongodb_uri: &str) {
    // TODO handle this expect call
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("failed to connect");
    let db = client.database(DATABASE_NAME);

    MONGODB.set(db).expect("TODO");
}
