use salvo::prelude::{Extractible, StatusError};

use super::*;

// TODO: genericize empty string checks with proc macro?

fn validate_username(username: &str) -> salvo::Result<()> {
    if username.is_empty() {
        Err(StatusError::bad_request()
            .with_summary("Invalid Payload")
            .with_detail("Username cannot be empty"))?;
    }

    Ok(())
}

fn validate_password(password: &str) -> salvo::Result<()> {
    if password.is_empty() {
        Err(StatusError::bad_request()
            .with_summary("Invalid Payload")
            .with_detail("Password cannot be empty"))?;
    }

    Ok(())
}

#[derive(Debug, Deserialize, Extractible, Validate)]
#[extract(default_source(from = "body", format = "json"))]
// TODO: consider the following
// #[validate(with = "validate_login")]
pub struct Login<'a> {
    #[validate(with = "validate_username")]
    // TODO: something like this maybe?
    // #[validate(non_empty_string)]
    #[serde(borrow)]
    username: Cow<'a, str>,
    #[validate(with = "validate_password")]
    #[serde(borrow)]
    password: Cow<'a, str>,
}
