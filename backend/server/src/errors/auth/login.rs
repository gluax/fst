use super::*;

// TODO fix repetitiveness: proc macro
#[derive(Debug, Default, Serialize, PartialEq, Eq)]
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

// TODO proc macro can derive this as well
impl ErrorIf for Login {
    fn error_or_ok(self) -> Result<()> {
        // TODO how to shorthand this logic
        if self == Self::default() {
            tracing::info!("Oked");
            Ok(())
        } else {
            tracing::info!("errored");
            Err(Error::Auth(Auth::Login(self)))
        }
    }
}

// TODO proc macro can derive this as well.
impl Piece for Login {
    fn render(self, res: &mut Response) {
        res.set_status_error(StatusError::not_acceptable());
        res.render(Json(self));
    }
}
