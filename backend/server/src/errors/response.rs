use salvo::Piece;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppResponse<R, E> {
    pub status_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<R>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<E>,
}

impl<R, E> AppResponse<R, E> {
    pub fn from_error(status_code: u16, details: E) -> Self {
        Self {
            status_code,
            response: None,
            details: Some(details),
        }
    }

    pub fn from_response(status_code: u16, response: R) -> Self {
        Self {
            status_code,
            response: Some(response),
            details: None,
        }
    }
}

impl<R, E> Piece for AppResponse<R, E>
where
    R: Serialize + Send,
    E: Serialize + Send,
{
    fn render(self, res: &mut salvo::Response) {
        res.render(salvo::writer::Json(self))
    }
}
