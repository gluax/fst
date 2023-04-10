use crate::errors::Result;

pub trait ErrorIf: Default {
    fn error_or_ok(self) -> Result<()>;
}

pub trait Validate {
    type Error: ErrorIf;

    fn validate(self) -> Result<()>;
}

pub trait RoutingGroup {
    // fn group(self) ->
}
