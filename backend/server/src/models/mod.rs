use mongodb::bson::oid::ObjectId;
use salvo::Request;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    errors::Result,
    utils::{ErrorIf, Validate},
};

mod auth;
pub use auth::*;
mod user;
pub use user::*;
