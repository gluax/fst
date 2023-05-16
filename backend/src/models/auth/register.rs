use regex::Regex;

use crate::utils::Validate;

use super::*;

#[derive(Debug, Deserialize, Extractible)]
#[extract(default_source(from = "body", format = "json"))]
// TODO: it should be a top level validate fn so a user can check pw fields are equal
pub struct Register<'a> {
    #[serde(borrow)]
    // #[validate(with = "validate_username")]
    pub username: Cow<'a, str>,
    #[serde(borrow)]
    // #[validate(with = "validate_email")]
    pub email: Cow<'a, str>,
    #[serde(borrow)]
    // #[validate(with = "validate_password")]
    pub password: Cow<'a, str>,
    #[serde(borrow)]
    // #[validate(with = "validate_password_confirmation")]
    password_confirmation: Cow<'a, str>,
}

#[autometrics::autometrics]
impl<'a> Validate for Register<'a> {
    fn validate(&self) -> salvo::Result<()> {
        validate_non_empty_field("username", &self.username)?;
        validate_non_empty_field("email", &self.email)?;

        let re = Regex::new(r#"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"#).expect("TODO");

        if !re.is_match(&self.email) {
            Err(StatusError::bad_request()
                .summary("Bad Email")
                .detail("Not a valid email."))?;
        }

        validate_non_empty_field("password", &self.password)?;
        validate_non_empty_field("password_confirmation", &self.password_confirmation)?;

        if self.password != self.password_confirmation {
            Err(StatusError::bad_request()
                .summary("Bad Password")
                .detail("Password fields don't match"))?;
        }

        if self.password.len() < 12 {
            Err(StatusError::bad_request()
                .summary("Bad Password")
                .detail("Password too short"))?;
        }

        Ok(())
    }
}
