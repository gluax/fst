use crate::utils::Validate;

use super::*;

#[derive(Debug, Deserialize, Extractible)]
#[extract(default_source(from = "body", format = "json"))]
// TODO: consider the following
// #[validate(with = "validate_login")]
pub struct Login<'a> {
    // TODO: something like this maybe?
    // #[validate(non_empty_string)]
    #[serde(borrow)]
    // #[validate(with = "validate_username")]
    username: Cow<'a, str>,
    #[serde(borrow)]
    // #[validate(with = "validate_password")]
    password: Cow<'a, str>,
}

#[autometrics::autometrics]
impl<'a> Validate for Login<'a> {
    fn validate(&self) -> salvo::Result<()> {
        validate_non_empty_field("username", &self.username)?;
        validate_non_empty_field("password", &self.password)
    }
}
