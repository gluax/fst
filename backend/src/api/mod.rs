mod auth;
pub use auth::*;
mod health_check;
pub use health_check::*;

use crate::{models::*, utils::Validate};
use salvo::{handler, prelude::StatusCode, Depot, Response};

#[handler]
pub fn metrics(res: &mut Response) {
    match autometrics::encode_global_metrics() {
        Ok(m) => {
            res.status_code(StatusCode::OK);
            res.render(m);
        }
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
