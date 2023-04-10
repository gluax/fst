mod auth;
pub use auth::*;
mod health_check;
pub use health_check::*;

use crate::{errors::Result, models::*, utils::Validate};
use salvo::{handler, http::StatusCode, Response};
