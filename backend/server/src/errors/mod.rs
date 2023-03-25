mod auth;
pub use auth::*;

use crate::utils::ErrorIf;
use fst_macros::{api_error, ErrorIf};
use salvo::{http::StatusError, writer::Json, Piece, Response};
use serde::Serialize;

mod response;
use response::AppResponse;

#[derive(Debug)]
pub enum Error {
    Auth(auth::Auth),
}

impl Piece for Error {
    fn render(self, res: &mut Response) {
        match self {
            Self::Auth(a) => a.render(res),
        };
    }
}

pub type Result<T> = core::result::Result<T, Error>;
