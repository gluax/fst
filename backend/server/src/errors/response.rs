use salvo::Piece;
use serde::Serialize;

fn is_successful_code(code: &u16) -> bool {
    match salvo::http::StatusCode::from_u16(*code) {
        Ok(sc) => sc.is_success(),
        // Shouldn't ever be reachable in case.
        Err(_) => unreachable!("Should always return a valid HTTP status code."),
    }
}

#[derive(Debug, Serialize)]
pub struct AppResponse<R, E> {
    #[serde(skip_serializing_if = "is_successful_code")]
    pub code: u16,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<R>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<E>,
}

impl<R, E> AppResponse<R, E> {
    pub fn from_error(code: u16, details: E) -> Self {
        Self {
            code,
            response: None,
            details: Some(details),
        }
    }

    pub fn from_response(code: u16, response: R) -> Self {
        Self {
            code,
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
