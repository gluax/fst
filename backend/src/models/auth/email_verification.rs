use super::*;

#[derive(Debug, Deserialize, Extractible)]
#[extract(default_source(from = "body", format = "json"))]
pub struct EmailVerificationTokenMessage {
    pub email: String,
}

#[autometrics::autometrics]
impl Validate for EmailVerificationTokenMessage {
    fn validate(&self) -> salvo::Result<()> {
        validate_non_empty_field("email", &self.email)?;
        Ok(())
    }
}
