use super::*;

#[derive(Debug, Default, Serialize)]
pub struct Register {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password_confirmation: Option<String>,
}

impl Piece for Register {
    fn render(self, res: &mut Response) {
        tracing::info!("Register");
        res.set_status_error(StatusError::not_acceptable());
        res.render(Json(self));
    }
}
