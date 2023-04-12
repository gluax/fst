mod auth;
pub use auth::*;
mod health_check;
pub use health_check::*;

use crate::{models::*, utils::Validate};
use salvo::{handler, prelude::StatusCode, Response};

#[handler]
pub fn metrics(res: &mut Response) {
    match autometrics::encode_global_metrics() {
        Ok(m) => {
            res.set_status_code(StatusCode::OK);
            res.render(m);
        }
        Err(_) => {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
