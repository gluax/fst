use super::*;

#[derive(Debug, Deserialize, Extractible)]
#[extract(default_source(from = "body", format = "json"))]
pub struct EmailVerificationTokenMessage {
    pub email: String,
}
