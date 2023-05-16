mod claims;
pub use claims::*;

mod login;
pub use login::*;

mod register;
pub use register::*;

use super::*;

use salvo::prelude::{Extractible, StatusError};

fn validate_non_empty_field(name: &str, value: &str) -> salvo::Result<()> {
    if value.is_empty() {
        Err(StatusError::bad_request()
            .summary("Invalid Payload")
            .detail(format!("{name} cannot be empty")))?;
    }

    Ok(())
}
