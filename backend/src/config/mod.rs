use std::env;

use lettre::{message::Mailbox, transport::smtp::authentication::Credentials};

#[derive(Debug, Clone)]
pub struct Config {
    pub email_app_credentials: Credentials,
    pub verification_email_from: Mailbox,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        let email_app_password = env::var("EMAIL_APP_PASSWORD")?;
        let email_app_username = env::var("EMAIL_APP_USERNAME")?;
        let email_app_credentials = Credentials::new(email_app_username, email_app_password);

        let verification_email_from = env::var("VERIFICATION_EMAIL_FROM")?.parse().expect("TODO");

        Ok(Self {
            email_app_credentials,
            verification_email_from,
        })
    }
}
