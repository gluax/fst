mod auth;
pub use auth::*;

use fst_macros::{ApiError, ErrorIf};
use salvo::{Piece, Response};
use serde::Serialize;

mod response;
pub use response::AppResponse;

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
