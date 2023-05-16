use std::env;

use lettre::{message::Mailbox, transport::smtp::authentication::Credentials};

// TODO: @gluax separate out by access.
// i.e. only routes that need to send emails should have email credentials affixes?
// is that overkill? ...it's definitely a pain :/. but is prob good practice
#[derive(Debug, Clone)]
pub struct Config {
    pub email_app_credentials: Credentials,
    pub mongodb_uri: String,
    pub verification_email_from: Mailbox,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        let email_app_password = env::var("EMAIL_APP_PASSWORD")?;
        let email_app_username = env::var("EMAIL_APP_USERNAME")?;
        let email_app_credentials = Credentials::new(email_app_username, email_app_password);

        let mongodb_uri = env::var("MONGODB_URI")?;

        let verification_email_from = env::var("VERIFICATION_EMAIL_FROM")?.parse().expect("TODO");

        Ok(Self {
            email_app_credentials,
            mongodb_uri,
            verification_email_from,
        })
    }
}
