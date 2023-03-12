use super::*;

// TODO fix repetitiveness
// TODO how to set default strings?
#[derive(Debug, Default, Serialize)]
pub struct Login {
    #[serde(rename = "deserialize_error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deserialize: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "username_error")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "password_error")]
    pub password: Option<String>,
}

impl ErrorIf for Login {
    fn error_or_ok(self) -> Result<()> {
        // TODO how to shorthand this logic
        if self.username.is_none() && self.password.is_none() && self.deserialize.is_none() {
            tracing::info!("Oked");
            Ok(())
        } else {
            tracing::info!("errored");
            Err(Error::Auth(Auth::Login(self)))
        }
    }
}

#[salvo::async_trait]
impl Writer for Login {
    async fn write(mut self, _: &mut Request, _: &mut Depot, res: &mut Response) {
        tracing::info!("Login");
        res.set_status_error(StatusError::not_acceptable());
        res.render(Json(self));
    }
}
