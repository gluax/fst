use salvo::prelude::StatusCode;

use super::*;

// TODO fix repetitiveness: proc macro
#[api_error(error = not_acceptable)]
#[derive(Debug, Default, Serialize, PartialEq, Eq, ErrorIf)]
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

// TODO proc macro can derive this as well.
impl Piece for Login {
    fn render(self, res: &mut Response) {
        res.set_status_error(StatusError::not_acceptable());
        let resp = Json(AppResponse::<(), _>::from_error(
            StatusCode::NOT_ACCEPTABLE.as_u16(),
            self,
        ));
        res.render(resp);
    }
}
