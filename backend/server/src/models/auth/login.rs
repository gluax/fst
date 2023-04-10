use fst_macros::Validate;
use salvo::prelude::Extractible;

use super::*;

fn validate_username(username: &str) -> Option<String> {
    if username.is_empty() {
        return Some("Cannot be empty.".to_string());
    }

    None
}

fn validate_password(password: &str) -> Option<String> {
    if password.is_empty() {
        return Some("Cannot be empty.".to_string());
    }

    None
}

#[derive(Debug, Deserialize, Extractible, Validate)]
#[extract(default_source(from = "body", format = "json"))]
#[validate(error = "crate::errors::Login")]
pub struct Login<'a> {
    #[validate(with = "validate_username")]
    #[serde(borrow)]
    username: Cow<'a, str>,
    #[validate(with = "validate_password")]
    #[serde(borrow)]
    password: Cow<'a, str>,
}
