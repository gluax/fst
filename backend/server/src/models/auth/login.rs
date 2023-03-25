use super::*;

#[derive(Debug, Deserialize)]
pub struct Login<'a> {
    #[serde(borrow)]
    username: Cow<'a, str>,
    #[serde(borrow)]
    password: Cow<'a, str>,
}

// TODO Proc macro derive Validate trait.
#[salvo::async_trait]
impl<'a> Validate for Login<'a> {
    type Error = crate::errors::Login;

    async fn validate(req: &mut Request) -> Result<()> {
        let mut error = Self::Error::default();
        let data = match req.parse_json::<Login<'_>>().await {
            Err(e) => {
                error.invalid_data = Some(e.to_string());
                return error.error_or_ok();
            }
            Ok(data) => data,
        };

        if data.username.is_empty() {
            error.username = Some("Cannot be empty.".to_string());
        }

        if data.password.is_empty() {
            error.password = Some("Cannot be empty.".to_string());
        }

        error.error_or_ok()
    }
}
