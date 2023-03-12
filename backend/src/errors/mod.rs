mod auth;
pub use auth::*;

use crate::utils::ErrorIf;
use salvo::{http::StatusError, writer::Json, Depot, Request, Response, Writer};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Error {
    Auth(auth::Auth),
}

#[salvo::async_trait]
impl Writer for Error {
    async fn write(mut self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        tracing::info!("Error");
        match self {
            Self::Auth(a) => a.write(req, depot, res).await,
        };
    }
}

pub type Result<T> = core::result::Result<T, Error>;
