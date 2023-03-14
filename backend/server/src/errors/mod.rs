mod auth;
pub use auth::*;

use crate::utils::ErrorIf;
use salvo::{http::StatusError, writer::Json, Piece, Response};
use serde::Serialize;

#[derive(Debug)]
pub enum Error {
    Auth(auth::Auth),
}

impl Piece for Error {
    fn render(self, res: &mut Response) {
        tracing::info!("Error");
        match self {
            Self::Auth(a) => a.render(res),
        };
    }
}

pub type Result<T> = core::result::Result<T, Error>;
